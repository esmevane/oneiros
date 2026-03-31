//! Transport-level storage tests.
//!
//! Storage keys can be file paths (e.g. "notes/design/architecture.md").
//! These tests verify that path-like keys survive the HTTP round-trip —
//! the CLI path handles them fine, but the HTTP transport must encode
//! them to avoid route collisions with the `/` path separator.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use oneiros_engine::*;

/// Boot a config with system + project + ticket, returning the router and token.
async fn setup() -> (axum::Router, String, tempfile::TempDir) {
    let dir = tempfile::tempdir().expect("create tempdir");
    let config = Config::builder()
        .data_dir(dir.path().to_path_buf())
        .brain(BrainName::new("test-brain"))
        .build();

    config.bootstrap().expect("bootstrap");

    let system = config.system();

    SystemService::init(&system, &InitSystem::builder().name("test").build())
        .await
        .unwrap();

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
    (server.router(), token.to_string(), dir)
}

#[tokio::test]
async fn storage_round_trips_path_like_key() {
    let (app, token, _dir) = setup().await;

    // Upload with a path-like key
    let upload_body = serde_json::json!({
        "key": "notes/design/architecture.md",
        "description": "Architecture notes",
        "data": [72, 101, 108, 108, 111]
    });

    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/storage")
                .header("content-type", "application/json")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::from(upload_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        resp.status(),
        StatusCode::CREATED,
        "upload with path-like key should succeed"
    );

    // Retrieve it by key — the client encodes the key as a StorageRef
    // so path separators don't collide with HTTP routing.
    let ref_key = StorageRef::encode(&StorageKey::new("notes/design/architecture.md"));

    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/storage/{ref_key}"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "GET with path-like key should find the uploaded artifact, but got {}",
        resp.status()
    );
}

#[tokio::test]
async fn storage_round_trips_absolute_path_key() {
    let (app, token, _dir) = setup().await;

    // Upload with an absolute-path key (leading slash)
    let upload_body = serde_json::json!({
        "key": "/usr/local/share/oneiros/config.toml",
        "description": "System config",
        "data": [72, 101, 108, 108, 111]
    });

    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/storage")
                .header("content-type", "application/json")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::from(upload_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        resp.status(),
        StatusCode::CREATED,
        "upload with absolute-path key should succeed"
    );

    // Retrieve — the leading slash makes this especially dangerous
    // because /storage//usr/local/... is malformed as a URL path.
    let ref_key = StorageRef::encode(&StorageKey::new("/usr/local/share/oneiros/config.toml"));

    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/storage/{ref_key}"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        resp.status(),
        StatusCode::OK,
        "GET with absolute-path key should find the uploaded artifact, but got {}",
        resp.status()
    );
}
