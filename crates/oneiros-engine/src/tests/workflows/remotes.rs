//! System workflow — remote distribution.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn add_remote_with_valid_ticket() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;

    let actor_id = {
        let client = remote.client();
        match client
            .actor()
            .list(&ListActors::builder_v1().build().into())
            .await?
        {
            ActorResponse::Listed(ActorsResponse::V1(l)) => {
                l.items.into_iter().next().unwrap().id.to_string()
            }
            o => panic!("{o:?}"),
        }
    };

    remote.command("project create --name test").await?;
    remote.command("bookmark create extra").await?;
    let host_addr = {
        let output = remote.command("bookmark share main").await?;
        output
            .prompt()
            .split_whitespace()
            .find_map(|w| w.parse::<OneirosUri>().ok())
            .and_then(|u| match u {
                OneirosUri::Peer(p) => Some(p.host),
                _ => None,
            })
            .expect("bookmark share should produce a URI")
    };

    let ticket = {
        let output = remote
            .command(&format!(
                "ticket issue --actor-id {actor_id} --project-name test"
            ))
            .await?;
        match output.into_response() {
            Responses::Ticket(TicketResponse::Created(TicketCreatedResponse::V1(c))) => c.ticket,
            o => panic!("{o:?}"),
        }
    };

    let uri = OneirosUri::Peer(PeerLink::new(host_addr, ticket.link)).to_string();
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

    let actor_id = {
        let client = remote.client();
        match client
            .actor()
            .list(&ListActors::builder_v1().build().into())
            .await?
        {
            ActorResponse::Listed(ActorsResponse::V1(l)) => {
                l.items.into_iter().next().unwrap().id.to_string()
            }
            o => panic!("{o:?}"),
        }
    };

    remote.command("project create --name test").await?;
    remote.command("bookmark create extra").await?;
    let host_addr = {
        let output = remote.command("bookmark share main").await?;
        output
            .prompt()
            .split_whitespace()
            .find_map(|w| w.parse::<OneirosUri>().ok())
            .and_then(|u| match u {
                OneirosUri::Peer(p) => Some(p.host),
                _ => None,
            })
            .expect("bookmark share should produce a URI")
    };

    let ticket = {
        let output = remote
            .command(&format!(
                "ticket issue --actor-id {actor_id} --project-name test"
            ))
            .await?;
        match output.into_response() {
            Responses::Ticket(TicketResponse::Created(TicketCreatedResponse::V1(c))) => c.ticket,
            o => panic!("{o:?}"),
        }
    };

    let uri = OneirosUri::Peer(PeerLink::new(host_addr, ticket.link)).to_string();
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

    let actor_id = {
        let client = remote.client();
        match client
            .actor()
            .list(&ListActors::builder_v1().build().into())
            .await?
        {
            ActorResponse::Listed(ActorsResponse::V1(l)) => {
                l.items.into_iter().next().unwrap().id.to_string()
            }
            o => panic!("{o:?}"),
        }
    };

    remote.command("project create --name test").await?;
    remote.command("bookmark create extra").await?;
    let host_addr = {
        let output = remote.command("bookmark share main").await?;
        output
            .prompt()
            .split_whitespace()
            .find_map(|w| w.parse::<OneirosUri>().ok())
            .and_then(|u| match u {
                OneirosUri::Peer(p) => Some(p.host),
                _ => None,
            })
            .expect("bookmark share should produce a URI")
    };

    let ticket = {
        let output = remote
            .command(&format!(
                "ticket issue --actor-id {actor_id} --project-name test"
            ))
            .await?;
        match output.into_response() {
            Responses::Ticket(TicketResponse::Created(TicketCreatedResponse::V1(c))) => c.ticket,
            o => panic!("{o:?}"),
        }
    };

    let uri = OneirosUri::Peer(PeerLink::new(host_addr, ticket.link)).to_string();
    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;
    let bookmarks = local.command("remote bookmarks dreamforge").await?;
    assert!(bookmarks.prompt().contains("extra"));
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

// ─── Push characterization tests ──────────────────────────────────

