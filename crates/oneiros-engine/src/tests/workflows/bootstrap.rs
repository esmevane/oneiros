//! Bootstrap workflow — from nothing to a functioning brain.
//!
//! This is the "hello world" of oneiros: initialize a system, create a
//! project, seed the vocabulary, emerge an agent, and dream them into
//! existence. If this works, the core lifecycle is sound.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn from_nothing_to_a_dreaming_agent() -> Result<(), Box<dyn core::error::Error>> {
    // Start fresh — initialize the system, create a project, seed the vocabulary
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    // The vocabulary is there — we can query it
    let client = app.client();
    match client
        .level()
        .list(&ListLevels::builder_v1().build().into())
        .await?
    {
        LevelResponse::Levels(LevelsResponse::V1(levels)) => {
            assert!(
                levels.items.len() >= 4,
                "seed should create at least 4 levels"
            );
        }
        other => panic!("expected Levels, got {other:?}"),
    }

    match client
        .persona()
        .list(&ListPersonas::builder_v1().build().into())
        .await?
    {
        PersonaResponse::Personas(PersonasResponse::V1(personas)) => {
            assert!(
                !personas.items.is_empty(),
                "seed should create at least one persona"
            );
        }
        other => panic!("expected Personas, got {other:?}"),
    }

    // Emerge an agent — this creates the agent and wakes them
    app.command("emerge thinker process").await?;

    // The agent exists
    match client
        .agent()
        .get(&GetAgent::V1(
            GetAgentV1::builder()
                .key(AgentName::new("thinker.process"))
                .build(),
        ))
        .await?
    {
        AgentResponse::AgentDetails(AgentDetailsResponse::V1(agent)) => {
            assert_eq!(agent.agent.name, AgentName::new("thinker.process"));
            assert_eq!(agent.agent.persona, PersonaName::new("process"));
        }
        other => panic!("expected AgentDetails, got {other:?}"),
    }

    // Dream them — assembles identity, vocabulary, and instructions
    let result = app.command("dream thinker.process").await?;
    let rendered = serde_json::to_value(result.response())?;
    let dream_text = rendered.to_string();

    assert!(
        dream_text.contains("thinker.process"),
        "dream should name the agent"
    );
    assert!(
        dream_text.contains("process"),
        "dream should reference their persona"
    );

    Ok(())
}

/// `system init` claims the host. Once a tenant exists, a second call must
/// surface the conflict via `HostAlreadyInitialized` rather than silently
/// re-emitting events. The route is unauthenticated by design — this is the
/// invariant that protects it from being usable as a takeover vector.
#[tokio::test]
async fn system_init_twice_returns_already_initialized() -> Result<(), Box<dyn core::error::Error>>
{
    let app = TestApp::new().await?;

    let host_client = Client::new(app.base_url());
    let system = SystemClient::new(&host_client);
    let request: InitSystem = InitSystem::builder_v1().name("test").build().into();

    match system.init(&request).await? {
        SystemResponse::SystemInitialized(_) => {}
        other => panic!("first init should succeed, got {other:?}"),
    }

    match system.init(&request).await? {
        SystemResponse::HostAlreadyInitialized => {}
        other => panic!("second init should refuse, got {other:?}"),
    }

    Ok(())
}

/// Setup orchestrates the full bootstrap through HTTP. With the server
/// already running, setup discovers it (no install prompt), drives the
/// init / seed sequence over HTTP, and the token issued by project init
/// authenticates the subsequent seed calls. The MCP step prompts in an
/// interactive shell; in a non-interactive test it falls through to
/// McpSkipped, leaving no `.mcp.json` side-effect.
#[tokio::test]
async fn setup_drives_bootstrap_through_http() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new().await?;

    let result = app.command("setup").await?;
    let response = match result.response() {
        Responses::Setup(setup) => setup.clone(),
        other => panic!("expected Setup response, got {other:?}"),
    };
    let SetupResponse::SetupComplete(SetupCompleteResponse::V1(complete)) = response;
    let steps = complete.steps;

    // Server was already running — no install/start step should appear.
    for step in &steps {
        match step {
            SetupStep::ServiceInstalled | SetupStep::ServiceStarted => panic!(
                "setup should not have installed/started the service when one was already running, steps: {steps:?}"
            ),
            _ => {}
        }
    }

    // Bootstrap chain must complete via HTTP.
    assert!(
        steps
            .iter()
            .any(|s| matches!(s, SetupStep::SystemInitialized)),
        "setup should report SystemInitialized, steps: {steps:?}"
    );
    assert!(
        steps
            .iter()
            .any(|s| matches!(s, SetupStep::ProjectInitialized(_))),
        "setup should report ProjectInitialized, steps: {steps:?}"
    );
    // Seed steps prove the token handoff worked: they require a valid Bearer
    // token, and the token is sourced from the ProjectInitialized response.
    assert!(
        steps
            .iter()
            .any(|s| matches!(s, SetupStep::VocabularySeeded)),
        "seed core must succeed via HTTP, proving token handoff, steps: {steps:?}"
    );
    assert!(
        steps.iter().any(|s| matches!(s, SetupStep::AgentsSeeded)),
        "seed agents must succeed via HTTP, proving token handoff, steps: {steps:?}"
    );

    // No StepFailed entries — the orchestration is end-to-end clean.
    for step in &steps {
        if let SetupStep::StepFailed { step, reason } = step {
            panic!("setup reported StepFailed at {step}: {reason}");
        }
    }

    // The bootstrap left a usable project: an authenticated client should
    // see the seeded vocabulary.
    let client = app.client();
    match client
        .level()
        .list(&ListLevels::builder_v1().build().into())
        .await?
    {
        LevelResponse::Levels(LevelsResponse::V1(levels)) => {
            assert!(
                levels.items.len() >= 4,
                "post-setup brain should have the seeded levels"
            );
        }
        other => panic!("expected Levels post-setup, got {other:?}"),
    }

    Ok(())
}

/// Project init should produce a "main" bookmark in the system DB.
///
/// Nothing implicit: if a resource exists, an event brought it into being
/// and a projection row reflects it. The default bookmark is no exception.
#[tokio::test]
async fn project_init_creates_main_bookmark() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let client = app.client();

    match client
        .bookmark()
        .list(&ListBookmarks::builder_v1().build().into())
        .await?
    {
        BookmarkResponse::Bookmarks(bookmarks) => {
            assert_eq!(bookmarks.len(), 1, "exactly one bookmark after init");
            assert_eq!(
                bookmarks.items[0].name,
                BookmarkName::main(),
                "the bookmark should be named main"
            );
        }
        other => panic!("expected Bookmarks, got {other:?}"),
    }

    Ok(())
}
