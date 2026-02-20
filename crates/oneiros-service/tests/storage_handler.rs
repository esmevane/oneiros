use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use oneiros_db::Database;
use oneiros_model::*;
use oneiros_protocol::*;
use oneiros_service::*;
use std::sync::Arc;
use tempfile::TempDir;
use tower::util::ServiceExt;

fn seed_tenant_and_brain(db: &Database, brain_path: &std::path::Path) -> String {
    let tenant_id = TenantId::new();
    let actor_id = ActorId::new();

    let event = Events::Tenant(TenantEvents::TenantCreated(Identity::new(
        tenant_id,
        Tenant {
            name: TenantName::new("Test Tenant"),
        },
    )));
    db.log_event(&event, projections::SYSTEM_PROJECTIONS)
        .unwrap();

    let event = Events::Actor(ActorEvents::ActorCreated(Identity::new(
        actor_id,
        Actor {
            tenant_id,
            name: ActorName::new("Test Actor"),
        },
    )));
    db.log_event(&event, projections::SYSTEM_PROJECTIONS)
        .unwrap();

    Database::create_brain_db(brain_path).unwrap();

    let brain_id = BrainId::new();
    let event = Events::Brain(BrainEvents::BrainCreated(Identity::new(
        brain_id,
        Brain {
            tenant_id,
            name: BrainName::new("test-brain"),
            path: brain_path.to_path_buf(),
            status: BrainStatus::Active,
        },
    )));
    db.log_event(&event, projections::SYSTEM_PROJECTIONS)
        .unwrap();

    let token = Token::issue(TokenClaims {
        brain_id,
        tenant_id,
        actor_id,
    });

    let event = Events::Ticket(TicketEvents::TicketIssued(Identity::new(
        TicketId::new(),
        Ticket {
            token: token.clone(),
            created_by: actor_id,
        },
    )));
    db.log_event(&event, projections::SYSTEM_PROJECTIONS)
        .unwrap();

    token.0
}

fn setup() -> (TempDir, Arc<ServiceState>, String) {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("service.db");
    let db = Database::create(db_path).unwrap();

    let brain_path = temp.path().join("brains").join("test-brain.db");
    std::fs::create_dir_all(brain_path.parent().unwrap()).unwrap();
    let token = seed_tenant_and_brain(&db, &brain_path);

    let state = Arc::new(ServiceState::new(db, temp.path().to_path_buf()));
    (temp, state, token)
}

/// Encode a storage key into a ref-based URI path segment.
fn storage_uri(key: &str) -> String {
    let storage_ref = StorageRef::encode(&StorageKey::new(key));
    format!("/storage/{storage_ref}")
}

fn get_auth(uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

fn put_binary_auth(uri: &str, body: &[u8], description: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::PUT)
        .uri(uri)
        .header("content-type", "application/octet-stream")
        .header("authorization", format!("Bearer {token}"))
        .header("x-storage-description", description)
        .body(Body::from(body.to_vec()))
        .unwrap()
}

fn delete_auth(uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::DELETE)
        .uri(uri)
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

