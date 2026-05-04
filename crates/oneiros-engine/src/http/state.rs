use std::sync::{Arc, OnceLock};

use aide::openapi::OpenApi;
use axum::{extract::FromRequestParts, http::request::Parts};

use crate::*;

/// Shared state for the HTTP server.
///
/// Carries the system context (always available) and resolves brain
/// context per-request via Bearer token.
#[derive(Clone)]
pub struct ServerState {
    config: Config,
    canons: CanonIndex,
    bridge: Bridge,
    api: Arc<OnceLock<OpenApi>>,
}

#[derive(Debug, thiserror::Error)]
pub enum ServerStateError {
    #[error("failed to read or generate host secret key: {0}")]
    HostKey(#[from] HostKeyError),
    #[error("failed to bind iroh bridge: {0}")]
    Bridge(#[from] BridgeError),
}

impl ServerState {
    /// Construct a server state with a bound iroh bridge. Loads (or
    /// generates) the host secret key from disk and binds a `Bridge`.
    pub async fn bind(config: Config) -> Result<Self, ServerStateError> {
        let secret = HostKey::new(&config.data_dir).ensure()?;
        let bridge = Bridge::bind(secret).await?;

        let canons = CanonIndex::new();
        Ok(Self {
            config,
            canons,
            bridge,
            api: Arc::new(OnceLock::new()),
        })
    }

    /// Install the OpenAPI spec. Called once after the router is assembled.
    pub fn set_api(&self, api: OpenApi) {
        let _ = self.api.set(api);
    }

    /// The installed OpenAPI spec, if set.
    pub fn api(&self) -> Option<&OpenApi> {
        self.api.get()
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

    /// Build a project context with pre-hydrated pipeline.
    ///
    /// Resolves the active bookmark for the brain unless the config
    /// already has an explicit bookmark override.
    pub fn project_log(&self, config: Config) -> Result<ProjectLog, EventError> {
        let entry = self.canons.brain_entry(&config.brain)?;
        Ok(ProjectLog::with_entry(config, entry))
    }

    /// Build a system context.
    pub fn host_log(&self) -> HostLog {
        HostLog::new(self.config.clone())
    }
}

impl FromRequestParts<ServerState> for HostLog {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        Ok(state.host_log())
    }
}

impl FromRequestParts<ServerState> for ProjectLog {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        let token_str = parts
            .headers
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .ok_or(AuthError::NoAuthHeader)?;

        // Decode claims from the self-describing token
        let token = Token::from(token_str)
            .decode()
            .map_err(|_| AuthError::InvalidToken)?;

        // Revocation check — verify the ticket still exists in the DB
        let scope = ComposeScope::new(state.config.clone())
            .host()
            .map_err(|_| AuthError::InvalidToken)?;
        let ticket = TicketRepo::new(&scope)
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

        // Assemble ProjectLog for this request
        let mut config = state.config.clone();
        config.brain = ticket.brain_name.clone();

        // Default to the active bookmark for this brain.
        config.bookmark = state
            .canons()
            .active_bookmark(&ticket.brain_name)
            .unwrap_or_else(|_| BookmarkName::main());

        // Override with explicit X-Bookmark header or ?bookmark= query param.
        if let Some(bookmark) = parts
            .headers
            .get("x-bookmark")
            .and_then(|v| v.to_str().ok())
            .or_else(|| {
                parts
                    .uri
                    .query()
                    .and_then(|q| q.split('&').find_map(|pair| pair.strip_prefix("bookmark=")))
            })
        {
            config.bookmark = BookmarkName::new(bookmark);
        }

        state
            .project_log(config)
            .map_err(|_| AuthError::InvalidToken)
    }
}
