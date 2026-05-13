//! System workflow — administering the host.
//!
//! Tenants, actors, projects, and tickets form the host layer.
//! These are administrative operations that manage who can access
//! what. The host is initialized before any project work begins.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn host_administration() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new().await?.init_host().await?;

    let client = app.client();

    // Host init creates a default tenant
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

    // Host init creates a default actor, plus ours
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

    let project_name = ProjectName::new("test-project");

    match client
        .project()
        .create(
            &CreateProject::builder_v1()
                .name(project_name.clone())
                .build()
                .into(),
        )
        .await?
    {
        ProjectResponse::Created(ProjectCreatedResponse::V1(created)) => {
            assert_eq!(created.project.name, project_name);
        }
        other => panic!("expected Created, got {other:?}"),
    }

    // Duplicate project name surfaces ProjectAlreadyExists
    let response = client
        .project()
        .create(
            &CreateProject::builder_v1()
                .name(project_name.clone())
                .build()
                .into(),
        )
        .await?;
    assert!(
        matches!(response, ProjectResponse::ProjectAlreadyExists(_)),
        "duplicate project name should surface ProjectAlreadyExists, got {response:?}"
    );

    // Project is retrievable
    match client
        .project()
        .get(
            &GetProject::builder_v1()
                .key(project_name.clone())
                .build()
                .into(),
        )
        .await?
    {
        ProjectResponse::Found(ProjectFoundResponse::V1(found)) => {
            assert_eq!(found.project.name, project_name);
        }
        other => panic!("expected Found, got {other:?}"),
    }

    // Issue a ticket — grants an actor access to a project
    let ticket = match client
        .ticket()
        .issue(
            &CreateTicket::builder_v1()
                .actor_id(actor.id)
                .project_name(project_name.clone())
                .build()
                .into(),
        )
        .await?
    {
        TicketResponse::Created(TicketCreatedResponse::V1(creation)) => {
            assert_eq!(creation.ticket.project_name, project_name);
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
            assert_eq!(found.ticket.project_name, project_name);
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
            assert_eq!(validated.ticket.project_name, project_name);
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

/// Host init must produce a usable host keypair on disk, and must not
/// rotate it on subsequent calls. We wield the app: drive `system init`
/// through the CLI (HTTP-mediated) and then verify the keypair file.
#[tokio::test]
async fn host_init_creates_host_keypair() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new().await?;
    let keys = HostKey::new(app.config().platform());

    // The server's bind path generates the host key (see `ServerState::bind`),
    // but `Server::spawn` returns before that bind completes. Wait for the
    // server to handle a request — that synchronizes against bind — so the
    // key file is guaranteed to be on disk.
    let _ = HostService::status(app.config()).await;

    let key_before = keys
        .load()?
        .expect("host key should exist after server bind");

    app.command("host init --name test").await?;

    let key_after = keys
        .load()?
        .expect("host key should still exist after init");
    assert_eq!(
        key_before.to_bytes(),
        key_after.to_bytes(),
        "host init must not rotate the host keypair",
    );

    Ok(())
}
