//! Peer CRUD workflow — add, list, show, remove a known peer.
//!
//! Peers are system-level resources representing hosts we've learned
//! about. Creating one from a base64url-encoded address derives the key
//! from the address automatically, defaults the name to a hex-prefix if
//! none is supplied, and emits a `PeerAdded` event that projects into
//! the system bookmarks table.

use crate::tests::harness::TestApp;
use crate::*;

/// Construct a sample `PeerAddress` string suitable for `AddPeer`.
fn sample_address_string() -> String {
    let secret = iroh::SecretKey::generate(&mut rand::rng());
    let endpoint_id = secret.public();
    let address = PeerAddress::new(iroh::EndpointAddr::new(endpoint_id));
    address.to_string()
}

#[tokio::test]
async fn peer_crud_lifecycle() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new().await?.init_system().await?;

    let client = app.client();

    // ── No peers yet ──────────────────────────────────────────────

    match client.peer().list(&ListPeers::default()).await? {
        PeerResponse::Listed(listed) => assert_eq!(listed.len(), 0),
        other => panic!("expected Listed, got {other:?}"),
    }

    // ── Add a peer ────────────────────────────────────────────────

    let address = sample_address_string();
    let added = match client
        .peer()
        .add(&AddPeer {
            address: address.clone(),
            name: None,
        })
        .await?
    {
        PeerResponse::Added(wrapped) => {
            assert!(
                wrapped.data.name.to_string().starts_with("peer-"),
                "default name should be prefixed with 'peer-', got: {}",
                wrapped.data.name
            );
            wrapped.data
        }
        other => panic!("expected Added, got {other:?}"),
    };

    // ── Can be listed ─────────────────────────────────────────────

    match client.peer().list(&ListPeers::default()).await? {
        PeerResponse::Listed(listed) => {
            assert_eq!(listed.len(), 1);
            assert_eq!(listed.items[0].data.id, added.id);
        }
        other => panic!("expected Listed, got {other:?}"),
    }

    // ── Can be retrieved by id ────────────────────────────────────

    match client.peer().get(&added.id).await? {
        PeerResponse::Found(wrapped) => {
            assert_eq!(wrapped.data.id, added.id);
            assert_eq!(wrapped.data.key, added.key);
            assert_eq!(wrapped.data.name, added.name);
        }
        other => panic!("expected Found, got {other:?}"),
    }

    // ── Can be removed ────────────────────────────────────────────

    match client.peer().remove(&added.id).await? {
        PeerResponse::Removed(id) => assert_eq!(id, added.id),
        other => panic!("expected Removed, got {other:?}"),
    }

    // ── And the list is empty again ───────────────────────────────

    match client.peer().list(&ListPeers::default()).await? {
        PeerResponse::Listed(listed) => assert_eq!(listed.len(), 0),
        other => panic!("expected Listed, got {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn peer_explicit_name_overrides_default() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new().await?.init_system().await?;

    let client = app.client();

    let added = match client
        .peer()
        .add(&AddPeer {
            address: sample_address_string(),
            name: Some("alice".to_string()),
        })
        .await?
    {
        PeerResponse::Added(wrapped) => wrapped.data,
        other => panic!("expected Added, got {other:?}"),
    };

    assert_eq!(added.name, PeerName::new("alice"));

    Ok(())
}
