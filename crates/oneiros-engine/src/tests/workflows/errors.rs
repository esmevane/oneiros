//! Error workflow — what happens when the world isn't right.
//!
//! Every layer should surface errors consistently. A nonexistent agent
//! should fail through the CLI, the typed client, and MCP alike. These
//! tests prove the error contract across boundaries.

use crate::tests::harness::TestApp;
use crate::*;

/// Project-scoped routes reject unauthenticated and badly-authenticated
/// requests. System routes remain open.
#[tokio::test]
async fn auth_boundaries() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let base_url = app.base_url();

    let no_token = Client::new(base_url.clone());
    let agent_client = AgentClient::new(&no_token);
    assert!(
        agent_client
            .list(&ListAgents::builder().build())
            .await
            .is_err(),
        "project routes should reject unauthenticated requests"
    );

    let bad_token = Client::with_token(base_url.clone(), Token::from("not-a-real-token"))?;
    let agent_client = AgentClient::new(&bad_token);
    assert!(
        agent_client
            .list(&ListAgents::builder().build())
            .await
            .is_err(),
        "project routes should reject invalid tokens"
    );

    let good_client = app.client();
    assert!(
        good_client
            .agent()
            .list(&ListAgents::builder().build())
            .await
            .is_ok(),
        "project routes should accept valid tokens"
    );

    let no_token = Client::new(base_url);
    let tenant_client = TenantClient::new(&no_token);
    assert!(
        tenant_client
            .list(&ListTenants::builder().build())
            .await
            .is_ok(),
        "system routes should not require auth"
    );

    Ok(())
}

