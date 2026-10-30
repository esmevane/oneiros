//! Query encoding workflow — free-text list parameters survive the trip
//! from client to server.
//!
//! List clients carry `query` and `lens` as URL query parameters. Those
//! values are prose and lens source, so they routinely contain `&`, `#`,
//! `+`, `=` and `%`. The fixtures below are chosen so that an unencoded
//! URL selects something different from what was asked for:
//!
//! - `#317` — unencoded, `#` truncates the URL, so the server sees an
//!   empty `query` and returns everything. This fails silently.
//! - `alpha+beta` — unencoded, `+` decodes to a space, turning one
//!   adjacent-phrase term ("alpha beta together") into two independent
//!   terms (which also match "alpha gamma beta apart").
//! - `alpha%20beta` — unencoded, `%20` decodes to a space; encoded, it
//!   tokenizes to the phrase "alpha 20beta".
//! - `&` in a lens — unencoded, it splits the lens into two parameters
//!   and the server rejects the query string.
//!
//! Every gov.process item has a bld.process or off-texture sibling that
//! only an over-broad selection would return. Results come back sorted.

use crate::tests::harness::TestApp;
use crate::*;

const FIXTURES: &[&str] = &[
    r#"agent create gov process --description "Governor agent""#,
    r#"agent create bld process --description "Builder agent""#,
    r##"cognition add gov.process observation "Tracking issue #317 here""##,
    r#"cognition add gov.process observation "alpha beta together""#,
    r#"cognition add gov.process observation "alpha gamma beta apart""#,
    r#"cognition add gov.process observation "alpha 20beta odd""#,
    r#"cognition add gov.process reflection "Reflecting elsewhere""#,
    r#"cognition add bld.process observation "Builder observes""#,
    r##"memory add gov.process project "Tracking issue #317 here""##,
    r#"memory add gov.process project "alpha beta together""#,
    r#"memory add gov.process project "alpha gamma beta apart""#,
    r#"memory add gov.process project "alpha 20beta odd""#,
    r#"memory add bld.process project "Builder remembers""#,
    r##"experience create gov.process echoes "Tracking issue #317 here""##,
    r#"experience create gov.process echoes "alpha beta together""#,
    r#"experience create gov.process echoes "alpha gamma beta apart""#,
    r#"experience create gov.process echoes "alpha 20beta odd""#,
    r#"experience create bld.process echoes "Builder thread""#,
];

// ── cognition ───────────────────────────────────────────────────────

#[tokio::test]
async fn cognition_list_lens_with_ampersand_intersects() -> Result<(), Box<dyn core::error::Error>>
{
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(FIXTURES).await?;

    let listed = app
        .testable()
        .cognitions(
            ListCognitions::builder_v1()
                .lens("texture(observation) & agent(gov.process)".to_string())
                .build()
                .into(),
        )
        .await?;

    assert_eq!(
        listed,
        [
            "Tracking issue #317 here",
            "alpha 20beta odd",
            "alpha beta together",
            "alpha gamma beta apart",
        ]
    );
    Ok(())
}

#[tokio::test]
async fn cognition_list_query_with_hash_filters() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(FIXTURES).await?;

    let listed = app
        .testable()
        .cognitions(
            ListCognitions::builder_v1()
                .query("#317".to_string())
                .build()
                .into(),
        )
        .await?;

    assert_eq!(listed, ["Tracking issue #317 here"]);
    Ok(())
}

#[tokio::test]
async fn cognition_list_query_with_hash_via_cli() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(FIXTURES).await?;

    let rendered = app
        .command(r##"cognition list --query "#317" --limit 1"##)
        .await?;
    let Responses::Cognition(CognitionResponse::Cognitions(CognitionsResponse::V1(listed))) =
        rendered.response()
    else {
        panic!("expected Cognitions, got {:#?}", rendered.response());
    };

    // Parameters after `query` must survive too — `--limit` included.
    assert_eq!(listed.total, 1);
    assert_eq!(listed.items.len(), 1);
    assert_eq!(
        listed.items[0].content.to_string(),
        "Tracking issue #317 here"
    );
    Ok(())
}

#[tokio::test]
async fn cognition_list_query_round_trips_reserved_characters()
-> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(FIXTURES).await?;

    let plus = app
        .testable()
        .cognitions(
            ListCognitions::builder_v1()
                .query("alpha+beta".to_string())
                .build()
                .into(),
        )
        .await?;
    assert_eq!(
        plus,
        ["alpha beta together"],
        "`+` must not decode to a space"
    );

    let percent = app
        .testable()
        .cognitions(
            ListCognitions::builder_v1()
                .query("alpha%20beta".to_string())
                .build()
                .into(),
        )
        .await?;
    assert_eq!(percent, ["alpha 20beta odd"], "`%20` must arrive literally");

    let equals = app
        .testable()
        .cognitions(
            ListCognitions::builder_v1()
                .lens(r#"search("alpha=beta") & texture(observation)"#.to_string())
                .build()
                .into(),
        )
        .await?;
    assert_eq!(
        equals,
        ["alpha beta together"],
        "`=` must survive in a lens"
    );

    Ok(())
}

// ── memory ──────────────────────────────────────────────────────────

#[tokio::test]
async fn memory_list_lens_with_ampersand_intersects() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(FIXTURES).await?;

    let listed = app
        .testable()
        .memories(
            ListMemories::builder_v1()
                .lens("level(project) & agent(gov.process)".to_string())
                .build()
                .into(),
        )
        .await?;

    assert_eq!(
        listed,
        [
            "Tracking issue #317 here",
            "alpha 20beta odd",
            "alpha beta together",
            "alpha gamma beta apart",
        ]
    );
    Ok(())
}

