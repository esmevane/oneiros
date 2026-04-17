use axum::{extract::FromRequestParts, http::request::Parts};
use tokio::sync::broadcast;

use crate::*;

/// Shared state for the HTTP server.
///
/// Carries the system context (always available), a shared broadcast
/// channel for SSE subscribers, and resolves brain context per-request
/// via Bearer token.
#[derive(Clone)]
pub struct ServerState {
    config: Config,
    broadcast: broadcast::Sender<StoredEvent>,
    canons: CanonIndex,
    bridge: Bridge,
}

#[derive(Debug, thiserror::Error)]
pub enum ServerStateError {
    #[error("failed to read or generate host secret key: {0}")]
    HostKey(#[from] std::io::Error),
    #[error("failed to bind iroh bridge: {0}")]
    Bridge(#[from] BridgeError),
}

impl ServerState {
    /// Construct a server state with a bound iroh bridge. Loads (or
    /// generates) the host secret key from disk and binds a `Bridge`.
    pub async fn bind(config: Config) -> Result<Self, ServerStateError> {
        let secret = config.ensure_host_secret_key()?;
        let bridge = Bridge::bind(secret).await?;

        let (broadcast, _) = broadcast::channel(256);
        let canons = CanonIndex::new();
        Ok(Self {
            config,
            broadcast,
            canons,
            bridge,
        })
    }

    /// The bound bridge.
    pub fn bridge(&self) -> &Bridge {
        &self.bridge
    }

    /// The host's identity (key + address).
    pub fn host_identity(&self) -> HostIdentity {
        self.bridge.host_identity()
    }

    /// The bookmark registry — shared state for all brains.
    pub fn canons(&self) -> &CanonIndex {
        &self.canons
    }

    /// Hydrate reducer pipelines and chronicles from event logs.
    /// Best-effort — skips databases that don't exist yet (pre-init).
    pub fn hydrate(&self) {
        let _ = self.canons.hydrate_brain(&self.config, &self.config.brain);
    }

    /// The server configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// The token for the configured brain, if one exists.
    pub fn token(&self) -> Option<Token> {
        self.config.token()
    }

    /// The brain name from the server config.
    pub fn brain_name(&self) -> &BrainName {
        &self.config.brain
    }

    /// The shared broadcast sender for SSE event streaming.
    pub fn broadcast(&self) -> &broadcast::Sender<StoredEvent> {
        &self.broadcast
    }

    /// Build a project context with shared broadcast, canon, and pipeline.
    pub fn project_context(&self, config: Config) -> Result<ProjectContext, EventError> {
        let entry = self.canons.brain_entry(&config.brain)?;
        Ok(ProjectContext::with_entry(
            config,
            self.broadcast.clone(),
            entry,
        ))
    }

    /// Build a system context.
    pub fn system_context(&self) -> SystemContext {
        SystemContext::new(self.config.clone())
    }
}

impl FromRequestParts<ServerState> for SystemContext {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        Ok(state.system_context())
    }
}

impl FromRequestParts<ServerState> for ProjectContext {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        // Extract Bearer token from header, falling back to ?token= query
        // param. The query fallback exists because browser EventSource
        // (SSE) cannot set custom headers.
        let token_str = parts
            .headers
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .or_else(|| {
                parts
                    .uri
                    .query()
                    .and_then(|q| q.split('&').find_map(|pair| pair.strip_prefix("token=")))
            })
            .ok_or(AuthError::NoAuthHeader)?;

        // Decode claims from the self-describing token
        let token = Token::from(token_str)
            .decode()
            .map_err(|_| AuthError::InvalidToken)?;

        // Revocation check — verify the ticket still exists in the DB
        let system = state.config.system();
        let ticket = TicketRepo::new(&system)
            .get_by_token(token_str)
            .await
            .map_err(|_| AuthError::InvalidToken)?
            .ok_or(AuthError::InvalidToken)?;

        match (
            ticket.actor_id == token.actor_id,
            ticket.brain_id == token.brain_id,
            true, // ticket.tenant_id == token.tenant_id,
        ) {
            (true, true, true) => {}
            _ => return Err(AuthError::InvalidToken),
        }

        // Assemble ProjectContext with shared broadcast channel
        let mut config = state.config.clone();
        config.brain = ticket.brain_name;

        state
            .project_context(config)
            .map_err(|_| AuthError::InvalidToken)
    }
}
