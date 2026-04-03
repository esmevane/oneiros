//! System workflow — administering the host.
//!
//! Tenants, actors, brains, and tickets form the system layer.
//! These are administrative operations that manage who can access
//! what. The system is initialized before any project work begins.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn system_administration() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new().await?.init_system().await?;

    let client = app.client();

    // ── Tenants ─────────────────────────────────────────────────

    // System init creates a default tenant
    match client
        .tenant()
        .list(&ListTenants {
            filters: SearchFilters::default(),
        })
        .await?
    {
        TenantResponse::Listed(tenants) => {
            assert!(!tenants.is_empty(), "init should create a default tenant");
        }
        other => panic!("expected Listed, got {other:?}"),
    }

    // Create another tenant
    let tenant = match client
        .tenant()
        .create(&CreateTenant::builder().name("acme").build())
        .await?
    {
        TenantResponse::Created(t) => {
            assert_eq!(t.data.name, TenantName::new("acme"));
            t
        }
        other => panic!("expected Created, got {other:?}"),
    };

    match client
        .tenant()
        .list(&ListTenants {
            filters: SearchFilters::default(),
        })
        .await?
    {
        TenantResponse::Listed(tenants) => assert_eq!(tenants.len(), 2),
        other => panic!("expected Listed, got {other:?}"),
    }

    // ── Actors ──────────────────────────────────────────────────

    // Create an actor within the tenant
    let actor = match client
        .actor()
        .create(
            &CreateActor::builder()
                .tenant_id(tenant.data.id)
                .name(ActorName::new("alice"))
                .build(),
        )
        .await?
    {
        ActorResponse::Created(a) => {
            assert_eq!(a.data.name, ActorName::new("alice"));
            a
        }
        other => panic!("expected Created, got {other:?}"),
    };

    // Actor is retrievable
    match client.actor().get(&actor.data.id).await? {
        ActorResponse::Found(a) => {
            assert_eq!(a.data.name, ActorName::new("alice"));
        }
        other => panic!("expected Details, got {other:?}"),
    }

    // System init creates a default actor, plus ours
    match client
        .actor()
        .list(&ListActors {
            filters: SearchFilters::default(),
        })
        .await?
    {
        ActorResponse::Listed(actors) => assert_eq!(actors.len(), 2),
        other => panic!("expected Listed, got {other:?}"),
    }

    // ── Brains ──────────────────────────────────────────────────

    let brain_name = BrainName::new("test-brain");

    match client
        .brain()
        .create(&CreateBrain::builder().name(brain_name.clone()).build())
        .await?
    {
        BrainResponse::Created(b) => {
            assert_eq!(b.data.name, brain_name);
        }
        other => panic!("expected Created, got {other:?}"),
    }

    // Duplicate brain name should conflict
    let result = client
        .brain()
        .create(&CreateBrain::builder().name(brain_name.clone()).build())
        .await;
    assert!(result.is_err(), "duplicate brain name should conflict");

    // Brain is retrievable
    match client.brain().get(&brain_name).await? {
        BrainResponse::Found(b) => {
            assert_eq!(b.data.name, brain_name);
        }
        other => panic!("expected Found, got {other:?}"),
    }

    // ── Tickets ─────────────────────────────────────────────────

    // Issue a ticket — grants an actor access to a brain
    let ticket = match client
        .ticket()
        .issue(
            &CreateTicket::builder()
                .actor_id(actor.data.id)
                .brain_name(brain_name.clone())
                .build(),
        )
        .await?
    {
        TicketResponse::Created(t) => {
            assert_eq!(t.brain_name, brain_name);
            t
        }
        other => panic!("expected Created, got {other:?}"),
    };

    // Ticket is retrievable
    match client.ticket().get(&ticket.id).await? {
        TicketResponse::Found(t) => {
            assert_eq!(t.brain_name, brain_name);
        }
        other => panic!("expected Details, got {other:?}"),
    }

    // Validate the token — proves the token is authentic
    match client
        .ticket()
        .validate(
            &ValidateTicket::builder()
                .token(ticket.token.clone())
                .build(),
        )
        .await?
    {
        TicketResponse::Validated(t) => {
            assert_eq!(t.brain_name, brain_name);
        }
        other => panic!("expected Validated, got {other:?}"),
    }

    // List tickets
    match client
        .ticket()
        .list(&ListTickets {
            filters: SearchFilters::default(),
        })
        .await?
    {
        TicketResponse::Listed(tickets) => {
            assert!(
                !tickets.is_empty(),
                "should have at least the ticket we created"
            );
        }
        other => panic!("expected Listed, got {other:?}"),
    }

    Ok(())
}
