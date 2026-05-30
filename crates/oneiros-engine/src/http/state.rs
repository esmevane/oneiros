use std::sync::{Arc, OnceLock};

use aide::openapi::OpenApi;
use axum::{extract::FromRequestParts, http::request::Parts};

use crate::*;

/// Shared state for the HTTP server.
///
/// Carries the host context (always available) and resolves project
/// context per-request via Bearer token.
#[derive(Clone)]
pub(crate) struct ServerState {
    config: Config,
    canons: CanonIndex,
    bridge: Bridge,
    api: Arc<OnceLock<OpenApi>>,
    mailbox: Mailbox,
    host_secret: iroh::SecretKey,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ServerStateError {
    #[error("failed to read or generate host secret key: {0}")]
    HostKey(#[from] HostKeyError),
    #[error("failed to bind iroh bridge: {0}")]
    Bridge(#[from] BridgeError),
}

impl ServerState {
    /// Construct a server state with a bound iroh bridge. Loads (or
    /// generates) the host secret key from disk, binds a `Bridge`, and
    /// spawns the host actor that consumes the bus.
    pub(crate) async fn bind(config: Config) -> Result<Self, ServerStateError> {
        let secret = HostKey::new(config.platform()).ensure()?;
        let bridge = Bridge::bind(secret.clone()).await?;

        let canons = CanonIndex::new();
        let mailbox = Mailbox::spawn(canons.clone());

        Ok(Self {
            config,
            canons,
            bridge,
            api: Arc::new(OnceLock::new()),
            mailbox,
            host_secret: secret,
        })
    }

    /// The bus mailbox — services dispatch events through this handle.
    pub(crate) fn mailbox(&self) -> &Mailbox {
        &self.mailbox
    }

    /// Install the OpenAPI spec. Called once after the router is assembled.
    pub(crate) fn set_api(&self, api: OpenApi) {
        let _ = self.api.set(api);
    }

    /// The installed OpenAPI spec, if set.
    pub(crate) fn api(&self) -> Option<&OpenApi> {
        self.api.get()
    }

    /// The bound bridge.
    pub(crate) fn bridge(&self) -> &Bridge {
        &self.bridge
    }

    /// The host's identity (key + address).
    pub(crate) fn host_identity(&self) -> HostIdentity {
        self.bridge.host_identity()
    }

    /// The bookmark registry — shared state for all projects.
    pub(crate) fn canons(&self) -> &CanonIndex {
        &self.canons
    }

    /// Hydrate reducer pipelines and chronicles from event logs.
    /// Best-effort — skips databases that don't exist yet (pre-init).
    pub(crate) fn hydrate(&self) {
        let _ = self
            .canons
            .hydrate_project(&self.config, &self.config.project);
    }

    /// The server configuration.
    pub(crate) fn config(&self) -> &Config {
        &self.config
    }

    /// The project name from the server config.
    pub(crate) fn project_name(&self) -> &ProjectName {
        &self.config.project
    }

    /// Build a project context for a request. Strangler — used by the
    /// `ProjectLog` extractor for legacy CLI/MCP dispatchers.
    #[expect(deprecated)]
    pub(crate) fn project_log(&self, config: Config) -> ProjectLog {
        ProjectLog::new(config, self.canons.clone())
    }

    /// Construct a ticket verifier backed by this server's config,
    /// canon registry, and host secret key.
    pub(crate) fn ticket_verifier(&self) -> TicketVerifier {
        TicketVerifier::new(
            self.config.clone(),
            self.canons.clone(),
            self.host_secret.clone(),
        )
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
        // The auth middleware already injected VerifiedSession into
        // extensions. For AtHost, we accept both host and project
        // sessions — all we need is a valid config to compose from.
        Ok(ComposeScope::new(state.config.clone()).host()?)
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub(crate) struct ScopeExtractError(#[from] ComposeError);

impl axum::response::IntoResponse for ScopeExtractError {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            self.to_string(),
        )
            .into_response()
    }
}

/// Resolve the bookmark for a request from the `X-Bookmark` header or
/// `?bookmark=` query parameter. Falls back to the active bookmark for
/// the given project, or `main` if none is active.
fn resolve_bookmark(
    parts: &Parts,
    state: &ServerState,
    project_name: &ProjectName,
) -> BookmarkName {
    let from_header_or_query = parts
        .headers
        .get("x-bookmark")
        .and_then(|v| v.to_str().ok())
        .or_else(|| {
            parts
                .uri
                .query()
                .and_then(|q| q.split('&').find_map(|pair| pair.strip_prefix("bookmark=")))
        });

    if let Some(bookmark) = from_header_or_query {
        return BookmarkName::new(bookmark);
    }

    state
        .canons()
        .active_bookmark(project_name)
        .unwrap_or_else(|_| BookmarkName::main())
}

impl aide::operation::OperationInput for Scope<AtBookmark> {}

impl FromRequestParts<ServerState> for Scope<AtBookmark> {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        // The auth middleware already validated the token and injected a
        // VerifiedSession into request extensions. Bookmark-scoped routes
        // require a project token (not just a host token).
        let session = parts
            .extensions
            .get::<VerifiedSession>()
            .ok_or(AuthError::NoAuthHeader)?;

        let project_name = match session {
            VerifiedSession::Host => return Err(AuthError::InvalidToken),
            VerifiedSession::Project(project_name) => project_name.clone(),
        };

        let bookmark = resolve_bookmark(parts, state, &project_name);
        let mut config = state.config().clone();
        config.project = project_name;
        config.bookmark = bookmark;

        let scope = ComposeScope::new(config.clone())
            .bookmark(config.project.clone(), config.bookmark.clone())?;
        Ok(scope)
    }
}

#[expect(deprecated)]
impl FromRequestParts<ServerState> for ProjectLog {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        // Read the verified session from extensions (set by auth middleware).
        // ProjectLog requires a project token — it carries the project-scoped
        // config that MCP dispatch and HTTP handlers use.
        let session = parts
            .extensions
            .get::<VerifiedSession>()
            .ok_or(AuthError::NoAuthHeader)?;

        let config = match session {
            VerifiedSession::Host => return Err(AuthError::InvalidToken),
            VerifiedSession::Project(project_name) => {
                let bookmark = resolve_bookmark(parts, state, project_name);
                let mut c = state.config().clone();
                c.project = project_name.clone();
                c.bookmark = bookmark;
                c
            }
        };

        Ok(state.project_log(config))
    }
}
