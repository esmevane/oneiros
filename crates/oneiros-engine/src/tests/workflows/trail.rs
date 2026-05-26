//! Trail workflow — the events ↔ entities bridge.
//!
//! Every entity has at least one event that emitted or touched it; the
//! `trail` projection records that join. `trail of <ref>` returns the
//! events that touched an entity; `trail from <event-id>` returns the
//! entities an event emitted.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn trail_of_entity_returns_emitting_event() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();

    let agent = match client
        .agent()
        .create(
            &CreateAgent::builder_v1()
                .name(AgentName::new("governor.process"))
                .persona(PersonaName::new("process"))
                .build()
                .into(),
        )
        .await?
    {
        AgentResponse::AgentCreated(AgentCreatedResponse::V1(r)) => r.agent,
        other => panic!("expected AgentCreated, got {other:?}"),
    };

    let agent_ref = RefToken::new(Ref::agent(agent.id));

    let items = match client
        .trail()
        .of(&TrailOf::builder_v1().r#ref(agent_ref).build().into())
        .await?
    {
        TrailResponse::TrailEvents(TrailEventsResponse::V1(r)) => r.items,
        other => panic!("expected TrailEvents, got {other:?}"),
    };

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].event_type, "agent-created");

    Ok(())
}

#[tokio::test]
async fn trail_from_event_returns_emitted_entities() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();

    let agent = match client
        .agent()
        .create(
            &CreateAgent::builder_v1()
                .name(AgentName::new("governor.process"))
                .persona(PersonaName::new("process"))
                .build()
                .into(),
        )
        .await?
    {
        AgentResponse::AgentCreated(AgentCreatedResponse::V1(r)) => r.agent,
        other => panic!("expected AgentCreated, got {other:?}"),
    };

    let agent_ref = RefToken::new(Ref::agent(agent.id));

    let event_id = match client
        .trail()
        .of(&TrailOf::builder_v1()
            .r#ref(agent_ref.clone())
            .build()
            .into())
        .await?
    {
        TrailResponse::TrailEvents(TrailEventsResponse::V1(r)) => r.items[0].event_id,
        other => panic!("expected TrailEvents, got {other:?}"),
    };

    let refs = match client
        .trail()
        .from(&TrailFrom::builder_v1().event(event_id).build().into())
        .await?
    {
        TrailResponse::EmittedRefs(EmittedRefsResponse::V1(r)) => r.items,
        other => panic!("expected EmittedRefs, got {other:?}"),
    };

    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0], agent_ref);

    Ok(())
}

#[tokio::test]
async fn cognition_emits_only_cognition_ref() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();

    let _agent = match client
        .agent()
        .create(
            &CreateAgent::builder_v1()
                .name(AgentName::new("governor.process"))
                .persona(PersonaName::new("process"))
                .build()
                .into(),
        )
        .await?
    {
        AgentResponse::AgentCreated(AgentCreatedResponse::V1(r)) => r.agent,
        other => panic!("expected AgentCreated, got {other:?}"),
    };

    let cognition = match client
        .cognition()
        .add(
            &AddCognition::builder_v1()
                .agent(AgentName::new("governor.process"))
                .texture(TextureName::new("observation"))
                .content("a fragment")
                .build()
                .into(),
        )
        .await?
    {
        CognitionResponse::CognitionAdded(CognitionAddedResponse::V1(r)) => r.cognition,
        other => panic!("expected CognitionAdded, got {other:?}"),
    };

    let cognition_ref = RefToken::new(Ref::cognition(cognition.id));

    // Find the CognitionAdded event id by walking trail-of for the cognition.
    let event_id = match client
        .trail()
        .of(&TrailOf::builder_v1()
            .r#ref(cognition_ref.clone())
            .build()
            .into())
        .await?
    {
        TrailResponse::TrailEvents(TrailEventsResponse::V1(r)) => {
            assert_eq!(r.items.len(), 1);
            assert_eq!(r.items[0].event_type, "cognition-added");
            r.items[0].event_id
        }
        other => panic!("expected TrailEvents, got {other:?}"),
    };

    // The load-bearing assertion: the cognition event emits ONLY the cognition
    // ref — not the texture, not the agent. Lineage runs through the cognition
    // itself, not through sources.
    let refs = match client
        .trail()
        .from(&TrailFrom::builder_v1().event(event_id).build().into())
        .await?
    {
        TrailResponse::EmittedRefs(EmittedRefsResponse::V1(r)) => r.items,
        other => panic!("expected EmittedRefs, got {other:?}"),
    };

    assert_eq!(refs, vec![cognition_ref]);

    Ok(())
}

#[tokio::test]
async fn trail_of_unknown_ref_returns_empty() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();

    // A ref that was never emitted by any event.
    let stranger = RefToken::new(Ref::agent(AgentId::new()));

    match client
        .trail()
        .of(&TrailOf::builder_v1().r#ref(stranger).build().into())
        .await?
    {
        TrailResponse::TrailEvents(TrailEventsResponse::V1(r)) => assert!(r.items.is_empty()),
        TrailResponse::NoTrail => {}
        other => panic!("expected empty TrailEvents or NoTrail, got {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn replay_is_idempotent() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();

    let _agent = match client
        .agent()
        .create(
            &CreateAgent::builder_v1()
                .name(AgentName::new("governor.process"))
                .persona(PersonaName::new("process"))
                .build()
                .into(),
        )
        .await?
    {
        AgentResponse::AgentCreated(AgentCreatedResponse::V1(r)) => r.agent,
        other => panic!("expected AgentCreated, got {other:?}"),
    };

    let cognition = match client
        .cognition()
        .add(
            &AddCognition::builder_v1()
                .agent(AgentName::new("governor.process"))
                .texture(TextureName::new("observation"))
                .content("a fragment")
                .build()
                .into(),
        )
        .await?
    {
        CognitionResponse::CognitionAdded(CognitionAddedResponse::V1(r)) => r.cognition,
        other => panic!("expected CognitionAdded, got {other:?}"),
    };

    let cognition_ref = RefToken::new(Ref::cognition(cognition.id));

    let before = match client
        .trail()
        .of(&TrailOf::builder_v1()
            .r#ref(cognition_ref.clone())
            .build()
            .into())
        .await?
    {
        TrailResponse::TrailEvents(TrailEventsResponse::V1(r)) => r.items,
        other => panic!("expected TrailEvents, got {other:?}"),
    };

    // Drop the projections and rebuild from the event log.
    app.command("project replay").await?;

    let after = match client
        .trail()
        .of(&TrailOf::builder_v1().r#ref(cognition_ref).build().into())
        .await?
    {
        TrailResponse::TrailEvents(TrailEventsResponse::V1(r)) => r.items,
        other => panic!("expected TrailEvents, got {other:?}"),
    };

    assert_eq!(before, after);

    Ok(())
}
