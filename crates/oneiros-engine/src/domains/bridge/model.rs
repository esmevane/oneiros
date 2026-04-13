use std::sync::{Arc, OnceLock};

use crate::*;

/// The ALPN string advertised and required by the oneiros sync protocol.
/// Only clients explicitly negotiating this ALPN can reach the sync handler.
pub(crate) const SYNC_ALPN: &[u8] = b"/oneiros/sync/1";

/// Maximum message size on the sync wire, in bytes. Guards against
/// absurdly large payloads.
pub(crate) const MAX_MESSAGE_SIZE: usize = 64 * 1024 * 1024;

/// The bridge to other oneiros hosts — the runtime value that owns the bound
/// `iroh::Endpoint` and acts as our wrapper around the transport layer.
///
/// A Bridge is bound once at service start using the host's persisted
/// keypair, lives on `ServerState`, and produces connections on demand
/// when the system needs to talk to a peer.
#[derive(Clone)]
pub(crate) struct Bridge {
    endpoint: iroh::Endpoint,
    public_key: PeerKey,
    router: Arc<OnceLock<iroh::protocol::Router>>,
}

impl Bridge {
    /// Bind a Bridge using the given iroh secret key. Advertises the
    /// `/oneiros/sync/1` ALPN so peers can negotiate the sync protocol.
    pub(crate) async fn bind(secret: iroh::SecretKey) -> Result<Self, BridgeError> {
        let public = secret.public();
        let endpoint = iroh::Endpoint::empty_builder()
            .secret_key(secret)
            .alpns(vec![SYNC_ALPN.to_vec()])
            .bind()
            .await
            .map_err(|e| BridgeError::from(IrohError::from(e)))?;

        let public_key = PeerKey::from_bytes(*public.as_bytes());

        Ok(Self {
            endpoint,
            public_key,
            router: Arc::new(OnceLock::new()),
        })
    }

    /// Register the sync protocol handler on this bridge's endpoint.
    /// Idempotent: calling it multiple times has no effect after the first.
    pub(crate) fn serve(&self, config: Config, canons: CanonIndex) {
        if self.router.get().is_some() {
            return;
        }
        let handler = SyncHandler::new(config, canons);
        let router = iroh::protocol::Router::builder(self.endpoint.clone())
            .accept(SYNC_ALPN, handler)
            .spawn();
        let _ = self.router.set(router);
    }

    /// Open a connection to a peer and confer — exchange version vectors,
    /// receive canon updates. Returns the raw Loro update bytes for the
    /// caller to import into their canon, or `None` if already current.
    pub(crate) async fn confer(
        &self,
        address: &PeerAddress,
        request: &BridgeRequest,
    ) -> Result<Option<Vec<u8>>, BridgeError> {
        let response = self.send(address, request).await?;

        match response {
            BridgeResponse::Updates { canon_bytes } => Ok(Some(canon_bytes)),
            BridgeResponse::Current => Ok(None),
            BridgeResponse::Events { .. } => Err(BridgeError::Protocol(
                "expected Updates or Current response for Confer request".into(),
            )),
            BridgeResponse::Denied { reason } => Err(BridgeError::Denied(reason)),
        }
    }

    /// Fetch specific events by ID from a peer. Issued after a conference
    /// when the local side has determined which events it needs.
    pub(crate) async fn fetch_events(
        &self,
        address: &PeerAddress,
        request: &BridgeRequest,
    ) -> Result<Vec<StoredEvent>, BridgeError> {
        let response = self.send(address, request).await?;

        match response {
            BridgeResponse::Events { events } => Ok(events),
            BridgeResponse::Current => Ok(Vec::new()),
            BridgeResponse::Updates { .. } => Err(BridgeError::Protocol(
                "expected Events response for FetchEvents request".into(),
            )),
            BridgeResponse::Denied { reason } => Err(BridgeError::Denied(reason)),
        }
    }

    /// The host's public key — stable across restarts if the caller binds
    /// with the same secret key.
    pub(crate) fn key(&self) -> PeerKey {
        self.public_key
    }

    /// The host's current reachability information.
    pub(crate) fn address(&self) -> PeerAddress {
        let mut addr = iroh::EndpointAddr::new(self.endpoint.id());
        for socket in self.endpoint.bound_sockets() {
            addr = addr.with_ip_addr(socket);
        }
        PeerAddress::new(addr)
    }

    /// Compose the host's current identity (key + address).
    pub(crate) fn host_identity(&self) -> HostIdentity {
        HostIdentity::new(self.key(), self.address())
    }

    /// Shut down the bridge gracefully, closing the iroh endpoint.
    pub(crate) async fn shutdown(&self) {
        self.endpoint.close().await;
    }

    /// Send a sync request to a peer and read the response.
    async fn send(
        &self,
        address: &PeerAddress,
        request: &BridgeRequest,
    ) -> Result<BridgeResponse, BridgeError> {
        let (response_bytes, conn) = self.exchange(address, request).await?;
        conn.close(0u32.into(), b"done");

        if response_bytes.len() > MAX_MESSAGE_SIZE {
            return Err(BridgeError::Protocol(format!(
                "response too large: {} bytes",
                response_bytes.len()
            )));
        }

        BridgeResponse::from_bytes(&response_bytes)
            .map_err(|e| BridgeError::Protocol(e.to_string()))
    }

    /// Low-level transport: connect, write request, read response bytes.
    /// Returns the raw response bytes and the connection (for caller to close).
    async fn exchange(
        &self,
        address: &PeerAddress,
        request: &BridgeRequest,
    ) -> Result<(Vec<u8>, iroh::endpoint::Connection), IrohError> {
        let conn = self
            .endpoint
            .connect(address.inner().clone(), SYNC_ALPN)
            .await?;

        let (mut send, mut recv) = conn.open_bi().await?;

        // Write length-prefixed request.
        let encoded = request.to_bytes();
        let len = (encoded.len() as u32).to_be_bytes();
        send.write_all(&len).await?;
        send.write_all(&encoded).await?;
        send.finish()?;

        // Read length-prefixed response.
        let mut len_buf = [0u8; 4];
        recv.read_exact(&mut len_buf).await?;
        let response_len = u32::from_be_bytes(len_buf) as usize;

        let mut buf = vec![0u8; response_len];
        recv.read_exact(&mut buf).await?;

        Ok((buf, conn))
    }
}

impl core::fmt::Debug for Bridge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Bridge")
            .field("public_key", &self.public_key.to_string())
            .finish()
    }
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
