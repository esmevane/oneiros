use crate::*;

/// Server-side handler for incoming sync requests. Validates tickets
/// against the host DB and serves chronicle nodes for the Merkle
/// diff protocol.
#[derive(Clone)]
pub(crate) struct SyncHandler {
    config: Config,
    canons: CanonIndex,
    bridge: Bridge,
    mailbox: Mailbox,
}

impl core::fmt::Debug for SyncHandler {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SyncHandler").finish()
    }
}

impl SyncHandler {
    pub(crate) fn new(
        config: Config,
        canons: CanonIndex,
        bridge: Bridge,
        mailbox: Mailbox,
    ) -> Self {
        Self {
            config,
            canons,
            bridge,
            mailbox,
        }
    }

    async fn validate_ticket(
        &self,
        scope: &Scope<AtHost>,
        link: &Link,
    ) -> Result<Ticket, BridgeError> {
        let ticket = TicketRepo::new(scope)
            .get_by_token(link.token.as_str())
            .await?
            .ok_or(DenyReason::TicketNotFound)?;

        if link.target != ticket.link.target {
            return Err(DenyReason::TargetMismatch.into());
        }

        ticket.check_validity()?;

        // All existing sync operations (diff, resolve, fetch) require read access.
        if !ticket.can(PermissionOp::Read) {
            return Err(DenyReason::InsufficientPermissions.into());
        }

        Ok(ticket)
    }

    async fn handle_diff(&self, diff: &BridgeDiff) -> Result<BridgeResponse, BridgeError> {
        let scope = ComposeScope::new(self.config.clone()).host()?;
        let ticket = self.validate_ticket(&scope, &diff.link).await?;
        let chronicle = self.canons.chronicle(&ticket.project_name)?;
        let server_root = chronicle.root()?;

        // Roots match — no diff needed.
        if diff.root_hash == server_root {
            return Ok(BridgeResponse::BridgeCurrent);
        }

        // Server has no events — nothing to send.
        let Some(root_hash) = server_root else {
            return Ok(BridgeResponse::BridgeCurrent);
        };

        // Chronicle objects live in the host DB.
        let host_db = HostDb::open(&scope).await?;
        let store = ChronicleStore::new(&host_db);
        let resolve = store.resolver();

        let node = resolve(&root_hash).ok_or(BridgeProtocolError::ChronicleRootMissing)?;

        Ok(BridgeResponse::BridgeRootNode(BridgeRootNode {
            root_hash,
            node,
        }))
    }

    async fn handle_resolve(
        &self,
        resolve_req: &BridgeResolve,
    ) -> Result<BridgeResponse, BridgeError> {
        let scope = ComposeScope::new(self.config.clone()).host()?;
        let _ticket = self.validate_ticket(&scope, &resolve_req.link).await?;

        // Chronicle objects live in the host DB.
        let host_db = HostDb::open(&scope).await?;
        let store = ChronicleStore::new(&host_db);
        let resolve = store.resolver();

        let nodes: Vec<(ContentHash, LedgerNode)> = resolve_req
            .hashes
            .iter()
            .filter_map(|hash| resolve(hash).map(|node| (hash.clone(), node)))
            .collect();

        Ok(BridgeResponse::BridgeNodes(BridgeNodes { nodes }))
    }

    async fn handle_fetch_events(
        &self,
        fetch: &BridgeFetchEvents,
    ) -> Result<BridgeResponse, BridgeError> {
        let scope = ComposeScope::new(self.config.clone()).host()?;
        let ticket = self.validate_ticket(&scope, &fetch.link).await?;

        // Compose at the target project's project tier — events DB
        // lives there. ComposeScope verifies the project exists.
        let project_scope =
            ComposeScope::new(self.config.clone()).project(ticket.project_name.clone())?;

        // Event log lives in events.db (standalone, no ATTACH).
        let db = EventsDb::open(&project_scope).await?;

        let ids: Vec<EventId> = fetch
            .event_ids
            .iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;

        let events = EventLog::new(&db).get_batch(&ids)?;

        Ok(BridgeResponse::BridgeEvents(BridgeEvents { events }))
    }