#[tokio::test]
async fn set_storage_returns_ok() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let content = b"hello world";
    let response = app
        .oneshot(put_binary_auth(
            &storage_uri("greeting"),
            content,
            "A greeting",
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let entry: StorageEntry = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(entry.key, StorageKey::new("greeting"));
    assert_eq!(entry.description.as_str(), "A greeting");
    assert!(!entry.hash.as_str().is_empty());
}

#[tokio::test]
async fn set_storage_is_idempotent() {
    let (_temp, state, token) = setup();

    let content = b"hello world";

    let uri = storage_uri("greeting");

    let app = router(state.clone());
    let response = app
        .oneshot(put_binary_auth(&uri, content, "Version 1", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Update description
    let app = router(state.clone());
    let response = app
        .oneshot(put_binary_auth(&uri, content, "Version 2", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify updated description
    let app = router(state);
    let response = app.oneshot(get_auth(&uri, &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let entry: StorageEntry = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(entry.description.as_str(), "Version 2");
}

#[tokio::test]
async fn set_storage_deduplicates_content() {
    let (_temp, state, token) = setup();

    let content = b"same content";

    let app = router(state.clone());
    let response = app
        .oneshot(put_binary_auth(&storage_uri("key-a"), content, "A", &token))
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let entry_a: StorageEntry = serde_json::from_slice(&bytes).unwrap();

    let app = router(state);
    let response = app
        .oneshot(put_binary_auth(&storage_uri("key-b"), content, "B", &token))
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let entry_b: StorageEntry = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(entry_a.hash, entry_b.hash);
}

#[tokio::test]
async fn get_storage_content_returns_original_data() {
    let (_temp, state, token) = setup();

    let content = b"binary content \x00\x01\x02\xff";

    let uri = storage_uri("binary-file");

    let app = router(state.clone());
    app.oneshot(put_binary_auth(&uri, content, "Binary test", &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app
        .oneshot(get_auth(&format!("{uri}/content"), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(bytes.as_ref(), content);
}

#[tokio::test]
async fn get_storage_not_found() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth(&storage_uri("nonexistent"), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn list_storage_empty() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app.oneshot(get_auth("/storage", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<StorageEntry> = serde_json::from_slice(&bytes).unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_storage_after_set() {
    let (_temp, state, token) = setup();

    let app = router(state.clone());
    app.oneshot(put_binary_auth(
        &storage_uri("alpha"),
        b"aaa",
        "Alpha",
        &token,
    ))
    .await
    .unwrap();

    let app = router(state.clone());
    app.oneshot(put_binary_auth(
        &storage_uri("beta"),
        b"bbb",
        "Beta",
        &token,
    ))
    .await
    .unwrap();

    let app = router(state);
    let response = app.oneshot(get_auth("/storage", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<StorageEntry> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn show_storage_metadata() {
    let (_temp, state, token) = setup();

    let uri = storage_uri("doc");

    let app = router(state.clone());
    let response = app
        .oneshot(put_binary_auth(
            &uri,
            b"document content",
            "A document",
            &token,
        ))
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: StorageEntry = serde_json::from_slice(&bytes).unwrap();

    let app = router(state);
    let response = app.oneshot(get_auth(&uri, &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let entry: StorageEntry = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(entry.key, StorageKey::new("doc"));
    assert_eq!(entry.description.as_str(), "A document");
    assert_eq!(entry.hash, created.hash);
}

#[tokio::test]
async fn remove_storage_then_gone() {
    let (_temp, state, token) = setup();

    let uri = storage_uri("ephemeral");

    let app = router(state.clone());
    app.oneshot(put_binary_auth(&uri, b"temp", "Temp", &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let response = app.oneshot(delete_auth(&uri, &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let app = router(state);
    let response = app.oneshot(get_auth(&uri, &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn remove_storage_leaves_blob_for_other_keys() {
    let (_temp, state, token) = setup();

    let content = b"shared content";
    let uri_a = storage_uri("key-a");
    let uri_b = storage_uri("key-b");

    let app = router(state.clone());
    app.oneshot(put_binary_auth(&uri_a, content, "A", &token))
        .await
        .unwrap();

    let app = router(state.clone());
    app.oneshot(put_binary_auth(&uri_b, content, "B", &token))
        .await
        .unwrap();

    // Remove key-a
    let app = router(state.clone());
    app.oneshot(delete_auth(&uri_a, &token)).await.unwrap();

    // key-b content should still work
    let app = router(state);
    let response = app
        .oneshot(get_auth(&format!("{uri_b}/content"), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(bytes.as_ref(), content);
}

#[tokio::test]
async fn storage_request_without_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/storage")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn storage_request_with_invalid_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/storage", "bogus-token-value"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_storage_by_link() {
    let (_temp, state, token) = setup();

    let uri = storage_uri("linked-doc");

    let app = router(state.clone());
    app.oneshot(put_binary_auth(
        &uri,
        b"link test content",
        "A linked doc",
        &token,
    ))
    .await
    .unwrap();

    // Fetch by StorageRef to get the link
    let app = router(state.clone());
    let response = app.oneshot(get_auth(&uri, &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let link = value["link"].as_str().unwrap().to_string();
    assert!(!link.is_empty());

    // Fetch by link
    let app = router(state);
    let response = app
        .oneshot(get_auth(&format!("/storage/{link}"), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let fetched: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(fetched["description"], "A linked doc");
    assert_eq!(fetched["link"], link);
}
