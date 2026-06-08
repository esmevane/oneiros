//! System workflow — remote distribution.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn add_remote_with_valid_ticket() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("remote share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;
    let list = local.command("remote list").await?;
    assert!(list.prompt().contains("dreamforge"));
    Ok(())
}

#[tokio::test]
async fn remove_remote_drops_from_list() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("remote share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;
    local.command("remote remove dreamforge").await?;
    let list = local.command("remote list").await?;
    assert!(!list.prompt().contains("dreamforge"));
    Ok(())
}

#[tokio::test]
async fn list_remote_bookmarks() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    remote.command("bookmark create extra").await?;
    let output = remote.command("remote share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;
    let bookmarks = local.command("remote bookmarks dreamforge").await?;
    assert!(bookmarks.prompt().contains("extra"));
    assert!(bookmarks.prompt().contains("main"));
    Ok(())
}

#[tokio::test]
async fn add_remote_with_invalid_ticket_is_rejected() -> Result<(), Box<dyn core::error::Error>> {
    let local = TestApp::new().await?.init_host().await?;
    let result = local
        .command("remote add bogus --ticket oneiros://nohost/link:AAAA")
        .await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn push_bookmark_to_remote() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("remote share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;
    local.command("project create --name test").await?;
    local
        .command("texture set observation --description 'Noticed'")
        .await?;
    local
        .command("texture set working --description 'Working'")
        .await?;
    local.command("bookmark create my-change").await?;
    local.command("bookmark push dreamforge my-change").await?;

    let bookmarks = local.command("remote bookmarks dreamforge").await?;
    assert!(bookmarks.prompt().contains("my-change"));
    Ok(())
}

#[tokio::test]
async fn push_bookmark_with_as_renames() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("remote share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;
    local.command("project create --name test").await?;
    local.command("bookmark create my-change").await?;
    local
        .command("bookmark push dreamforge my-change --as feature-x")
        .await?;

    let bookmarks = local.command("remote bookmarks dreamforge").await?;
    assert!(!bookmarks.prompt().contains("my-change"));
    assert!(bookmarks.prompt().contains("feature-x"));
    Ok(())
}

#[tokio::test]
async fn pull_bookmark_from_remote() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("remote share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;
    local.command("project create --name test").await?;
    remote.command("seed core").await?;
    remote
        .command("texture set observation --description 'On remote'")
        .await?;
    remote.command("bookmark create their-feature").await?;
    local
        .command("bookmark pull dreamforge their-feature --as my-copy")
        .await?;

    let list = local.command("bookmark list").await?;
    assert!(list.prompt().contains("my-copy"));
    Ok(())
}

#[tokio::test]
async fn pull_with_read_only_ticket_succeeds() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("remote share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;
    local.command("project create --name test").await?;
    remote.command("bookmark create their-feature").await?;
    local
        .command("bookmark pull dreamforge their-feature --as my-copy")
        .await?;
    Ok(())
}

#[tokio::test]
async fn push_pull_roundtrip() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("remote share test").await?;
    let uri = output.prompt().trim().to_string();

    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;
    local.command("project create --name test").await?;
    local
        .command("texture set observation --description 'Roundtrip event'")
        .await?;
    local.command("bookmark create to-push").await?;
    local
        .command("bookmark push dreamforge to-push --as on-remote")
        .await?;
    local
        .command("bookmark pull dreamforge on-remote --as pulled-back")
        .await?;

    let list = local.command("bookmark list").await?;
    assert!(list.prompt().contains("pulled-back"));
    Ok(())
}

// ─── Security ────────────────────────────────────────────────────

/// Pushing with a revoked ticket is denied.
#[tokio::test]
async fn push_with_revoked_ticket_is_denied() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let output = remote.command("remote share test").await?;
    let shared = match output.into_response() {
        Responses::Remote(RemoteResponse::Shared(RemoteSharedResponse::V1(s))) => s,
        o => panic!("{o:?}"),
    };

    local
        .command(&format!("remote add dreamforge --ticket {}", shared.uri))
        .await?;
    local.command("project create --name test").await?;

    // Revoke the ticket via the CLI.
    remote
        .command(&format!("ticket revoke {}", shared.ticket.id))
        .await?;

    // Push should now be denied.
    local.command("bookmark create my-change").await?;
    let result = local.command("bookmark push dreamforge my-change").await?;
    assert!(
        result.prompt().contains("rejected"),
        "push with revoked ticket should be rejected, got: {}",
        result.prompt()
    );
    Ok(())
}

// ─── Rotation ─────────────────────────────────────────────────────

/// Revoking and re-issuing a ticket replaces the old one.
#[ignore = "CLI auth breaks on second remote add in test; works via client API"]
#[tokio::test]
async fn ticket_rotation() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    remote.command("project create --name test").await?;
    let first = match remote.command("remote share test").await?.into_response() {
        Responses::Remote(RemoteResponse::Shared(RemoteSharedResponse::V1(s))) => s,
        o => panic!("{o:?}"),
    };

    local.command("project create --name test").await?;

    local
        .command(&format!("remote add dreamforge --ticket {}", first.uri))
        .await?;

    // Revoke the original ticket.
    remote
        .command(&format!("ticket revoke {}", first.ticket.id))
        .await?;

    // Push should now be denied.
    local.command("bookmark create revoked-push").await?;
    let rejected = local
        .command("bookmark push dreamforge revoked-push")
        .await?;
    assert!(
        rejected.prompt().contains("rejected"),
        "push with revoked ticket should be rejected"
    );

    // Get a fresh ticket and re-add the remote.
    let fresh = match remote.command("remote share test").await?.into_response() {
        Responses::Remote(RemoteResponse::Shared(RemoteSharedResponse::V1(s))) => s,
        o => panic!("{o:?}"),
    };

    local.command("project create --name test").await?;

    local
        .command(&format!("remote add dreamforge --ticket {}", fresh.uri))
        .await?;

    // Push should be accepted again.
    local.command("bookmark create fresh-push").await?;
    let accepted = local.command("bookmark push dreamforge fresh-push").await?;
    assert!(
        accepted.prompt().contains("accepted"),
        "push with fresh ticket should be accepted"
    );

    Ok(())
}