#[tokio::test]
async fn missing_entities() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();
    let ghost = AgentName::new("ghost.nobody");

    let result = client
        .agent()
        .get(&GetAgent::builder().key(ghost.clone()).build())
        .await;
    assert!(result.is_err(), "nonexistent agent should 404 via client");

    let result = app.command("agent show ghost.nobody").await;
    assert!(result.is_err(), "nonexistent agent should fail via command");

    let result = client
        .persona()
        .get(&GetPersona::builder().key(PersonaName::new("nope")).build())
        .await;
    assert!(result.is_err(), "nonexistent persona should 404 via client");

    let result = app.command("persona show nope").await;
    assert!(
        result.is_err(),
        "nonexistent persona should fail via command"
    );

    assert!(
        client
            .level()
            .get(&GetLevel::builder().key(LevelName::new("nope")).build())
            .await
            .is_err()
    );
    assert!(
        client
            .texture()
            .get(&GetTexture::builder().key(TextureName::new("nope")).build())
            .await
            .is_err()
    );
    assert!(
        client
            .nature()
            .get(&GetNature::builder().key(NatureName::new("nope")).build())
            .await
            .is_err()
    );
    assert!(
        client
            .sensation()
            .get(
                &GetSensation::builder()
                    .key(SensationName::new("nope"))
                    .build()
            )
            .await
            .is_err()
    );
    assert!(
        client
            .urge()
            .get(&GetUrge::builder().key(UrgeName::new("nope")).build())
            .await
            .is_err()
    );

    assert!(
        client
            .storage()
            .show(&GetStorage::builder().key(StorageKey::new("nope")).build())
            .await
            .is_err()
    );

    let fake_id = Id(uuid::Uuid::nil());
    assert!(
        client
            .memory()
            .get(&GetMemory::builder().key(MemoryId::from(fake_id)).build())
            .await
            .is_err(),
        "nonexistent memory should 404"
    );

    assert!(
        client
            .experience()
            .get(
                &GetExperience::builder()
                    .key(ExperienceId::from(fake_id))
                    .build()
            )
            .await
            .is_err(),
        "nonexistent experience should 404"
    );

    assert!(
        client
            .connection()
            .get(
                &GetConnection::builder()
                    .key(ConnectionId::from(fake_id))
                    .build()
            )
            .await
            .is_err(),
        "nonexistent connection should 404"
    );

    assert!(
        client.continuity().dream(&ghost).await.is_err(),
        "dreaming nonexistent agent should fail via client"
    );

    assert!(
        client.continuity().introspect(&ghost).await.is_err(),
        "introspecting nonexistent agent should fail via client"
    );

    assert!(
        client.continuity().sleep(&ghost).await.is_err(),
        "sleeping nonexistent agent should fail via client"
    );

    // Pressure doesn't validate agent existence — it returns empty readings.
    // This documents the current behavior.

    match client
        .pressure()
        .get(&GetPressure::builder().agent(ghost).build())
        .await?
    {
        PressureResponse::Readings(r) => {
            assert!(
                r.pressures.is_empty(),
                "nonexistent agent should have no pressures"
            );
        }
        other => panic!("expected Readings, got {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn invalid_references() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();
    let ghost = AgentName::new("nobody.nobody");

    let result = client
        .agent()
        .create(
            &CreateAgent::builder()
                .name("ghost")
                .persona("nonexistent")
                .build(),
        )
        .await;
    assert!(
        result.is_err(),
        "agent with nonexistent persona should fail via client"
    );

    let result = app.command("agent create ghost nonexistent").await;
    assert!(
        result.is_err(),
        "agent with nonexistent persona should fail via command"
    );

    // Via CLI
    let result = app
        .command(r#"cognition add nobody.nobody observation "hello""#)
        .await;
    assert!(
        result.is_err(),
        "cognition for nonexistent agent should fail via command"
    );

    // Via client
    let result = client
        .cognition()
        .add(
            &AddCognition::builder()
                .agent(ghost.clone())
                .texture("observation")
                .content("hello")
                .build(),
        )
        .await;
    assert!(
        result.is_err(),
        "cognition for nonexistent agent should fail via client"
    );

    let result = app
        .command(r#"memory add nobody.nobody session "hello""#)
        .await;
    assert!(
        result.is_err(),
        "memory for nonexistent agent should fail via command"
    );

    let result = client
        .memory()
        .add(
            &AddMemory::builder()
                .agent(ghost.clone())
                .level("session")
                .content("hello")
                .build(),
        )
        .await;
    assert!(
        result.is_err(),
        "memory for nonexistent agent should fail via client"
    );

    let result = app
        .command(r#"experience create nobody.nobody echoes "hello""#)
        .await;
    assert!(
        result.is_err(),
        "experience for nonexistent agent should fail via command"
    );

    let result = client
        .experience()
        .create(
            &CreateExperience::builder()
                .agent(ghost.clone())
                .sensation("echoes")
                .description("hello")
                .build(),
        )
        .await;
    assert!(
        result.is_err(),
        "experience for nonexistent agent should fail via client"
    );

    assert!(app.command("dream nobody.nobody").await.is_err());
    assert!(app.command("introspect nobody.nobody").await.is_err());
    assert!(app.command("sleep nobody.nobody").await.is_err());

    Ok(())
}

#[tokio::test]
async fn duplicate_entities() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();

    app.command("agent create gov process").await?;

    // Via client
    let result = client
        .agent()
        .create(
            &CreateAgent::builder()
                .name("gov")
                .persona("process")
                .build(),
        )
        .await;
    assert!(
        result.is_err(),
        "duplicate agent should conflict via client"
    );

    // Via command
    let result = app.command("agent create gov process").await;
    assert!(
        result.is_err(),
        "duplicate agent should conflict via command"
    );

    client
        .brain()
        .create(&CreateBrain::builder().name("dupe-brain").build())
        .await?;

    let result = client
        .brain()
        .create(&CreateBrain::builder().name("dupe-brain").build())
        .await;
    assert!(result.is_err(), "duplicate brain should conflict");

    app.command(r#"persona set custom --description "First" --prompt """#)
        .await?;
    app.command(r#"persona set custom --description "Second" --prompt """#)
        .await?;

    match client
        .persona()
        .get(
            &GetPersona::builder()
                .key(PersonaName::new("custom"))
                .build(),
        )
        .await?
    {
        PersonaResponse::PersonaDetails(p) => {
            assert_eq!(
                p.data.description().to_string(),
                "Second",
                "second set should win"
            );
        }
        other => panic!("expected PersonaDetails, got {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn removing_nonexistent_entities() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();

    // Removing a level that doesn't exist — should not corrupt state
    let _ = app.command("level remove nonexistent").await;

    // Removing a nonexistent storage key
    let _ = client
        .storage()
        .remove(&RemoveStorage::builder().key("nope").build())
        .await;

    // Removing a nonexistent connection
    let fake_id = Id(uuid::Uuid::nil());
    let _ = client
        .connection()
        .remove(
            &RemoveConnection::builder()
                .id(ConnectionId::from(fake_id))
                .build(),
        )
        .await;

    // After all the remove attempts, the system should still be functional
    app.command(r#"level set working --description "Still works" --prompt """#)
        .await?;

    match client
        .level()
        .get(&GetLevel::builder().key(LevelName::new("working")).build())
        .await?
    {
        LevelResponse::LevelDetails(l) => {
            assert_eq!(l.data.description().to_string(), "Still works");
        }
        other => panic!("expected LevelDetails, got {other:?}"),
    }

    Ok(())
}
