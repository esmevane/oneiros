//! Lens workflow — parse, validate, name-resolve, and explain queries
//! end-to-end through the HTTP service.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn lens_parse_returns_round_trip_display() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let rendered = app
        .command(r#"lens parse "agent(governor.process)""#)
        .await?;
    let Responses::Lens(LensResponse::Parsed(ParsedLensResponse::V1(parsed))) = rendered.response()
    else {
        panic!("expected Lens(Parsed), got {:#?}", rendered.response());
    };

    assert_eq!(parsed.source, "agent(governor.process)");
    assert_eq!(parsed.display, "agent(governor.process)");

    Ok(())
}

#[tokio::test]
async fn lens_explain_rejects_unknown_agent_name() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    // Seed an agent so the *known* name validates; the typo is then the
    // only thing that should differ between accept and reject paths.
    app.command(r#"agent create governor process --description "Governor agent""#)
        .await?;

    let known = app
        .command(r#"lens explain "agent(governor.process)""#)
        .await?;
    let Responses::Lens(LensResponse::Explained(_)) = known.response() else {
        panic!("expected Explained for known agent, got {:#?}", known.response());
    };

    let typo = app
        .command(r#"lens explain "agent(governorr.process)""#)
        .await;
    let error = typo.expect_err("typo must reject end-to-end");
    let message = format!("{error}");
    assert!(
        message.contains("unknown agent"),
        "expected UnknownSymbol message, got: {message}"
    );
    assert!(
        message.contains("governorr.process"),
        "expected the bad name in the message, got: {message}"
    );

    Ok(())
}

#[tokio::test]
async fn lens_explain_rejects_unknown_texture_name() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    // `observation` ships in the seeded vocabulary; the typo should fail.
    let known = app
        .command(r#"lens explain "texture(observation)""#)
        .await?;
    let Responses::Lens(LensResponse::Explained(_)) = known.response() else {
        panic!("expected Explained for known texture");
    };

    let typo = app
        .command(r#"lens explain "texture(observasion)""#)
        .await;
    let error = typo.expect_err("typo must reject end-to-end");
    let message = format!("{error}");
    assert!(
        message.contains("unknown texture"),
        "expected UnknownSymbol message, got: {message}"
    );

    Ok(())
}
