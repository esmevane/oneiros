use std::sync::{Arc, OnceLock};

use crate::*;

/// The ALPN string advertised and required by the oneiros sync protocol.
/// Only clients explicitly negotiating this ALPN can reach the sync handler.
pub const SYNC_ALPN: &[u8] = b"/oneiros/sync/1";

/// Maximum message size on the sync wire, in bytes. Guards against
/// absurdly large payloads.
const MAX_MESSAGE_SIZE: usize = 64 * 1024 * 1024;

/// The bridge to other oneiros hosts — the runtime value that owns the bound
/// `iroh::Endpoint` and acts as our wrapper around the transport layer.
///
/// This is the second of the two files in the engine that imports `iroh::*`
/// (the other is `peer_address.rs`). All other code talks to `Bridge` rather
/// than reaching into iroh types.
///
/// A Bridge is bound once at service start using the host's persisted
/// keypair, lives on `ServerState`, and produces `Remote` handles on demand
/// when the system needs to talk to a peer.
#[derive(Clone)]
pub struct Bridge {
    endpoint: iroh::Endpoint,
    public_key: PeerKey,
    router: Arc<OnceLock<iroh::protocol::Router>>,
}

#[derive(Debug, thiserror::Error)]
pub enum BridgeError {
    #[error("failed to bind iroh endpoint: {0}")]
    Bind(String),
    #[error("transport error: {0}")]
    Transport(String),
    #[error("protocol error: {0}")]
    Protocol(String),
    #[error("sync denied: {0}")]
    Denied(String),
}

impl Bridge {
    /// Bind a Bridge using the given iroh secret key. Advertises the
    /// `/oneiros/sync/1` ALPN so peers can negotiate the sync protocol.
    ///
    /// This is async because iroh's endpoint construction is async — it
    /// has to set up relay connections and bind sockets.
    pub async fn bind(secret: iroh::SecretKey) -> Result<Self, BridgeError> {
        let public = secret.public();
        let endpoint = iroh::Endpoint::empty_builder()
            .secret_key(secret)
            .alpns(vec![SYNC_ALPN.to_vec()])
            .bind()
            .await
            .map_err(|e| BridgeError::Bind(e.to_string()))?;

        let public_key = PeerKey::from_bytes(*public.as_bytes());

        Ok(Self {
            endpoint,
            public_key,
            router: Arc::new(OnceLock::new()),
        })
    }

    /// Register the sync protocol handler on this bridge's endpoint.
    /// Idempotent: calling it multiple times has no effect after the
    /// first. The handler runs as a detached tokio task for the
    /// lifetime of the endpoint, serving incoming `SyncRequest::Pull`
    /// calls against the given config's databases.
    pub fn serve(&self, config: Config) {
        if self.router.get().is_some() {
            return;
        }
        let handler = SyncHandler::new(config);
        let router = iroh::protocol::Router::builder(self.endpoint.clone())
            .accept(SYNC_ALPN, handler)
            .spawn();
        let _ = self.router.set(router);
    }

    /// Open a connection to a peer and perform a single sync request.
    /// Returns the events the peer sent in response, or an error if the
    /// request was denied or the transport failed.
    pub async fn request_events(
        &self,
        address: &PeerAddress,
        request: &SyncRequest,
    ) -> Result<Vec<StoredEvent>, BridgeError> {
        let conn = self
            .endpoint
            .connect(address.inner().clone(), SYNC_ALPN)
            .await
            .map_err(|e| BridgeError::Transport(e.to_string()))?;

        let (mut send, mut recv) = conn
            .open_bi()
            .await
            .map_err(|e| BridgeError::Transport(e.to_string()))?;

        // Write length-prefixed request.
        let encoded = request.to_bytes();
        let len = (encoded.len() as u32).to_be_bytes();
        send.write_all(&len)
            .await
            .map_err(|e| BridgeError::Transport(e.to_string()))?;
        send.write_all(&encoded)
            .await
            .map_err(|e| BridgeError::Transport(e.to_string()))?;
        send.finish()
            .map_err(|e| BridgeError::Transport(e.to_string()))?;

        // Read length-prefixed response.
        let mut len_buf = [0u8; 4];
        recv.read_exact(&mut len_buf)
            .await
            .map_err(|e| BridgeError::Transport(e.to_string()))?;
        let response_len = u32::from_be_bytes(len_buf) as usize;
        if response_len > MAX_MESSAGE_SIZE {
            return Err(BridgeError::Protocol(format!(
                "response too large: {response_len} bytes"
            )));
        }
        let mut buf = vec![0u8; response_len];
        recv.read_exact(&mut buf)
            .await
            .map_err(|e| BridgeError::Transport(e.to_string()))?;

        // Close the connection gracefully so the server's handler returns.
        conn.close(0u32.into(), b"done");

        let response =
            SyncResponse::from_bytes(&buf).map_err(|e| BridgeError::Protocol(e.to_string()))?;

        match response {
            SyncResponse::Events { events } => Ok(events),
            SyncResponse::Denied { reason } => Err(BridgeError::Denied(reason)),
        }
    }

    /// The host's public key — stable across restarts if the caller binds
    /// with the same secret key.
    pub fn key(&self) -> PeerKey {
        self.public_key
    }

    /// The host's current reachability information. Includes relay URLs and
    /// direct socket addresses as discovered by iroh.
    pub fn address(&self) -> PeerAddress {
        let mut addr = iroh::EndpointAddr::new(self.endpoint.id());
        for socket in self.endpoint.bound_sockets() {
            addr = addr.with_ip_addr(socket);
        }
        PeerAddress::new(addr)
    }

