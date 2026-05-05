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
    mailbox: Mailbox,
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
    /// generates) the host secret key from disk, binds a `Bridge`, and
    /// spawns the host actor that consumes the bus.
    pub async fn bind(config: Config) -> Result<Self, ServerStateError> {
        let secret = HostKey::new(&config.data_dir).ensure()?;
        let bridge = Bridge::bind(secret).await?;

        let canons = CanonIndex::new();
        let (mailbox, rx) = Mailbox::open();
        let _actor = HostActor::spawn(config.clone(), canons.clone(), rx);

        Ok(Self {
            config,
            canons,
            bridge,
            api: Arc::new(OnceLock::new()),
            mailbox,
        })
    }

    /// The bus mailbox — services dispatch events through this handle.
    pub fn mailbox(&self) -> &Mailbox {
        &self.mailbox
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

    /// Build a project context for a request. Strangler — used by the
    /// `ProjectLog` extractor for legacy CLI/MCP dispatchers.
    pub fn project_log(&self, config: Config) -> ProjectLog {
        ProjectLog::new(config)
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

impl aide::operation::OperationInput for Mailbox {}

impl FromRequestParts<ServerState> for Mailbox {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        Ok(state.mailbox().clone())
    }
}

impl aide::operation::OperationInput for Scope<AtHost> {}

impl FromRequestParts<ServerState> for Scope<AtHost> {
    type Rejection = ScopeExtractError;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        ComposeScope::new(state.config.clone())
            .host()
            .map_err(|e| ScopeExtractError::Other(e.to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ScopeExtractError {
    #[error("{0}")]
    Other(String),
}

impl axum::response::IntoResponse for ScopeExtractError {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            self.to_string(),
        )
            .into_response()
    }
}

/// Resolve the bookmark-tier capability for a request: validate the
/// Bearer token against the ticket store, pick the bookmark (active by
/// default; `X-Bookmark` header / `?bookmark=` query overrides), and
/// return both the request-shaped `Config` and the composed scope.
///
/// Standalone helper — not built on top of `ProjectLog`. Both the new
/// `Scope<AtBookmark>` extractor and the legacy `ProjectLog` extractor
/// share this resolution.
async fn resolve_request(
    parts: &Parts,
    state: &ServerState,
) -> Result<(Config, Scope<AtBookmark>), AuthError> {
    let token_str = parts
        .headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(AuthError::NoAuthHeader)?;

    let token = Token::from(token_str)
        .decode()
        .map_err(|_| AuthError::InvalidToken)?;

    let host_scope = ComposeScope::new(state.config.clone())
        .host()
        .map_err(|_| AuthError::InvalidToken)?;
    let ticket = TicketRepo::new(&host_scope)
        .get_by_token(token_str)
        .await
        .map_err(|_| AuthError::InvalidToken)?
        .ok_or(AuthError::InvalidToken)?;

    if ticket.actor_id != token.actor_id || ticket.brain_id != token.brain_id {
        return Err(AuthError::InvalidToken);
    }

    let mut config = state.config.clone();
    config.brain = ticket.brain_name.clone();
    config.bookmark = state
        .canons()
        .active_bookmark(&ticket.brain_name)
        .unwrap_or_else(|_| BookmarkName::main());

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

    let scope = ComposeScope::new(config.clone())
        .bookmark(config.brain.clone(), config.bookmark.clone())
        .map_err(|_| AuthError::InvalidToken)?;
    Ok((config, scope))
}

impl aide::operation::OperationInput for Scope<AtBookmark> {}

impl FromRequestParts<ServerState> for Scope<AtBookmark> {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        let (_config, scope) = resolve_request(parts, state).await?;
        Ok(scope)
    }
}

impl FromRequestParts<ServerState> for ProjectLog {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        let (config, _scope) = resolve_request(parts, state).await?;
        Ok(state.project_log(config))
    }
}
