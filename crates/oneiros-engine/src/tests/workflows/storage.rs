//! Storage workflow — archiving, retrieving, and transporting artifacts.
//!
//! Storage uses content-addressed blobs: uploads are compressed, hashed,
//! and deduplicated. This workflow exercises the full lifecycle including
//! content integrity through the export/import path.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn storage_lifecycle() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let client = app.client();

    // Upload via client
    client
        .storage()
        .upload(
            &UploadStorage::builder()
                .key("notes/design.md")
                .description("Design notes")
                .data(b"# Architecture\nEvent sourcing all the way down.".to_vec())
                .build(),
        )
        .await?;

    // Show — metadata is there
    match client
        .storage()
        .show(&GetStorage::builder().key("notes/design.md").build())
        .await?
    {
        StorageResponse::StorageDetails(entry) => {
            assert_eq!(entry.key.as_str(), "notes/design.md");
            assert_eq!(entry.description.as_str(), "Design notes");
        }
        other => panic!("expected StorageDetails, got {other:?}"),
    }

    // List — one entry
    match client.storage().list().await? {
        StorageResponse::Entries(entries) => assert_eq!(entries.len(), 1),
        other => panic!("expected Entries, got {other:?}"),
    }

    // Remove — metadata gone, system still functional
    client
        .storage()
        .remove(&RemoveStorage::builder().key("notes/design.md").build())
        .await?;

    assert!(
        client
            .storage()
            .show(&GetStorage::builder().key("notes/design.md").build())
            .await
            .is_err()
    );

    match client.storage().list().await? {
        StorageResponse::NoEntries => {}
        other => panic!("expected NoEntries, got {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn storage_via_cli() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    // Storage CLI takes a file path
    let temp = tempfile::NamedTempFile::new()?;
    std::fs::write(temp.path(), b"Hello from a temp file")?;

    let path = temp.path().display();
    app.command(&format!(
        r#"storage set test.txt {path} --description "A test file""#
    ))
    .await?;

    match app
        .client()
        .storage()
        .show(&GetStorage::builder().key("test.txt").build())
        .await?
    {
        StorageResponse::StorageDetails(entry) => {
            assert_eq!(entry.key.as_str(), "test.txt");
        }
        other => panic!("expected StorageDetails, got {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn storage_content_survives_export_import() -> Result<(), Box<dyn core::error::Error>> {
    // Instance A: upload content
    let app_a = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let content = b"Exact content that must survive the round-trip.";

    app_a
        .client()
        .storage()
        .upload(
            &UploadStorage::builder()
                .key("integrity-test.txt")
                .description("Content integrity test")
                .data(content.to_vec())
                .build(),
        )
        .await?;

    // Capture the hash for comparison
    let original_hash = match app_a
        .client()
        .storage()
        .show(&GetStorage::builder().key("integrity-test.txt").build())
        .await?
    {
        StorageResponse::StorageDetails(entry) => entry.hash,
        other => panic!("expected StorageDetails, got {other:?}"),
    };

    // Export
    let export_dir = tempfile::tempdir()?;
    app_a
        .command(&format!(
            "project export --target {}",
            export_dir.path().display()
        ))
        .await?;

    let export_file = std::fs::read_dir(export_dir.path())?
        .filter_map(|e| e.ok())
        .find(|e| e.path().extension().is_some_and(|ext| ext == "jsonl"))
        .expect("export should produce a .jsonl file");

    // Instance B: import
    let app_b = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    app_b
        .command(&format!("project import {}", export_file.path().display()))
        .await?;

    // The storage entry should exist with the same hash
    match app_b
        .client()
        .storage()
        .show(&GetStorage::builder().key("integrity-test.txt").build())
        .await?
    {
        StorageResponse::StorageDetails(entry) => {
            assert_eq!(entry.key.as_str(), "integrity-test.txt");
            assert_eq!(
                entry.hash, original_hash,
                "content hash should be identical after import"
            );
        }
        other => panic!("expected StorageDetails, got {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn storage_path_like_keys() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let client = app.client();

    // Path-like keys (with slashes) should work
    client
        .storage()
        .upload(
            &UploadStorage::builder()
                .key("notes/design/architecture.md")
                .description("Deep path")
                .data(b"nested".to_vec())
                .build(),
        )
        .await?;

    match client
        .storage()
        .show(
            &GetStorage::builder()
                .key("notes/design/architecture.md")
                .build(),
        )
        .await?
    {
        StorageResponse::StorageDetails(entry) => {
            assert_eq!(entry.key.as_str(), "notes/design/architecture.md");
        }
        other => panic!("expected StorageDetails, got {other:?}"),
    }

    // Absolute-path keys should also work
    client
        .storage()
        .upload(
            &UploadStorage::builder()
                .key("/usr/local/share/config.toml")
                .description("Absolute path")
                .data(b"absolute".to_vec())
                .build(),
        )
        .await?;

    match client
        .storage()
        .show(
            &GetStorage::builder()
                .key("/usr/local/share/config.toml")
                .build(),
        )
        .await?
    {
        StorageResponse::StorageDetails(entry) => {
            assert_eq!(entry.key.as_str(), "/usr/local/share/config.toml");
        }
        other => panic!("expected StorageDetails, got {other:?}"),
    }

    Ok(())
}