    async fn handle_submit(&self, submit: &BridgeSubmit) -> Result<BridgeResponse, BridgeError> {
        let scope = ComposeScope::new(self.config.clone()).host()?;

        // Validate the submit ticket — must have Write permission.
        let submit_ticket = self.validate_ticket(&scope, &submit.ticket).await?;
        if !submit_ticket.can(PermissionOp::Write) {
            return Err(DenyReason::InsufficientPermissions.into());
        }

        // Validate the bookmark link (read access).
        self.validate_ticket(&scope, &submit.bookmark.link).await?;

        // Ensure the bookmark exists on the receiver side so the
        // chronicle is available for the diff + import.
        if !self
            .canons
            .has_bookmark(&submit_ticket.project_name, &submit.bookmark_name)?
        {
            self.canons
                .fork_project(&submit_ticket.project_name, &submit.bookmark_name)?;
            BookmarkService::create_bookmark_db(
                &self.config,
                &submit_ticket.project_name,
                &submit.bookmark_name,
                &[],
            )
            .map_err(|e| BridgeError::from(DenyReason::Remote(OpaquePeer::from(e.to_string()))))?;
        }

        // Pull the peer's bookmark data via the existing collect logic.
        BookmarkService::collect_from_peer_link(
            &ServerState::from_parts(
                self.config.clone(),
                self.canons.clone(),
                self.bridge.clone(),
                self.mailbox.clone(),
            ),
            &submit_ticket.project_name,
            &submit.bookmark_name,
            submit.bookmark.clone(),
        )
        .await
        .map_err(|e| BridgeError::from(DenyReason::Remote(OpaquePeer::from(e.to_string()))))?;

        Ok(BridgeResponse::BridgeSubmissionAccepted)
    }
}

impl iroh::protocol::ProtocolHandler for SyncHandler {
    async fn accept(
        &self,
        connection: iroh::endpoint::Connection,
    ) -> Result<(), iroh::protocol::AcceptError> {
        let (mut send, mut recv) = connection
            .accept_bi()
            .await
            .map_err(iroh::protocol::AcceptError::from_err)?;

        // Read length-prefixed request.
        let mut len_buf = [0u8; 4];
        recv.read_exact(&mut len_buf)
            .await
            .map_err(iroh::protocol::AcceptError::from_err)?;
        let len = u32::from_be_bytes(len_buf) as usize;
        if len > MAX_MESSAGE_SIZE {
            let response = BridgeResponse::BridgeDenied(BridgeDenied {
                reason: format!("request too large: {len} bytes"),
            });
            write_response(&mut send, &response).await.ok();
            connection.closed().await;
            return Ok(());
        }
        let mut buf = vec![0u8; len];
        recv.read_exact(&mut buf)
            .await
            .map_err(iroh::protocol::AcceptError::from_err)?;

        let response = match BridgeRequest::from_bytes(&buf) {
            Ok(BridgeRequest::BridgeDiff(diff)) => {
                self.handle_diff(&diff).await.unwrap_or_else(|e| {
                    BridgeResponse::BridgeDenied(BridgeDenied {
                        reason: e.to_string(),
                    })
                })
            }
            Ok(BridgeRequest::BridgeResolve(resolve)) => {
                self.handle_resolve(&resolve).await.unwrap_or_else(|e| {
                    BridgeResponse::BridgeDenied(BridgeDenied {
                        reason: e.to_string(),
                    })
                })
            }
            Ok(BridgeRequest::BridgeFetchEvents(fetch)) => {
                self.handle_fetch_events(&fetch).await.unwrap_or_else(|e| {
                    BridgeResponse::BridgeDenied(BridgeDenied {
                        reason: e.to_string(),
                    })
                })
            }
            Ok(BridgeRequest::BridgeSubmit(submit)) => {
                self.handle_submit(&submit).await.unwrap_or_else(|e| {
                    BridgeResponse::BridgeDenied(BridgeDenied {
                        reason: e.to_string(),
                    })
                })
            }
            Err(e) => BridgeResponse::BridgeDenied(BridgeDenied {
                reason: format!("invalid request: {e}"),
            }),
        };

        if let Err(e) = write_response(&mut send, &response).await {
            return Err(iroh::protocol::AcceptError::from_err(
                std::io::Error::other(e.to_string()),
            ));
        }

        connection.closed().await;

        Ok(())
    }
}

async fn write_response(
    send: &mut iroh::endpoint::SendStream,
    response: &BridgeResponse,
) -> Result<(), std::io::Error> {
    let encoded = response.to_bytes();
    let len = (encoded.len() as u32).to_be_bytes();
    send.write_all(&len).await?;
    send.write_all(&encoded).await?;
    send.finish()
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    Ok(())
}
