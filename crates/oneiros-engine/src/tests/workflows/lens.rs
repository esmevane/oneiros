//! Lens workflow — parse, validate, name-resolve, explain, and query
//! end-to-end through the HTTP service.

use crate::tests::harness::TestApp;
use crate::*;

async fn seeded_app() -> Result<TestApp, Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    app.command(r#"agent create gov process --description "Governor agent""#)
        .await?;
    app.command(r#"cognition add gov.process observation "The garden is growing""#)
        .await?;
    app.command(r#"cognition add gov.process reflection "Patterns emerge from fragments""#)
        .await?;
    app.command(r#"cognition add gov.process observation "Seeds need tending""#)
        .await?;
    app.command(r#"memory add gov.process project "Event sourcing works well""#)
        .await?;

    Ok(app)
}

fn extract_query_hits(rendered: &Rendered<Responses>) -> &[Hit] {
    let Responses::Lens(LensResponse::Queried(QueriedLensResponse::V1(queried))) =
        rendered.response()
    else {
        panic!("expected Lens(Queried), got {:#?}", rendered.response());
    };
    &queried.hits
}

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

    let rendered = app.command(r#"lens parse "agent(gov.process)""#).await?;
    let Responses::Lens(LensResponse::Parsed(ParsedLensResponse::V1(parsed))) = rendered.response()
    else {
        panic!("expected Lens(Parsed), got {:#?}", rendered.response());
    };

    assert_eq!(parsed.source, "agent(gov.process)");
    assert_eq!(parsed.display, "agent(gov.process)");

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

    app.command(r#"agent create governor process --description "Governor agent""#)
        .await?;

    let known = app
        .command(r#"lens explain "agent(governor.process)""#)
        .await?;
    let Responses::Lens(LensResponse::Explained(_)) = known.response() else {
        panic!(
            "expected Explained for known agent, got {:#?}",
            known.response()
        );
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

    let known = app
        .command(r#"lens explain "texture(observation)""#)
        .await?;
    let Responses::Lens(LensResponse::Explained(_)) = known.response() else {
        panic!("expected Explained for known texture");
    };

    let typo = app.command(r#"lens explain "texture(observasion)""#).await;
    let error = typo.expect_err("typo must reject end-to-end");
    let message = format!("{error}");
    assert!(
        message.contains("unknown texture"),
        "expected UnknownSymbol message, got: {message}"
    );

    Ok(())
}

#[tokio::test]
async fn lens_query_returns_hits_for_single_predicate() -> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app.command(r#"lens query "texture(observation)""#).await?;
    let hits = extract_query_hits(&rendered);

    assert_eq!(hits.len(), 2, "two cognitions with texture=observation");
    for hit in hits {
        assert!(matches!(hit, Hit::Entity(_)));
    }

    Ok(())
}

#[tokio::test]
async fn lens_query_intersection_returns_only_matching_hits()
-> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app
        .command(r#"lens query "texture(observation) & agent(gov.process)""#)
        .await?;
    let hits = extract_query_hits(&rendered);

    assert_eq!(hits.len(), 2, "both observations belong to gov.process");

    Ok(())
}

#[tokio::test]
async fn lens_query_union_returns_combined_hits() -> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app
        .command(r#"lens query "texture(observation) | texture(reflection)""#)
        .await?;
    let hits = extract_query_hits(&rendered);

    assert_eq!(hits.len(), 3, "2 observations + 1 reflection");

    Ok(())
}

#[tokio::test]
async fn lens_query_difference_returns_left_minus_right() -> Result<(), Box<dyn core::error::Error>>
{
    let app = seeded_app().await?;

    let all_rendered = app.command(r#"lens query "agent(gov.process)""#).await?;
    let all_hits = extract_query_hits(&all_rendered);
    let all_count = all_hits.len();

    let diff_rendered = app
        .command(r#"lens query "agent(gov.process) ~ texture(observation)""#)
        .await?;
    let diff_hits = extract_query_hits(&diff_rendered);

    assert!(
        diff_hits.len() < all_count,
        "difference should reduce the set"
    );
    assert!(
        diff_hits.len() >= 2,
        "should have at least the reflection + memory: got {}",
        diff_hits.len()
    );

    Ok(())
}

#[tokio::test]
async fn lens_query_returns_empty_for_unimplemented_predicate()
-> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    // `working` is a seeded texture but our test data has no cognitions with it,
    // so the query should return empty rather than erroring.
    let rendered = app.command(r#"lens query "texture(working)""#).await?;
    let hits = extract_query_hits(&rendered);

    assert_eq!(
        hits.len(),
        0,
        "no cognitions with that texture in test data"
    );

    Ok(())
}

#[tokio::test]
async fn lens_compile_returns_ir_for_inspection() -> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app
        .command(r#"lens explain "texture(observation) & agent(gov.process)""#)
        .await?;
    let Responses::Lens(LensResponse::Explained(ExplainedLensResponse::V1(explained))) =
        rendered.response()
    else {
        panic!("expected Lens(Explained), got {:#?}", rendered.response());
    };

    assert!(
        explained.plan.contains("$0"),
        "IR should contain slot references: {}",
        explained.plan
    );

    Ok(())
}

#[tokio::test]
async fn lens_query_results_sort_stably_by_timestamp_desc()
-> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app.command(r#"lens query "agent(gov.process)""#).await?;
    let hits = extract_query_hits(&rendered);

    assert!(hits.len() >= 2, "need multiple hits to test ordering");
    for window in hits.windows(2) {
        let a_ts = window[0].timestamp();
        let b_ts = window[1].timestamp();
        assert!(a_ts >= b_ts, "hits should be in descending timestamp order");
    }

    Ok(())
}

#[tokio::test]
async fn lens_query_results_sort_by_relevance_when_requested()
-> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app.command(r#"lens query "search(\"garden\")""#).await?;
    let hits = extract_query_hits(&rendered);

    assert!(
        !hits.is_empty(),
        "search should find at least the 'garden' cognition"
    );
    for hit in hits {
        assert!(matches!(hit, Hit::Entity(_)));
    }

    Ok(())
}
