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
}

impl ServerState {
    pub fn new(config: Config) -> Self {
        let (broadcast, _) = broadcast::channel(256);
        Self { config, broadcast }
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

    /// Build a project context that shares this server's broadcast channel.
    pub fn project_context(&self, config: Config) -> ProjectContext {
        ProjectContext::with_broadcast(config, self.broadcast.clone())
    }
}

impl FromRequestParts<ServerState> for SystemContext {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        Ok(state.config.system())
    }
}

impl FromRequestParts<ServerState> for ProjectContext {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        // Extract Bearer token
        let token_str = parts
            .headers
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthError::NoAuthHeader)?
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidAuthHeader)?;

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

        Ok(state.project_context(config))
    }
}
