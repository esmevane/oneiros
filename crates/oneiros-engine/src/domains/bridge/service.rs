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

    async fn handle_list_bookmarks(
        &self,
        request: &BridgeListBookmarks,
    ) -> Result<BridgeResponse, BridgeError> {
        let scope = ComposeScope::new(self.config.clone()).host()?;
        let ticket = self.validate_ticket(&scope, &request.ticket).await?;

        if !ticket.can(PermissionOp::BookmarkList) {
            return Err(DenyReason::InsufficientPermissions.into());
        }

        // The request must target the same project the ticket was issued for.
        if request.project != ticket.project_name {
            return Err(DenyReason::TargetMismatch.into());
        }

        // List bookmark names from the host DB projection.
        let host_db = self.config.host_db().map_err(|e| {
            BridgeError::Denied(DenyReason::Remote(OpaquePeer::from(e.to_string())))
        })?;
        let bookmarks = BookmarkStore::new(&host_db)
            .list_for_project(&request.project)
            .map_err(|e| {
                BridgeError::Denied(DenyReason::Remote(OpaquePeer::from(e.to_string())))
            })?;

        Ok(BridgeResponse::BridgeBookmarkList(BridgeBookmarkList {
            bookmarks,
        }))
    }

    async fn handle_pull_bookmark(
        &self,
        request: &BridgePullBookmark,
    ) -> Result<BridgeResponse, BridgeError> {
        let scope = ComposeScope::new(self.config.clone()).host()?;
        let ticket = self.validate_ticket(&scope, &request.ticket).await?;

        if !ticket.can(PermissionOp::BookmarkPull) {
            return Err(DenyReason::InsufficientPermissions.into());
        }

        let project_scope =
            ComposeScope::new(self.config.clone()).project(ticket.project_name.clone())?;
        let db = EventsDb::open(&project_scope).await?;

        // Get all event IDs for this bookmark from the chronicle.
        let chronicle = self
            .canons
            .bookmark_chronicle(&ticket.project_name, &request.bookmark_name)?;
        let root = chronicle.root()?;
        let resolve = {
            let config = self.config.clone();
            move |hash: &ContentHash| -> Option<LedgerNode> {
                let db = config.host_db().ok()?;
                ChronicleStore::new(&db).get(hash)
            }
        };
        let ids: Vec<String> = match root.as_ref() {
            Some(hash) => Ledger::collect_all_ids(Some(hash), &resolve)
                .into_iter()
                .map(|id| id.to_string())
                .collect(),
            None => vec![],
        };

        let event_ids: Vec<EventId> = ids
            .iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;
        let events = EventLog::new(&db).get_batch(&event_ids)?;

        Ok(BridgeResponse::BridgeEvents(BridgeEvents { events }))
    }

    async fn handle_push_bookmark(
        &self,
        request: &BridgePushBookmark,
    ) -> Result<BridgeResponse, BridgeError> {
        let scope = ComposeScope::new(self.config.clone()).host()?;
        let ticket = self.validate_ticket(&scope, &request.ticket).await?;

        if !ticket.can(PermissionOp::BookmarkPush) {
            return Err(DenyReason::InsufficientPermissions.into());
        }

        // Ensure the target bookmark exists on our side.
        if !self
            .canons
            .has_bookmark(&ticket.project_name, &request.bookmark_name)?
        {
            self.canons
                .fork_project(&ticket.project_name, &request.bookmark_name)?;
            BookmarkService::create_bookmark_db(
                &self.config,
                &ticket.project_name,
                &request.bookmark_name,
                &[],
            )
            .map_err(|e| {
                BridgeError::Denied(DenyReason::Remote(OpaquePeer::from(e.to_string())))
            })?;

            // Emit BookmarkForked so the host DB projection picks it up.
            let from = self.canons.active_bookmark(&ticket.project_name)?;
            let bookmark_id = BookmarkId::new();
            let new_event = NewEvent::builder()
                .data(Events::Bookmark(BookmarkEvents::BookmarkForked(
                    BookmarkForked::builder_v1()
                        .bookmark(
                            Bookmark::builder()
                                .id(bookmark_id)
                                .project(ticket.project_name.clone())
                                .name(request.bookmark_name.clone())
                                .build(),
                        )
                        .from(from)
                        .build()
                        .into(),
                )))
                .build();
            self.mailbox.tell(HostMessage::from(
                AppendHostLog::builder()
                    .scope(ComposeScope::new(self.config.clone()).host()?)
                    .event(new_event)
                    .build(),
            ));

            // Write directly to the bookmark store so compose can find it.
            let bookmark = Bookmark::builder()
                .id(bookmark_id)
                .project(ticket.project_name.clone())
                .name(request.bookmark_name.clone())
                .build();
            let host_db = self.config.host_db().map_err(|e| {
                BridgeError::Denied(DenyReason::Remote(OpaquePeer::from(e.to_string())))
            })?;
            host_db
                .execute(
                    "INSERT OR REPLACE INTO bookmarks (id, project, name, created_at) VALUES (?1, ?2, ?3, ?4)",
                    rusqlite::params![
                        bookmark.id.to_string(),
                        bookmark.project.to_string(),
                        bookmark.name.to_string(),
                        bookmark.created_at.to_string(),
                    ],
                )
                .map_err(|e| {
                    BridgeError::Denied(DenyReason::Remote(OpaquePeer::from(e.to_string())))
                })?;
        }

        // Pull the pusher's data via chronicle diff + fetch.
        BookmarkService::collect_from_peer_link(
            &ServerState::from_parts(
                self.config.clone(),
                self.canons.clone(),
                self.bridge.clone(),
                self.mailbox.clone(),
            ),
            &ticket.project_name,
            &request.bookmark_name,
            request.bookmark.clone(),
        )
        .await
        .map_err(|e| BridgeError::Denied(DenyReason::Remote(OpaquePeer::from(e.to_string()))))?;

        Ok(BridgeResponse::BridgePushAccepted)
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
            Ok(BridgeRequest::BridgeListBookmarks(list)) => {
                self.handle_list_bookmarks(&list).await.unwrap_or_else(|e| {
                    BridgeResponse::BridgeDenied(BridgeDenied {
                        reason: e.to_string(),
                    })
                })
            }
            Ok(BridgeRequest::BridgePullBookmark(pull)) => {
                self.handle_pull_bookmark(&pull).await.unwrap_or_else(|e| {
                    BridgeResponse::BridgeDenied(BridgeDenied {
                        reason: e.to_string(),
                    })
                })
            }
            Ok(BridgeRequest::BridgePushBookmark(push)) => {
                self.handle_push_bookmark(&push).await.unwrap_or_else(|e| {
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
