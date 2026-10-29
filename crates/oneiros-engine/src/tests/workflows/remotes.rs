//! System workflow — peer distribution.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn add_peer_with_valid_ticket() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("project share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("peer add \"{uri}\" --name dreamforge"))
        .await?;
    let list = local.command("peer list").await?;
    assert!(list.prompt().contains("dreamforge"));
    Ok(())
}

/// Add a peer named `dreamforge` to a fresh local host, returning the
/// local app (and the remote, which must outlive it) plus the peer's id.
async fn with_added_peer() -> Result<(TestApp, TestApp, PeerId), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("project share test").await?;
    let uri = output.prompt().trim().to_string();

    let added = local
        .command(&format!("peer add \"{uri}\" --name dreamforge"))
        .await?;
    let id = match added.response() {
        Responses::Peer(PeerResponse::Added(PeerAddedResponse::V1(added))) => added.id,
        other => panic!("expected peer Added, got {other:#?}"),
    };

    Ok((remote, local, id))
}

async fn assert_peer_removed(
    local: &TestApp,
    id: PeerId,
    removal: &Rendered<Responses>,
) -> Result<(), Box<dyn core::error::Error>> {
    match removal.response() {
        Responses::Peer(PeerResponse::Removed(PeerRemovedResponse::V1(removed))) => {
            assert_eq!(removed.id, id);
        }
        other => panic!("expected peer Removed, got {other:#?}"),
    }

    let list = local.command("peer list").await?;
    assert!(!list.prompt().contains("dreamforge"));
    Ok(())
}

#[tokio::test]
async fn remove_peer_drops_from_list() -> Result<(), Box<dyn core::error::Error>> {
    let (_remote, local, id) = with_added_peer().await?;

    let removal = local.command(&format!("peer remove {id}")).await?;
    assert_peer_removed(&local, id, &removal).await
}

#[tokio::test]
async fn remove_peer_by_ref() -> Result<(), Box<dyn core::error::Error>> {
    let (_remote, local, id) = with_added_peer().await?;

    let token = RefToken::new(Ref::peer(id));
    let removal = local.command(&format!("peer remove {token}")).await?;
    assert_peer_removed(&local, id, &removal).await
}

#[tokio::test]
async fn remove_peer_by_wrong_kind_ref_is_rejected() -> Result<(), Box<dyn core::error::Error>> {
    let (_remote, local, _id) = with_added_peer().await?;

    let token = RefToken::new(Ref::connection(ConnectionId::new()));
    let Err(err) = local.command(&format!("peer remove {token}")).await else {
        panic!("expected error for wrong-kind ref, got Ok");
    };
    let message = err.to_string();
    assert!(
        message.contains("peer") && message.contains("connection"),
        "expected wrong-kind error naming both kinds, got: {message}"
    );

    let list = local.command("peer list").await?;
    assert!(list.prompt().contains("dreamforge"));
    Ok(())
}

#[tokio::test]
async fn add_peer_with_invalid_ticket_is_rejected() -> Result<(), Box<dyn core::error::Error>> {
    let local = TestApp::new().await?.init_host().await?;
    let result = local
        .command("peer add \"oneiros://nohost/link:AAAA\" --name bogus")
        .await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn list_peer_bookmarks() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    remote.command("bookmark create extra").await?;
    let output = remote.command("project share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("peer add \"{uri}\" --name dreamforge"))
        .await?;
    local.command("project create --name test").await?;
    let bookmarks = local.command("bookmark list --from dreamforge").await?;
    assert!(bookmarks.prompt().contains("extra"));
    assert!(bookmarks.prompt().contains("main"));
    Ok(())
}

#[tokio::test]
async fn submit_bookmark_to_peer() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("project share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("peer add \"{uri}\" --name dreamforge"))
        .await?;
    local.command("project create --name test").await?;
    local
        .command("texture set observation --description 'Noticed'")
        .await?;
    local
        .command("texture set working --description 'Working'")
        .await?;
    local.command("bookmark create my-change").await?;
    local
        .command("bookmark submit dreamforge my-change")
        .await?;

    let bookmarks = local.command("bookmark list --from dreamforge").await?;
    assert!(bookmarks.prompt().contains("my-change"));
    Ok(())
}

#[tokio::test]
async fn submit_bookmark_with_as_renames() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("project share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("peer add \"{uri}\" --name dreamforge"))
        .await?;
    local.command("project create --name test").await?;
    local.command("bookmark create my-change").await?;
    local
        .command("bookmark submit dreamforge my-change --as feature-x")
        .await?;

    let bookmarks = local.command("bookmark list --from dreamforge").await?;
    assert!(bookmarks.prompt().contains("feature-x"));
    Ok(())
}

/// Collect a bookmark from a peer.
#[tokio::test]
async fn collect_bookmark_from_peer() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("project share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("peer add \"{uri}\" --name dreamforge"))
        .await?;
    local.command("project create --name test").await?;
    remote.command("seed core").await?;
    remote
        .command("texture set observation --description 'On remote'")
        .await?;
    remote.command("bookmark create their-feature").await?;
    local
        .command("bookmark collect their-feature --from dreamforge --as my-copy")
        .await?;

    let list = local.command("bookmark list").await?;
    assert!(list.prompt().contains("my-copy"));
    Ok(())
}

/// Collect with a read-only ticket succeeds (uses chronicle diff protocol).
#[tokio::test]
async fn collect_with_read_only_ticket_succeeds() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("project share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("peer add \"{uri}\" --name dreamforge"))
        .await?;
    local.command("project create --name test").await?;
    remote.command("bookmark create their-feature").await?;
    local
        .command("bookmark collect their-feature --from dreamforge --as my-copy")
        .await?;
    Ok(())
}

/// Submit to peer then collect back — roundtrip.
#[tokio::test]
async fn submit_collect_roundtrip() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("project share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("peer add \"{uri}\" --name dreamforge"))
        .await?;
    local.command("project create --name test").await?;
    local
        .command("texture set observation --description 'Roundtrip event'")
        .await?;
    local.command("bookmark create to-push").await?;
    local
        .command("bookmark submit dreamforge to-push --as on-remote")
        .await?;
    local
        .command("bookmark collect on-remote --from dreamforge --as pulled-back")
        .await?;

    let list = local.command("bookmark list").await?;
    assert!(list.prompt().contains("pulled-back"));
    Ok(())
}
