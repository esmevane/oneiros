//! Vocabulary workflow — define and manage the language of cognition.
//!
//! Oneiros has six vocabulary domains: levels, textures, sensations,
//! natures, personas, and urges. Each follows set/get/list/remove.
//! This workflow proves the full lifecycle and cross-layer equivalence.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn vocabulary_lifecycle() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let client = app.client();

    // ── Levels ──────────────────────────────────────────────────

    app.command(r#"level set working --description "Active processing" --prompt "What you're actively processing""#)
        .await?;
    app.command(r#"level set session --description "Session context" --prompt "Current session""#)
        .await?;

    match client
        .level()
        .list(&ListLevels {
            filters: SearchFilters::default(),
        })
        .await?
    {
        LevelResponse::Levels(levels) => assert_eq!(levels.len(), 2),
        other => panic!("expected Levels, got {other:?}"),
    }

    match client.level().get(&LevelName::new("working")).await? {
        LevelResponse::LevelDetails(l) => {
            assert_eq!(l.data.description.to_string(), "Active processing");
        }
        other => panic!("expected LevelDetails, got {other:?}"),
    }

    // Remove and verify
    app.command("level remove session").await?;
    assert!(
        client
            .level()
            .get(&LevelName::new("session"))
            .await
            .is_err()
    );

    match client
        .level()
        .list(&ListLevels {
            filters: SearchFilters::default(),
        })
        .await?
    {
        LevelResponse::Levels(levels) => assert_eq!(levels.len(), 1),
        other => panic!("expected Levels, got {other:?}"),
    }

    // ── Textures ────────────────────────────────────────────────

    app.command(r#"texture set observation --description "Noticing" --prompt "When you notice""#)
        .await?;

    match client
        .texture()
        .get(&TextureName::new("observation"))
        .await?
    {
        TextureResponse::TextureDetails(t) => {
            assert_eq!(t.data.name, TextureName::new("observation"));
        }
        other => panic!("expected TextureDetails, got {other:?}"),
    }

    // ── Sensations ──────────────────────────────────────────────

    app.command(r#"sensation set echoes --description "Resonance" --prompt "Things that rhyme""#)
        .await?;

    match client
        .sensation()
        .get(&SensationName::new("echoes"))
        .await?
    {
        SensationResponse::SensationDetails(s) => {
            assert_eq!(s.data.name, SensationName::new("echoes"));
        }
        other => panic!("expected SensationDetails, got {other:?}"),
    }

    // ── Natures ─────────────────────────────────────────────────

    app.command(r#"nature set reference --description "Related" --prompt "Cross-reference""#)
        .await?;

    match client.nature().get(&NatureName::new("reference")).await? {
        NatureResponse::NatureDetails(n) => {
            assert_eq!(n.data.name, NatureName::new("reference"));
        }
        other => panic!("expected NatureDetails, got {other:?}"),
    }

    // ── Personas ────────────────────────────────────────────────

    app.command(
        r#"persona set process --description "Process agents" --prompt "You manage process""#,
    )
    .await?;

    // Command and client agree
    let cmd_result = app.command("persona show process").await?;
    let cmd_json = serde_json::to_value(cmd_result.response())?;

    match client.persona().get(&PersonaName::new("process")).await? {
        PersonaResponse::PersonaDetails(p) => {
            assert_eq!(cmd_json["data"]["name"], p.data.name.to_string());
        }
        other => panic!("expected PersonaDetails, got {other:?}"),
    }

    // ── Urges ───────────────────────────────────────────────────

    app.command(r#"urge set introspect --description "Look inward" --prompt "Pause and reflect""#)
        .await?;

    match client.urge().get(&UrgeName::new("introspect")).await? {
        UrgeResponse::UrgeDetails(u) => {
            assert_eq!(u.name, UrgeName::new("introspect"));
        }
        other => panic!("expected UrgeDetails, got {other:?}"),
    }

    // ── Set is idempotent ───────────────────────────────────────

    // Setting the same level again with different description should update, not conflict
    app.command(r#"level set working --description "Updated description" --prompt """#)
        .await?;

    match client.level().get(&LevelName::new("working")).await? {
        LevelResponse::LevelDetails(l) => {
            assert_eq!(l.data.description.to_string(), "Updated description");
        }
        other => panic!("expected LevelDetails, got {other:?}"),
    }

    Ok(())
}
