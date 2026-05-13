use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub(crate) enum AuthError {
    #[error("Missing authorization header")]
    NoAuthHeader,
    #[error("Invalid or expired token")]
    InvalidToken,
    #[error(transparent)]
    Database(#[from] rusqlite::Error),
    #[error(transparent)]
    Event(#[from] EventError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match &self {
            AuthError::NoAuthHeader | AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::Database(_) | AuthError::Event(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, axum::Json(ErrorResponse::new(self.to_string()))).into_response()
    }
}

/// The outcome of token verification — what kind of session was granted.
#[derive(Clone, Debug)]
pub(crate) enum VerifiedSession {
    /// Authenticated with a host-level token. Grants access to host-scoped
    /// routes (dashboard, host management, project CRUD).
    Host,
    /// Authenticated with a project-scoped project token. Grants access to
    /// project-scoped routes and (by implication) host-scoped routes.
    Project {
        /// The project this token grants access to — resolved from the
        /// ticket. Callers derive the full config from `ServerState`.
        project_name: ProjectName,
    },
}

/// Unified token verifier for both host and project tokens.
///
/// Host tokens are statelessly verified via HMAC against the host secret
/// key. Project tokens are verified via DB lookup against the ticket store.
pub(crate) struct TicketVerifier {
    config: Config,
    host_secret: iroh::SecretKey,
}

impl TicketVerifier {
    pub(crate) fn new(config: Config, _canons: CanonIndex, host_secret: iroh::SecretKey) -> Self {
        Self {
            config,
            host_secret,
        }
    }

    /// Verify a bearer token. Tries the host token path first (purely
    /// local, no DB), then falls back to the project token path (requires
    /// ticket lookup).
    pub(crate) async fn verify(&self, token_str: &str) -> Result<VerifiedSession, AuthError> {
        // 1. Try host token (fast path — no DB or async work)
        let candidate = HostToken::from(token_str.to_string());
        if candidate.verify(&self.host_secret) {
            return Ok(VerifiedSession::Host);
        }

        // 2. Try project token (requires async DB lookup)
        self.verify_project_token(token_str).await
    }

    /// Validate a project-scoped project token against the ticket store.
    async fn verify_project_token(&self, token_str: &str) -> Result<VerifiedSession, AuthError> {
        let token = Token::from(token_str)
            .decode()
            .map_err(|_| AuthError::InvalidToken)?;

        let host_scope = ComposeScope::new(self.config.clone())
            .host()
            .map_err(|_| AuthError::InvalidToken)?;

        let ticket = TicketRepo::new(&host_scope)
            .get_by_token(token_str)
            .await
            .map_err(|_| AuthError::InvalidToken)?
            .ok_or(AuthError::InvalidToken)?;

        if ticket.actor_id != token.actor_id || ticket.project_id != token.project_id {
            return Err(AuthError::InvalidToken);
        }

        ticket
            .check_validity()
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(VerifiedSession::Project {
            project_name: ticket.project_name,
        })
    }
}

/// Paths excluded from auth middleware — these are public.
const EXCLUDED_PATHS: &[&str] = &["/", "/health", "/docs", "/api.json"];

/// Paths that match by prefix — e.g., `/docs/` has sub-resources
/// that should also be exempt, and `/mcp` handles its own auth in
/// the MCP `initialize` method.
const EXCLUDED_PREFIXES: &[&str] = &["/docs/", "/mcp"];

/// Axum middleware that enforces Bearer token authentication on every
/// request except the public paths listed above.
///
/// Extracts the `Authorization: Bearer <token>` header, verifies it via
/// [`TicketVerifier`], and injects the resulting [`VerifiedSession`] into
/// request extensions for downstream extractors.
pub(crate) async fn auth_middleware(
    State(state): State<ServerState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let path = req.uri().path();

    if EXCLUDED_PATHS.contains(&path) || EXCLUDED_PREFIXES.iter().any(|p| path.starts_with(p)) {
        return Ok(next.run(req).await);
    }

    let token_str = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AuthError::NoAuthHeader)?;

    let verifier = state.ticket_verifier();
    let session = verifier.verify(token_str).await?;

    req.extensions_mut().insert(session);
    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(dir: &std::path::Path) -> Config {
        Config::builder()
            .data_dir(dir.to_path_buf())
            .project(ProjectName::new("test"))
            .build()
    }

    #[tokio::test]
    async fn verifier_accepts_valid_host_token() {
        let dir = tempfile::TempDir::new().unwrap();
        let secret = iroh::SecretKey::generate();
        let config = test_config(dir.path());
        let canons = CanonIndex::new();
        let verifier = TicketVerifier::new(config, canons, secret.clone());

        let host_token = HostToken::generate(&secret);
        let result = verifier.verify(&host_token.to_string()).await;

        assert!(
            matches!(result, Ok(VerifiedSession::Host)),
            "valid host token should be accepted, got {result:?}"
        );
    }

    #[tokio::test]
    async fn verifier_rejects_invalid_host_token() {
        let dir = tempfile::TempDir::new().unwrap();
        let secret = iroh::SecretKey::generate();
        let other_secret = iroh::SecretKey::generate();
        let config = test_config(dir.path());
        let canons = CanonIndex::new();
        let verifier = TicketVerifier::new(config, canons, secret);

        let wrong_token = HostToken::generate(&other_secret);
        let result = verifier.verify(&wrong_token.to_string()).await;

        assert!(
            matches!(result, Err(AuthError::InvalidToken)),
            "token from different host key should be rejected, got {result:?}"
        );
    }

    #[tokio::test]
    async fn verifier_rejects_garbage_token() {
        let dir = tempfile::TempDir::new().unwrap();
        let secret = iroh::SecretKey::generate();
        let config = test_config(dir.path());
        let canons = CanonIndex::new();
        let verifier = TicketVerifier::new(config, canons, secret);

        let result = verifier.verify("not-a-valid-token-at-all").await;

        assert!(
            matches!(result, Err(AuthError::InvalidToken)),
            "garbage token should be rejected, got {result:?}"
        );
    }

    // Project token tests require a running host with tickets.
    // Those are exercised via integration tests in tests/acceptance/.
}
