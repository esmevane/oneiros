mod common;
use common::*;

/// Encode a storage key into a ref-based URI path segment.
fn storage_uri(key: &str) -> String {
    let storage_ref = StorageRef::encode(&StorageKey::new(key));
    format!("/storage/{storage_ref}")
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

    let entry: StorageEntry = body_json(response).await;
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

    let entry: StorageEntry = body_json(response).await;
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
    let entry_a: StorageEntry = body_json(response).await;

    let app = router(state);
    let response = app
        .oneshot(put_binary_auth(&storage_uri("key-b"), content, "B", &token))
        .await
        .unwrap();
    let entry_b: StorageEntry = body_json(response).await;

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

    let list: Vec<StorageEntry> = body_json(response).await;
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

    let list: Vec<StorageEntry> = body_json(response).await;
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
    let created: StorageEntry = body_json(response).await;

    let app = router(state);
    let response = app.oneshot(get_auth(&uri, &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let entry: StorageEntry = body_json(response).await;
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
