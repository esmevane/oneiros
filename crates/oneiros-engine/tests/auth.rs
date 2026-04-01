//! Transport-level auth tests.
//!
//! These test the HTTP layer directly — the acceptance harness goes through
//! the CLI, so it never exercises token checking on HTTP routes.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use oneiros_engine::*;

/// Boot a config with system + project initialized, and a ticket issued.
/// Returns the router, the token string, and the tempdir handle.
async fn setup() -> (axum::Router, String, tempfile::TempDir) {
    let dir = tempfile::tempdir().expect("create tempdir");
    let config = Config::builder()
        .data_dir(dir.path().to_path_buf())
        .brain(BrainName::new("test-brain"))
        .build();

    let system = config.system();

    // SystemService::init creates the data dir and migrates the system DB.
    SystemService::init(&system, &InitSystem::builder().name("test").build())
        .await
        .unwrap();

    // Create brain + ticket via ProjectService.
    let token = match ProjectService::init(
        &system,
        &InitProject::builder()
            .name(BrainName::new("test-brain"))
            .build(),
    )
    .await
    .unwrap()
    {
        ProjectResponse::Initialized(result) => result.token,
        other => panic!("expected Initialized, got {other:?}"),
    };

    let server = Server::new(config);
    let router = server.router();

    (router, token.to_string(), dir)
}

fn get(uri: &str) -> Request<Body> {
    Request::builder()
        .method("GET")
        .uri(uri)
        .body(Body::empty())
        .unwrap()
}

fn get_with_token(uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method("GET")
        .uri(uri)
        .header("Authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

#[tokio::test]
async fn project_routes_require_auth_token() {
    let (app, _token, _dir) = setup().await;

    // A request to a brain-scoped endpoint without a token should be rejected.
    let response = app.oneshot(get("/agents")).await.unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "project routes should require a Bearer token, but got {}",
        response.status()
    );
}

#[tokio::test]
async fn project_routes_reject_invalid_token() {
    let (app, _token, _dir) = setup().await;

    // A request with a garbage token should be rejected.
    let response = app
        .oneshot(get_with_token("/agents", "not-a-real-token"))
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "invalid tokens should be rejected, but got {}",
        response.status()
    );
}

#[tokio::test]
async fn project_routes_accept_valid_token() {
    let (app, token, _dir) = setup().await;

    // A request with a valid ticket token should succeed.
    let response = app
        .oneshot(get_with_token("/agents", &token))
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "valid token should grant access, but got {}",
        response.status()
    );
}

#[tokio::test]
async fn system_routes_do_not_require_auth() {
    let (app, _token, _dir) = setup().await;

    // System routes (tenants, actors, etc.) should be accessible without a token.
    // They are host-local administrative endpoints.
    let response = app.oneshot(get("/tenants")).await.unwrap();

    assert!(
        response.status() != StatusCode::UNAUTHORIZED,
        "system routes should not require auth, but got UNAUTHORIZED"
    );
}
