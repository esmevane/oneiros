use crate::*;

/// Server-side handler for incoming sync requests. Validates tickets
/// against the system DB and exports canon updates for the requested
/// brain's bookmark, using the CRDT delta mechanism.
#[derive(Clone)]
pub(crate) struct SyncHandler {
    config: Config,
    canons: CanonIndex,
}

impl core::fmt::Debug for SyncHandler {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SyncHandler").finish()
    }
}

impl SyncHandler {
    pub(crate) fn new(config: Config, canons: CanonIndex) -> Self {
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

    async fn handle_confer(
        &self,
        link: &Link,
        version_vector: &[u8],
    ) -> Result<BridgeResponse, BridgeError> {
        let ticket = self.validate_ticket(link).await?;

        let canon = self.canons.brain(&ticket.brain_name)?;

        let canon_bytes = canon.export_updates_since(version_vector)?;

        if canon_bytes.is_empty() {
            Ok(BridgeResponse::Current)
        } else {
            Ok(BridgeResponse::Updates { canon_bytes })
        }
    }

    async fn handle_fetch_events(
        &self,
        link: &Link,
        event_ids: &[String],
    ) -> Result<BridgeResponse, BridgeError> {
        let ticket = self.validate_ticket(link).await?;

        let mut brain_config = self.config.clone();
        brain_config.brain = ticket.brain_name.clone();
        let db = brain_config.brain_db()?;

        let ids: Vec<EventId> = event_ids
            .iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;

        let events = EventLog::new(&db).get_batch(&ids)?;

        Ok(BridgeResponse::Events { events })
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
            let response = BridgeResponse::Denied {
                reason: format!("request too large: {len} bytes"),
            };
            write_response(&mut send, &response).await.ok();
            connection.closed().await;
            return Ok(());
        }
        let mut buf = vec![0u8; len];
        recv.read_exact(&mut buf)
            .await
            .map_err(iroh::protocol::AcceptError::from_err)?;

        let response = match BridgeRequest::from_bytes(&buf) {
            Ok(BridgeRequest::Confer {
                link,
                version_vector,
            }) => self
                .handle_confer(&link, &version_vector)
                .await
                .unwrap_or_else(|e| BridgeResponse::Denied {
                    reason: e.to_string(),
                }),
            Ok(BridgeRequest::FetchEvents { link, event_ids }) => self
                .handle_fetch_events(&link, &event_ids)
                .await
                .unwrap_or_else(|e| BridgeResponse::Denied {
                    reason: e.to_string(),
                }),
            Err(e) => BridgeResponse::Denied {
                reason: format!("invalid request: {e}"),
            },
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
