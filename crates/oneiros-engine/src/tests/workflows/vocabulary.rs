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

    app.command(r#"level set working --description "Active processing" --prompt "What you're actively processing""#)
        .await?;
    app.command(r#"level set session --description "Session context" --prompt "Current session""#)
        .await?;

    match client
        .level()
        .list(&ListLevels::builder_v1().build().into())
        .await?
    {
        LevelResponse::Levels(LevelsResponse::V1(levels)) => assert_eq!(levels.items.len(), 2),
        other => panic!("expected Levels, got {other:?}"),
    }

    match client
        .level()
        .get(
            &GetLevel::builder_v1()
                .key(LevelName::new("working"))
                .build()
                .into(),
        )
        .await?
    {
        LevelResponse::LevelDetails(LevelDetailsResponse::V1(l)) => {
            assert_eq!(l.level.description.to_string(), "Active processing");
        }
        other => panic!("expected LevelDetails, got {other:?}"),
    }

    // Remove and verify
    app.command("level remove session").await?;
    assert!(
        client
            .level()
            .get(
                &GetLevel::builder_v1()
                    .key(LevelName::new("session"))
                    .build()
                    .into()
            )
            .await
            .is_err()
    );

    match client
        .level()
        .list(&ListLevels::builder_v1().build().into())
        .await?
    {
        LevelResponse::Levels(LevelsResponse::V1(levels)) => assert_eq!(levels.items.len(), 1),
        other => panic!("expected Levels, got {other:?}"),
    }

    app.command(r#"texture set observation --description "Noticing" --prompt "When you notice""#)
        .await?;

    match client
        .texture()
        .get(
            &GetTexture::builder_v1()
                .key(TextureName::new("observation"))
                .build()
                .into(),
        )
        .await?
    {
        TextureResponse::TextureDetails(TextureDetailsResponse::V1(t)) => {
            assert_eq!(t.texture.name, TextureName::new("observation"));
        }
        other => panic!("expected TextureDetails, got {other:?}"),
    }

    app.command(r#"sensation set echoes --description "Resonance" --prompt "Things that rhyme""#)
        .await?;

    match client
        .sensation()
        .get(
            &GetSensation::builder_v1()
                .key(SensationName::new("echoes"))
                .build()
                .into(),
        )
        .await?
    {
        SensationResponse::SensationDetails(SensationDetailsResponse::V1(s)) => {
            assert_eq!(s.sensation.name, SensationName::new("echoes"));
        }
        other => panic!("expected SensationDetails, got {other:?}"),
    }

    app.command(r#"nature set reference --description "Related" --prompt "Cross-reference""#)
        .await?;

    match client
        .nature()
        .get(
            &GetNature::builder_v1()
                .key(NatureName::new("reference"))
                .build()
                .into(),
        )
        .await?
    {
        NatureResponse::NatureDetails(NatureDetailsResponse::V1(n)) => {
            assert_eq!(n.nature.name, NatureName::new("reference"));
        }
        other => panic!("expected NatureDetails, got {other:?}"),
    }

    app.command(
        r#"persona set process --description "Process agents" --prompt "You manage process""#,
    )
    .await?;

    // Command and client agree
    let cmd_result = app.command("persona show process").await?;
    let cmd_json = serde_json::to_value(cmd_result.response())?;

    match client
        .persona()
        .get(
            &GetPersona::builder_v1()
                .key(PersonaName::new("process"))
                .build()
                .into(),
        )
        .await?
    {
        PersonaResponse::PersonaDetails(PersonaDetailsResponse::V1(p)) => {
            assert_eq!(cmd_json["data"]["name"], p.persona.name.to_string());
        }
        other => panic!("expected PersonaDetails, got {other:?}"),
    }

    app.command(r#"urge set introspect --description "Look inward" --prompt "Pause and reflect""#)
        .await?;

    match client
        .urge()
        .get(
            &GetUrge::builder_v1()
                .key(UrgeName::new("introspect"))
                .build()
                .into(),
        )
        .await?
    {
        UrgeResponse::UrgeDetails(UrgeDetailsResponse::V1(u)) => {
            assert_eq!(u.urge.name, UrgeName::new("introspect"));
        }
        other => panic!("expected UrgeDetails, got {other:?}"),
    }

    // Setting the same level again with different description should update, not conflict
    app.command(r#"level set working --description "Updated description" --prompt """#)
        .await?;

    match client
        .level()
        .get(
            &GetLevel::builder_v1()
                .key(LevelName::new("working"))
                .build()
                .into(),
        )
        .await?
    {
        LevelResponse::LevelDetails(LevelDetailsResponse::V1(l)) => {
            assert_eq!(l.level.description.to_string(), "Updated description");
        }
        other => panic!("expected LevelDetails, got {other:?}"),
    }

    Ok(())
}