#[tokio::test]
async fn push_bookmark_to_remote() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;
    let actor_id = {
        let client = remote.client();
        match client
            .actor()
            .list(&ListActors::builder_v1().build().into())
            .await?
        {
            ActorResponse::Listed(ActorsResponse::V1(l)) => {
                l.items.into_iter().next().unwrap().id.to_string()
            }
            o => panic!("{o:?}"),
        }
    };
    remote.command("project create --name test").await?;
    remote.command("bookmark create extra").await?;
    let host_addr = {
        let output = remote.command("bookmark share main").await?;
        output
            .prompt()
            .split_whitespace()
            .find_map(|w| w.parse::<OneirosUri>().ok())
            .and_then(|u| match u {
                OneirosUri::Peer(p) => Some(p.host),
                _ => None,
            })
            .expect("bookmark share should produce a URI")
    };
    let ticket = {
        let output = remote
            .command(&format!(
                "ticket issue --actor-id {actor_id} --project-name test"
            ))
            .await?;
        match output.into_response() {
            Responses::Ticket(TicketResponse::Created(TicketCreatedResponse::V1(c))) => c.ticket,
            o => panic!("{o:?}"),
        }
    };
    let uri = OneirosUri::Peer(PeerLink::new(host_addr, ticket.link)).to_string();
    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;

    local.command("project create --name test").await?;

    // Create events and bookmark locally, then push.
    local
        .command("texture set observation --description 'Noticed'")
        .await?;
    local
        .command("texture set working --description 'Working'")
        .await?;
    local.command("bookmark create my-change").await?;
    let push_result = local.command("bookmark push dreamforge my-change").await?;
    assert!(
        push_result.prompt().contains("accepted"),
        "push result: {}",
        push_result.prompt()
    );

    // Remote should now have the bookmark with our events.
    let bookmarks = local.command("remote bookmarks dreamforge").await?;
    assert!(bookmarks.prompt().contains("my-change"));
    Ok(())
}

#[tokio::test]
async fn push_bookmark_with_as_renames() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;
    let actor_id = {
        let client = remote.client();
        match client
            .actor()
            .list(&ListActors::builder_v1().build().into())
            .await?
        {
            ActorResponse::Listed(ActorsResponse::V1(l)) => {
                l.items.into_iter().next().unwrap().id.to_string()
            }
            o => panic!("{o:?}"),
        }
    };
    remote.command("project create --name test").await?;
    remote.command("bookmark create extra").await?;
    let host_addr = {
        let output = remote.command("bookmark share main").await?;
        output
            .prompt()
            .split_whitespace()
            .find_map(|w| w.parse::<OneirosUri>().ok())
            .and_then(|u| match u {
                OneirosUri::Peer(p) => Some(p.host),
                _ => None,
            })
            .expect("bookmark share should produce a URI")
    };
    let ticket = {
        let output = remote
            .command(&format!(
                "ticket issue --actor-id {actor_id} --project-name test"
            ))
            .await?;
        match output.into_response() {
            Responses::Ticket(TicketResponse::Created(TicketCreatedResponse::V1(c))) => c.ticket,
            o => panic!("{o:?}"),
        }
    };
    let uri = OneirosUri::Peer(PeerLink::new(host_addr, ticket.link)).to_string();
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

// ─── Pull characterization tests ──────────────────────────────────

