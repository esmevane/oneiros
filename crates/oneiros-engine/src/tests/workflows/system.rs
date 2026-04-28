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

    // System init creates a default tenant
    match client
        .tenant()
        .list(&ListTenants::builder_v1().build().into())
        .await?
    {
        TenantResponse::Listed(TenantsResponse::V1(tenants)) => {
            assert!(
                !tenants.items.is_empty(),
                "init should create a default tenant"
            );
        }
        other => panic!("expected Listed, got {other:?}"),
    }

    // Create another tenant
    let tenant = match client
        .tenant()
        .create(&CreateTenant::builder_v1().name("acme").build().into())
        .await?
    {
        TenantResponse::Created(TenantCreatedResponse::V1(creation)) => {
            assert_eq!(creation.tenant.name, TenantName::new("acme"));
            creation.tenant
        }
        other => panic!("expected Created, got {other:?}"),
    };

    match client
        .tenant()
        .list(&ListTenants::builder_v1().build().into())
        .await?
    {
        TenantResponse::Listed(TenantsResponse::V1(tenants)) => {
            assert_eq!(tenants.items.len(), 2)
        }
        other => panic!("expected Listed, got {other:?}"),
    }

    // Create an actor within the tenant
    let actor = match client
        .actor()
        .create(
            &CreateActor::builder_v1()
                .tenant_id(tenant.id)
                .name(ActorName::new("alice"))
                .build()
                .into(),
        )
        .await?
    {
        ActorResponse::Created(ActorCreatedResponse::V1(creation)) => {
            assert_eq!(creation.actor.name, ActorName::new("alice"));
            creation.actor
        }
        other => panic!("expected Created, got {other:?}"),
    };

    // Actor is retrievable
    match client
        .actor()
        .get(&GetActor::builder_v1().key(actor.id).build().into())
        .await?
    {
        ActorResponse::Found(ActorFoundResponse::V1(found)) => {
            assert_eq!(found.actor.name, ActorName::new("alice"));
        }
        other => panic!("expected Details, got {other:?}"),
    }

    // System init creates a default actor, plus ours
    match client
        .actor()
        .list(&ListActors::builder_v1().build().into())
        .await?
    {
        ActorResponse::Listed(ActorsResponse::V1(actors)) => {
            assert_eq!(actors.items.len(), 2)
        }
        other => panic!("expected Listed, got {other:?}"),
    }

    let brain_name = BrainName::new("test-brain");

    match client
        .brain()
        .create(
            &CreateBrain::builder_v1()
                .name(brain_name.clone())
                .build()
                .into(),
        )
        .await?
    {
        BrainResponse::Created(BrainCreatedResponse::V1(created)) => {
            assert_eq!(created.brain.name, brain_name);
        }
        other => panic!("expected Created, got {other:?}"),
    }

    // Duplicate brain name should conflict
    let result = client
        .brain()
        .create(
            &CreateBrain::builder_v1()
                .name(brain_name.clone())
                .build()
                .into(),
        )
        .await;
    assert!(result.is_err(), "duplicate brain name should conflict");

    // Brain is retrievable
    match client
        .brain()
        .get(
            &GetBrain::builder_v1()
                .key(brain_name.clone())
                .build()
                .into(),
        )
        .await?
    {
        BrainResponse::Found(BrainFoundResponse::V1(found)) => {
            assert_eq!(found.brain.name, brain_name);
        }
        other => panic!("expected Found, got {other:?}"),
    }

    // Issue a ticket — grants an actor access to a brain
    let ticket = match client
        .ticket()
        .issue(
            &CreateTicket::builder_v1()
                .actor_id(actor.id)
                .brain_name(brain_name.clone())
                .build()
                .into(),
        )
        .await?
    {
        TicketResponse::Created(TicketCreatedResponse::V1(creation)) => {
            assert_eq!(creation.ticket.brain_name, brain_name);
            creation.ticket
        }
        other => panic!("expected Created, got {other:?}"),
    };

    // Ticket is retrievable
    match client
        .ticket()
        .get(
            &GetTicket::builder_v1()
                .key(ResourceKey::Key(ticket.id))
                .build()
                .into(),
        )
        .await?
    {
        TicketResponse::Found(TicketFoundResponse::V1(found)) => {
            assert_eq!(found.ticket.brain_name, brain_name);
        }
        other => panic!("expected Details, got {other:?}"),
    }

    // Validate the token — proves the token is authentic
    match client
        .ticket()
        .validate(
            &ValidateTicket::builder_v1()
                .token(ticket.link.token.clone())
                .build()
                .into(),
        )
        .await?
    {
        TicketResponse::Validated(TicketValidatedResponse::V1(validated)) => {
            assert_eq!(validated.ticket.brain_name, brain_name);
        }
        other => panic!("expected Validated, got {other:?}"),
    }

    // List tickets
    match client
        .ticket()
        .list(&ListTickets::builder_v1().build().into())
        .await?
    {
        TicketResponse::Listed(TicketsResponse::V1(tickets)) => {
            assert!(
                !tickets.items.is_empty(),
                "should have at least the ticket we created"
            );
        }
        other => panic!("expected Listed, got {other:?}"),
    }

    Ok(())
}

/// System init must generate the host keypair so that the host
/// identity exists before the server is ever started.
#[tokio::test]
async fn system_init_creates_host_keypair() -> Result<(), Box<dyn core::error::Error>> {
    let dir = tempfile::tempdir()?;
    let config = Config::builder()
        .data_dir(dir.path().to_path_buf())
        .brain(BrainName::new("test"))
        .build();

    let context = SystemContext::new(config.clone());

    // Before init, no host key exists
    assert!(
        !config.host_key_path().exists(),
        "host key should not exist before system init"
    );

    SystemService::init(&context, &InitSystem::builder_v1().build().into()).await?;

    // After init, the host key should exist
    assert!(
        config.host_key_path().exists(),
        "system init should create the host keypair"
    );

    // The key should be loadable
    let secret = config.load_host_secret_key()?;
    assert!(secret.is_some(), "host key should be loadable after init");

    Ok(())
}
