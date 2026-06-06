//! System workflow — remote distribution.

use crate::tests::harness::TestApp;
use crate::*;

/// Get the first actor ID from the host as a string for CLI use.
async fn first_actor_id(app: &TestApp) -> Result<String, Box<dyn core::error::Error>> {
    let client = app.client();
    match client
        .actor()
        .list(&ListActors::builder_v1().build().into())
        .await?
    {
        ActorResponse::Listed(ActorsResponse::V1(listed)) => Ok(listed
            .items
            .into_iter()
            .next()
            .expect("host init should create an actor")
            .id
            .to_string()),
        other => panic!("expected Listed, got {other:?}"),
    }
}

/// Get the host's peer address by sharing a bookmark and extracting from the URI.
async fn host_peer_address(app: &TestApp) -> Result<PeerAddress, Box<dyn core::error::Error>> {
    app.command("project create --name test").await?;
    app.command("bookmark create extra").await?; // creates a bookmark in the canon
    let output = app.command("bookmark share main").await?;
    let share_text = output.prompt();
    // The prompt contains the oneiros:// URI.
    for word in share_text.split_whitespace() {
        if let Ok(OneirosUri::Peer(peer_link)) = word.parse::<OneirosUri>() {
            return Ok(peer_link.host);
        }
    }
    Err("could not find oneiros:// URI in bookmark share output".into())
}

/// Issue a ticket via CLI and return the ticket.
async fn issue_ticket(
    app: &TestApp,
    actor_id: &str,
    project: &str,
    permissions: &str,
) -> Result<Ticket, Box<dyn core::error::Error>> {
    let output = app
        .command(&format!(
            "ticket issue --actor-id {actor_id} --project-name {project} {permissions}"
        ))
        .await?;
    match output.into_response() {
        Responses::Ticket(TicketResponse::Created(TicketCreatedResponse::V1(c))) => Ok(c.ticket),
        other => Err(format!("expected Created, got {other:?}").into()),
    }
}

/// Build a full ticket URI from a ticket's link and the host's peer address.
fn ticket_uri(host: &PeerAddress, ticket: &Ticket) -> String {
    let peer_link = PeerLink::new(host.clone(), ticket.link.clone());
    OneirosUri::Peer(peer_link).to_string()
}

// ─── Tests ─────────────────────────────────────────────────────────

#[tokio::test]
async fn add_remote_with_valid_ticket() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let actor_id = first_actor_id(&remote).await?;
    let host_addr = host_peer_address(&remote).await?; // creates project "test" too
    let ticket = issue_ticket(
        &remote,
        &actor_id,
        "test",
        "--permission read --permission write",
    )
    .await?;
    let uri = ticket_uri(&host_addr, &ticket);

    let local = TestApp::new().await?.init_host().await?;
    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;

    let list = local.command("remote list").await?;
    assert!(
        list.prompt().contains("dreamforge"),
        "got: {}",
        list.prompt()
    );
    Ok(())
}

#[tokio::test]
async fn remove_remote_drops_from_list() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let actor_id = first_actor_id(&remote).await?;
    let host_addr = host_peer_address(&remote).await?;
    let ticket = issue_ticket(&remote, &actor_id, "test", "--permission read").await?;
    let uri = ticket_uri(&host_addr, &ticket);

    let local = TestApp::new().await?.init_host().await?;
    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;
    local.command("remote remove dreamforge").await?;

    let list = local.command("remote list").await?;
    assert!(
        !list.prompt().contains("dreamforge"),
        "got: {}",
        list.prompt()
    );
    Ok(())
}

#[tokio::test]
async fn list_remote_bookmarks() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let actor_id = first_actor_id(&remote).await?;
    let host_addr = host_peer_address(&remote).await?;
    let ticket = issue_ticket(&remote, &actor_id, "test", "--permission read").await?;
    let uri = ticket_uri(&host_addr, &ticket);

    let local = TestApp::new().await?.init_host().await?;
    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;

    let bookmarks = local.command("remote bookmarks dreamforge").await?;
    assert!(
        bookmarks.prompt().contains("extra"),
        "got: {}",
        bookmarks.prompt()
    );
    Ok(())
}

// ─── Stubs ─────────────────────────────────────────────────────────

#[tokio::test]
async fn add_remote_with_invalid_ticket_is_rejected() -> Result<(), Box<dyn core::error::Error>> {
    Ok(())
}
#[tokio::test]
async fn push_bookmark_to_remote() -> Result<(), Box<dyn core::error::Error>> {
    Ok(())
}
#[tokio::test]
async fn push_bookmark_with_as_renames() -> Result<(), Box<dyn core::error::Error>> {
    Ok(())
}
#[tokio::test]
async fn push_with_read_only_ticket_is_rejected() -> Result<(), Box<dyn core::error::Error>> {
    Ok(())
}
#[tokio::test]
async fn pull_bookmark_from_remote() -> Result<(), Box<dyn core::error::Error>> {
    Ok(())
}
#[tokio::test]
async fn pull_with_read_only_ticket_succeeds() -> Result<(), Box<dyn core::error::Error>> {
    Ok(())
}
#[tokio::test]
async fn pull_with_write_only_ticket_is_rejected() -> Result<(), Box<dyn core::error::Error>> {
    Ok(())
}
#[tokio::test]
async fn push_pull_roundtrip() -> Result<(), Box<dyn core::error::Error>> {
    Ok(())
}
