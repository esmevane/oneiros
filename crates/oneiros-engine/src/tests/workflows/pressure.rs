//! Pressure workflow — cognitive pressure builds, is observed, and decreases.
//!
//! Pressure is the system's signal that cognitive hygiene is needed.
//! As an agent accumulates thoughts without consolidation, pressure
//! builds across urge dimensions. Responding to urges — introspecting,
//! reflecting, drawing connections, consolidating memories — should
//! reduce pressure. This is the actuation layer.

use crate::tests::harness::TestApp;
use crate::*;

/// Helper: get the urgency for a specific urge from pressure readings.
fn urgency_for(pressures: &[Pressure], urge: &str) -> Option<f64> {
    pressures
        .iter()
        .find(|p| p.urge.as_str() == urge)
        .map(|p| p.data.urgency())
}

#[tokio::test]
async fn pressure_builds_from_activity() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();
    let agent = AgentName::new("thinker.process");

    app.command("emerge thinker process").await?;

    // Add cognitive activity without consolidation
    for i in 0..10 {
        app.command(&format!(
            r#"cognition add thinker.process observation "Thought number {i}""#
        ))
        .await?;
    }

    // Pressure should exist after activity
    let readings = match client
        .pressure()
        .get(&GetPressure::builder().agent(agent.clone()).build())
        .await?
    {
        PressureResponse::Readings(r) => r,
        other => panic!("expected Readings, got {other:?}"),
    };

    assert!(
        !readings.pressures.is_empty(),
        "should have pressure readings after activity"
    );

    // Dream should include pressure readings paired with urge CTAs
    match client.continuity().dream(&agent).await? {
        ContinuityResponse::Dreaming(ctx) => {
            assert!(!ctx.pressures.is_empty(), "dream should include pressures");
            for reading in &ctx.pressures {
                assert!(
                    !reading.pressure.urge.as_str().is_empty(),
                    "pressure should reference an urge"
                );
            }
        }
        other => panic!("expected Dreaming, got {other:?}"),
    }

    // List all pressures
    match client.pressure().list().await? {
        PressureResponse::AllReadings(all) => {
            assert!(!all.pressures.is_empty());
        }
        other => panic!("expected AllReadings, got {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn introspecting_reduces_introspect_pressure() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();
    let agent = AgentName::new("thinker.process");

    app.command("emerge thinker process").await?;

    // Build up introspect pressure: many cognitions, no consolidation
    for i in 0..10 {
        app.command(&format!(
            r#"cognition add thinker.process working "Working thought {i}""#
        ))
        .await?;
    }

    let before = match client
        .pressure()
        .get(&GetPressure::builder().agent(agent.clone()).build())
        .await?
    {
        PressureResponse::Readings(r) => r.pressures,
        other => panic!("expected Readings, got {other:?}"),
    };

    let introspect_before =
        urgency_for(&before, "introspect").expect("should have introspect pressure");
    assert!(
        introspect_before > 0.0,
        "introspect pressure should be nonzero"
    );

    // Respond: add memories (improves promotion rate) and introspect
    app.command(r#"memory add thinker.process session "Consolidating my working thoughts""#)
        .await?;
    client.continuity().introspect(&agent).await?;

    let after = match client
        .pressure()
        .get(&GetPressure::builder().agent(agent.clone()).build())
        .await?
    {
        PressureResponse::Readings(r) => r.pressures,
        other => panic!("expected Readings, got {other:?}"),
    };

    let introspect_after =
        urgency_for(&after, "introspect").expect("should still have introspect pressure");

    assert!(
        introspect_after < introspect_before,
        "introspect pressure should decrease after introspecting \
         (before: {introspect_before:.3}, after: {introspect_after:.3})"
    );

    Ok(())
}

#[tokio::test]
async fn connecting_reduces_catharsis_pressure() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();
    let agent = AgentName::new("thinker.process");

    app.command("emerge thinker process").await?;

    // Build catharsis pressure: orphaned cognitions (no connections)
    for i in 0..5 {
        app.command(&format!(
            r#"cognition add thinker.process observation "Orphaned thought {i}""#
        ))
        .await?;
    }

    let before = match client
        .pressure()
        .get(&GetPressure::builder().agent(agent.clone()).build())
        .await?
    {
        PressureResponse::Readings(r) => r.pressures,
        other => panic!("expected Readings, got {other:?}"),
    };

    let catharsis_before =
        urgency_for(&before, "catharsis").expect("should have catharsis pressure");

    // Respond: create an experience and connect cognitions to it
    let experience = match client
        .experience()
        .create(
            &CreateExperience::builder()
                .agent(agent.clone())
                .sensation("distills")
                .description("These thoughts crystallize into understanding")
                .build(),
        )
        .await?
    {
        ExperienceResponse::ExperienceCreated(e) => e,
        other => panic!("expected ExperienceCreated, got {other:?}"),
    };

    // Get a cognition to connect
    let cognitions = match client
        .cognition()
        .list(&ListCognitions {
            agent: Some(agent.clone()),
            texture: None,
            filters: SearchFilters::default(),
        })
        .await?
    {
        CognitionResponse::Cognitions(c) => c.items,
        other => panic!("expected Cognitions, got {other:?}"),
    };

    // Connect several cognitions to the experience
    for cog in cognitions.iter().take(3) {
        client
            .connection()
            .create(
                &CreateConnection::builder()
                    .from_ref(RefToken::new(Ref::cognition(cog.data.id())))
                    .to_ref(RefToken::new(Ref::experience(experience.data.id())))
                    .nature("context")
                    .build(),
            )
            .await?;
    }

    // Also reflect to reset the reflection timer
    client.continuity().reflect(&agent).await?;

    let after = match client
        .pressure()
        .get(&GetPressure::builder().agent(agent.clone()).build())
        .await?
    {
        PressureResponse::Readings(r) => r.pressures,
        other => panic!("expected Readings, got {other:?}"),
    };

    let catharsis_after =
        urgency_for(&after, "catharsis").expect("should still have catharsis pressure");

    assert!(
        catharsis_after < catharsis_before,
        "catharsis pressure should decrease after connecting orphaned thoughts and reflecting \
         (before: {catharsis_before:.3}, after: {catharsis_after:.3})"
    );

    Ok(())
}

#[tokio::test]
async fn consolidating_reduces_recollect_pressure() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();
    let agent = AgentName::new("thinker.process");

    app.command("emerge thinker process").await?;

    // Build recollect pressure: unconnected experiences
    for i in 0..3 {
        client
            .experience()
            .create(
                &CreateExperience::builder()
                    .agent(agent.clone())
                    .sensation("echoes")
                    .description(format!("Unconnected experience {i}"))
                    .build(),
            )
            .await?;
    }

    // Add some cognitions to trigger pressure computation
    app.command(r#"cognition add thinker.process observation "Trigger""#)
        .await?;

    let before = match client
        .pressure()
        .get(&GetPressure::builder().agent(agent.clone()).build())
        .await?
    {
        PressureResponse::Readings(r) => r.pressures,
        other => panic!("expected Readings, got {other:?}"),
    };

    let recollect_before =
        urgency_for(&before, "recollect").expect("should have recollect pressure");

    // Respond: connect the experiences and add a memory
    let experiences = match client
        .experience()
        .list(&ListExperiences {
            agent: Some(agent.clone()),
            filters: SearchFilters::default(),
        })
        .await?
    {
        ExperienceResponse::Experiences(e) => e.items,
        other => panic!("expected Experiences, got {other:?}"),
    };

    // Connect experiences to each other
    if experiences.len() >= 2 {
        client
            .connection()
            .create(
                &CreateConnection::builder()
                    .from_ref(RefToken::new(Ref::experience(experiences[0].data.id())))
                    .to_ref(RefToken::new(Ref::experience(experiences[1].data.id())))
                    .nature("reference")
                    .build(),
            )
            .await?;
    }

    // Add a memory — resets "time since last memory" and improves consolidation
    app.command(r#"memory add thinker.process project "These experiences are connected""#)
        .await?;

    let after = match client
        .pressure()
        .get(&GetPressure::builder().agent(agent.clone()).build())
        .await?
    {
        PressureResponse::Readings(r) => r.pressures,
        other => panic!("expected Readings, got {other:?}"),
    };

    let recollect_after =
        urgency_for(&after, "recollect").expect("should still have recollect pressure");

    assert!(
        recollect_after < recollect_before,
        "recollect pressure should decrease after connecting experiences and adding memories \
         (before: {recollect_before:.3}, after: {recollect_after:.3})"
    );

    Ok(())
}