#[tokio::test]
async fn pull_bookmark_from_remote() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;
    let actor_id = {
        let client = remote.client();
        match client
            .actor()
            .list(&ListActors::builder_v1().build().into())
            .await?
        {
            ActorResponse::Listed(ActorsResponse::V1(l)) => {
                l.items.into_iter().next().unwrap().id.to_string()
            }
            o => panic!("{o:?}"),
        }
    };
    remote.command("project create --name test").await?;
    remote.command("bookmark create extra").await?;
    let host_addr = {
        let output = remote.command("bookmark share main").await?;
        output
            .prompt()
            .split_whitespace()
            .find_map(|w| w.parse::<OneirosUri>().ok())
            .and_then(|u| match u {
                OneirosUri::Peer(p) => Some(p.host),
                _ => None,
            })
            .expect("bookmark share should produce a URI")
    };
    let ticket = {
        let output = remote
            .command(&format!(
                "ticket issue --actor-id {actor_id} --project-name test"
            ))
            .await?;
        match output.into_response() {
            Responses::Ticket(TicketResponse::Created(TicketCreatedResponse::V1(c))) => c.ticket,
            o => panic!("{o:?}"),
        }
    };
    let uri = OneirosUri::Peer(PeerLink::new(host_addr, ticket.link)).to_string();
    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;

    local.command("project create --name test").await?;
    remote.command("seed core").await?;

    // Create a bookmark with events on the remote.
    remote
        .command("texture set observation --description 'On remote'")
        .await?;
    remote.command("bookmark create their-feature").await?;

    // Pull it to the local host under a new name.
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
    let actor_id = {
        let client = remote.client();
        match client
            .actor()
            .list(&ListActors::builder_v1().build().into())
            .await?
        {
            ActorResponse::Listed(ActorsResponse::V1(l)) => {
                l.items.into_iter().next().unwrap().id.to_string()
            }
            o => panic!("{o:?}"),
        }
    };
    remote.command("project create --name test").await?;
    remote.command("bookmark create extra").await?;
    let host_addr = {
        let output = remote.command("bookmark share main").await?;
        output
            .prompt()
            .split_whitespace()
            .find_map(|w| w.parse::<OneirosUri>().ok())
            .and_then(|u| match u {
                OneirosUri::Peer(p) => Some(p.host),
                _ => None,
            })
            .expect("bookmark share should produce a URI")
    };
    let ticket = {
        let output = remote
            .command(&format!(
                "ticket issue --actor-id {actor_id} --project-name test"
            ))
            .await?;
        match output.into_response() {
            Responses::Ticket(TicketResponse::Created(TicketCreatedResponse::V1(c))) => c.ticket,
            o => panic!("{o:?}"),
        }
    };
    let uri = OneirosUri::Peer(PeerLink::new(host_addr, ticket.link)).to_string();
    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;

    local.command("project create --name test").await?;

    remote.command("bookmark create their-feature").await?;

    // Read-only ticket is sufficient for pull.
    local
        .command("bookmark pull dreamforge their-feature --as my-copy")
        .await?;
    Ok(())
}

// ─── Pull characterization tests ──────────────────────────────────
// ─── Roundtrip ────────────────────────────────────────────────────

#[ignore = "needs bookmark push + pull service + CLI"]
#[tokio::test]
async fn push_pull_roundtrip() -> Result<(), Box<dyn core::error::Error>> {
    let remote = TestApp::new().await?.init_host().await?;
    let local = TestApp::new().await?.init_host().await?;
    let actor_id = {
        let client = remote.client();
        match client
            .actor()
            .list(&ListActors::builder_v1().build().into())
            .await?
        {
            ActorResponse::Listed(ActorsResponse::V1(l)) => {
                l.items.into_iter().next().unwrap().id.to_string()
            }
            o => panic!("{o:?}"),
        }
    };
    remote.command("project create --name test").await?;
    remote.command("bookmark create extra").await?;
    let host_addr = {
        let output = remote.command("bookmark share main").await?;
        output
            .prompt()
            .split_whitespace()
            .find_map(|w| w.parse::<OneirosUri>().ok())
            .and_then(|u| match u {
                OneirosUri::Peer(p) => Some(p.host),
                _ => None,
            })
            .expect("bookmark share should produce a URI")
    };
    let ticket = {
        let output = remote
            .command(&format!(
                "ticket issue --actor-id {actor_id} --project-name test"
            ))
            .await?;
        match output.into_response() {
            Responses::Ticket(TicketResponse::Created(TicketCreatedResponse::V1(c))) => c.ticket,
            o => panic!("{o:?}"),
        }
    };
    let uri = OneirosUri::Peer(PeerLink::new(host_addr, ticket.link)).to_string();
    local
        .command(&format!("remote add dreamforge --ticket {uri}"))
        .await?;

    // Push a bookmark with events.
    local
        .command("texture set observation --description 'Roundtrip event'")
        .await?;
    local.command("bookmark create to-push").await?;
    local
        .command("bookmark push dreamforge to-push --as on-remote")
        .await?;

    // Pull it back under a new name.
    local
        .command("bookmark pull dreamforge on-remote --as pulled-back")
        .await?;

    let list = local.command("bookmark list").await?;
    assert!(list.prompt().contains("pulled-back"));
    Ok(())
}