#[tokio::test]
async fn memory_list_query_with_hash_filters() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(FIXTURES).await?;

    let listed = app
        .testable()
        .memories(
            ListMemories::builder_v1()
                .query("#317".to_string())
                .build()
                .into(),
        )
        .await?;

    assert_eq!(listed, ["Tracking issue #317 here"]);
    Ok(())
}

#[tokio::test]
async fn memory_list_query_round_trips_reserved_characters()
-> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(FIXTURES).await?;

    let plus = app
        .testable()
        .memories(
            ListMemories::builder_v1()
                .query("alpha+beta".to_string())
                .build()
                .into(),
        )
        .await?;
    assert_eq!(plus, ["alpha beta together"]);

    let percent = app
        .testable()
        .memories(
            ListMemories::builder_v1()
                .query("alpha%20beta".to_string())
                .build()
                .into(),
        )
        .await?;
    assert_eq!(percent, ["alpha 20beta odd"]);

    Ok(())
}

// ── experience ──────────────────────────────────────────────────────

#[tokio::test]
async fn experience_list_lens_with_ampersand_intersects() -> Result<(), Box<dyn core::error::Error>>
{
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(FIXTURES).await?;

    let listed = app
        .testable()
        .experiences(
            ListExperiences::builder_v1()
                .lens("kind(experience) & agent(gov.process)".to_string())
                .build()
                .into(),
        )
        .await?;

    assert_eq!(
        listed,
        [
            "Tracking issue #317 here",
            "alpha 20beta odd",
            "alpha beta together",
            "alpha gamma beta apart",
        ]
    );
    Ok(())
}

#[tokio::test]
async fn experience_list_query_with_hash_filters() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(FIXTURES).await?;

    let listed = app
        .testable()
        .experiences(
            ListExperiences::builder_v1()
                .query("#317".to_string())
                .build()
                .into(),
        )
        .await?;

    assert_eq!(listed, ["Tracking issue #317 here"]);
    Ok(())
}

#[tokio::test]
async fn experience_list_query_round_trips_reserved_characters()
-> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(FIXTURES).await?;

    let plus = app
        .testable()
        .experiences(
            ListExperiences::builder_v1()
                .query("alpha+beta".to_string())
                .build()
                .into(),
        )
        .await?;
    assert_eq!(plus, ["alpha beta together"]);

    let percent = app
        .testable()
        .experiences(
            ListExperiences::builder_v1()
                .query("alpha%20beta".to_string())
                .build()
                .into(),
        )
        .await?;
    assert_eq!(percent, ["alpha 20beta odd"]);

    Ok(())
}

// ── agent ───────────────────────────────────────────────────────────

#[tokio::test]
async fn agent_list_lens_with_ampersand_intersects() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(FIXTURES).await?;

    let listed = app
        .testable()
        .agents(
            ListAgents::builder_v1()
                .lens(r#"kind(agent) & search("Governor")"#.to_string())
                .build()
                .into(),
        )
        .await?;

    assert_eq!(listed, ["gov.process"]);
    Ok(())
}

#[tokio::test]
async fn agent_list_query_with_hash_filters() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(FIXTURES).await?;
    app.command(r##"agent create trk process --description "Tracks issue #317""##)
        .await?;

    let listed = app
        .testable()
        .agents(
            ListAgents::builder_v1()
                .query("#317".to_string())
                .build()
                .into(),
        )
        .await?;

    assert_eq!(listed, ["trk.process"]);
    Ok(())
}

#[tokio::test]
async fn agent_list_query_round_trips_reserved_characters()
-> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(FIXTURES).await?;
    app.commands(&[
        r#"agent create adj process --description "alpha beta together""#,
        r#"agent create spl process --description "alpha gamma beta apart""#,
        r#"agent create esc process --description "alpha 20beta odd""#,
    ])
    .await?;

    let plus = app
        .testable()
        .agents(
            ListAgents::builder_v1()
                .query("alpha+beta".to_string())
                .build()
                .into(),
        )
        .await?;
    assert_eq!(plus, ["adj.process"]);

    let percent = app
        .testable()
        .agents(
            ListAgents::builder_v1()
                .query("alpha%20beta".to_string())
                .build()
                .into(),
        )
        .await?;
    assert_eq!(percent, ["esc.process"]);

    Ok(())
}

// ── connection ──────────────────────────────────────────────────────

#[tokio::test]
async fn connection_list_lens_with_ampersand_intersects() -> Result<(), Box<dyn core::error::Error>>
{
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;
    app.commands(FIXTURES).await?;
    app.command(r#"nature set caused --description "A produced B""#)
        .await?;

    let mut refs = Vec::new();
    for label in ["a", "b", "c"] {
        let rendered = app
            .command(&format!(
                r#"cognition add gov.process working "node {label}""#
            ))
            .await?;
        let Responses::Cognition(CognitionResponse::CognitionAdded(CognitionAddedResponse::V1(
            added,
        ))) = rendered.response()
        else {
            panic!("expected CognitionAdded, got {:#?}", rendered.response());
        };
        refs.push(RefToken::new(Ref::cognition(added.cognition.id)));
    }
    for window in refs.windows(2) {
        app.command(&format!(
            "connection create caused {} {}",
            window[0], window[1]
        ))
        .await?;
    }

    let listed = match app
        .client()
        .connection()
        .list(
            &ListConnections::builder_v1()
                .lens("kind(connection) & kind(connection)".to_string())
                .build()
                .into(),
        )
        .await?
    {
        ConnectionResponse::Connections(ConnectionsResponse::V1(listed)) => listed.items,
        ConnectionResponse::NoConnections => Vec::new(),
        other => panic!("expected Connections, got {other:#?}"),
    };

    assert_eq!(listed.len(), 2, "both connections survive the intersection");
    Ok(())
}