    /// Compose the host's current identity (key + address) for use when
    /// minting `oneiros://` URIs.
    pub fn host_identity(&self) -> HostIdentity {
        HostIdentity::new(self.key(), self.address())
    }

    /// Shut down the bridge gracefully, closing the iroh endpoint.
    pub async fn shutdown(&self) {
        self.endpoint.close().await;
    }
}

impl core::fmt::Debug for Bridge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Bridge")
            .field("public_key", &self.public_key.to_string())
            .finish()
    }
}

/// Server-side handler for incoming sync requests. Wraps a `Config` for
/// database access; each call opens the relevant databases fresh rather
/// than holding connections. Validates tickets against the system DB and
/// reads events from the brain DB referenced by the ticket.
#[derive(Debug, Clone)]
pub struct SyncHandler {
    config: Config,
}

impl SyncHandler {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Handle a parsed Pull request: validate the ticket, open the
    /// brain DB, read events, and produce a response.
    async fn handle_pull(
        &self,
        link: &Link,
        _checkpoint: &Checkpoint,
    ) -> Result<SyncResponse, String> {
        // Validate the ticket by looking up the token in the system DB.
        // For MVP we only check existence; expiry/revocation/uses
        // enforcement lands when TicketUsed/TicketRejected events are
        // actually emitted in a follow-up slice.
        let system = SystemContext::new(self.config.clone());
        let ticket = TicketRepo::new(&system)
            .get_by_token(link.token.as_str())
            .await
            .map_err(|e| format!("ticket lookup failed: {e}"))?
            .ok_or_else(|| "ticket not found".to_string())?;

        // Verify the link's target is a bookmark and that the ticket's
        // link matches (defense against confused-deputy style attacks).
        if link.target != ticket.link.target {
            return Err("link target does not match ticket target".into());
        }

        // Open the ticket's brain DB and read all events. Incremental
        // collection (since checkpoint) is a follow-up — MVP returns
        // the full log and lets the caller dedupe on event id.
        let mut brain_config = self.config.clone();
        brain_config.brain = ticket.brain_name.clone();
        let db = brain_config
            .brain_db()
            .map_err(|e| format!("brain db open failed: {e}"))?;
        let events = EventLog::new(&db)
            .load_all()
            .map_err(|e| format!("event log load failed: {e}"))?;

        Ok(SyncResponse::Events { events })
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
            .map_err(|e| iroh::protocol::AcceptError::from_err(e))?;

        // Read length-prefixed request.
        let mut len_buf = [0u8; 4];
        recv.read_exact(&mut len_buf)
            .await
            .map_err(|e| iroh::protocol::AcceptError::from_err(e))?;
        let len = u32::from_be_bytes(len_buf) as usize;
        if len > MAX_MESSAGE_SIZE {
            let response = SyncResponse::Denied {
                reason: format!("request too large: {len} bytes"),
            };
            write_response(&mut send, &response).await.ok();
            connection.closed().await;
            return Ok(());
        }
        let mut buf = vec![0u8; len];
        recv.read_exact(&mut buf)
            .await
            .map_err(|e| iroh::protocol::AcceptError::from_err(e))?;

        let response = match SyncRequest::from_bytes(&buf) {
            Ok(SyncRequest::Pull { link, checkpoint }) => self
                .handle_pull(&link, &checkpoint)
                .await
                .unwrap_or_else(|reason| SyncResponse::Denied { reason }),
            Err(e) => SyncResponse::Denied {
                reason: format!("invalid request: {e}"),
            },
        };

        if let Err(e) = write_response(&mut send, &response).await {
            return Err(iroh::protocol::AcceptError::from_err(
                std::io::Error::other(e.to_string()),
            ));
        }

        // Wait for the client to close so our response is guaranteed
        // delivered before the connection drops.
        connection.closed().await;

        Ok(())
    }
}

async fn write_response(
    send: &mut iroh::endpoint::SendStream,
    response: &SyncResponse,
) -> Result<(), std::io::Error> {
    let encoded = response.to_bytes();
    let len = (encoded.len() as u32).to_be_bytes();
    send.write_all(&len).await?;
    send.write_all(&encoded).await?;
    send.finish()
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn bridge_binds_with_fresh_key() {
        let secret = iroh::SecretKey::generate(&mut rand::rng());
        let expected_key = PeerKey::from_bytes(*secret.public().as_bytes());

        let bridge = Bridge::bind(secret).await.unwrap();
        assert_eq!(bridge.key(), expected_key);

        bridge.shutdown().await;
    }

    #[tokio::test]
    async fn bridge_exposes_host_identity() {
        let secret = iroh::SecretKey::generate(&mut rand::rng());
        let bridge = Bridge::bind(secret).await.unwrap();

        let identity = bridge.host_identity();
        assert_eq!(identity.key, bridge.key());

        bridge.shutdown().await;
    }

    #[tokio::test]
    async fn two_bridges_have_distinct_keys() {
        let a = Bridge::bind(iroh::SecretKey::generate(&mut rand::rng()))
            .await
            .unwrap();
        let b = Bridge::bind(iroh::SecretKey::generate(&mut rand::rng()))
            .await
            .unwrap();

        assert_ne!(a.key(), b.key());

        a.shutdown().await;
        b.shutdown().await;
    }
}
