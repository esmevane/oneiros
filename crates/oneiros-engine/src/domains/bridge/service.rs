use crate::*;

/// Server-side handler for incoming sync requests. Validates tickets
/// against the system DB and serves chronicle nodes for the Merkle
/// diff protocol.
#[derive(Clone)]
pub struct SyncHandler {
    config: Config,
    canons: CanonIndex,
}

impl core::fmt::Debug for SyncHandler {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SyncHandler").finish()
    }
}

impl SyncHandler {
    pub fn new(config: Config, canons: CanonIndex) -> Self {
        Self { config, canons }
    }

    async fn validate_ticket(&self, link: &Link) -> Result<Ticket, BridgeError> {
        let system = SystemContext::new(self.config.clone());
        let ticket = TicketRepo::new(&system)
            .get_by_token(link.token.as_str())
            .await?
            .ok_or_else(|| BridgeError::Denied("ticket not found".into()))?;

        if link.target != ticket.link.target {
            return Err(BridgeError::Denied(
                "link target does not match ticket target".into(),
            ));
        }

        Ok(ticket)
    }

    fn events_db(&self, brain: &BrainName) -> Result<rusqlite::Connection, BridgeError> {
        let mut config = self.config.clone();
        config.brain = brain.clone();
        let path = config.events_db_path();
        let conn = rusqlite::Connection::open(path)?;
        conn.pragma_update(None, "journal_mode", "wal")?;
        Ok(conn)
    }

    async fn handle_diff(&self, diff: &BridgeDiff) -> Result<BridgeResponse, BridgeError> {
        let ticket = self.validate_ticket(&diff.link).await?;
        let chronicle = self.canons.chronicle(&ticket.brain_name)?;
        let server_root = chronicle.root()?;

        // Roots match — no diff needed.
        if diff.root_hash == server_root {
            return Ok(BridgeResponse::BridgeCurrent);
        }

        // Server has no events — nothing to send.
        let Some(root_hash) = server_root else {
            return Ok(BridgeResponse::BridgeCurrent);
        };

        // Chronicle objects live in the system DB.
        let system_db = self.config.system_db()?;
        let store = ChronicleStore::new(&system_db);
        let resolve = store.resolver();

        let node = resolve(&root_hash).ok_or_else(|| {
            BridgeError::Protocol("chronicle root node not found in store".into())
        })?;

        Ok(BridgeResponse::BridgeRootNode(BridgeRootNode {
            root_hash,
            node,
        }))
    }

    async fn handle_resolve(
        &self,
        resolve_req: &BridgeResolve,
    ) -> Result<BridgeResponse, BridgeError> {
        let _ticket = self.validate_ticket(&resolve_req.link).await?;

        // Chronicle objects live in the system DB.
        let system_db = self.config.system_db()?;
        let store = ChronicleStore::new(&system_db);
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
        let ticket = self.validate_ticket(&fetch.link).await?;

        // Event log lives in events.db (standalone, no ATTACH).
        let db = self.events_db(&ticket.brain_name)?;

        let ids: Vec<EventId> = fetch
            .event_ids
            .iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;

        let events = EventLog::new(&db).get_batch(&ids)?;

        Ok(BridgeResponse::BridgeEvents(BridgeEvents { events }))
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
