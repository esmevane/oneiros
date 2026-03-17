use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use http_body_util::BodyExt;
use oneiros_db::Database;
use oneiros_model::*;
use oneiros_resource::Fulfill;
use tower::ServiceExt;

// crate re-exports AgentResource/LevelResource to avoid collision with oneiros_model types.
use crate::{AgentResource, LevelResource, ProjectScope, ProjectScopeError, ServiceState};

// ── Test helpers ───────────────────────────────────────────────────

/// Create a brain database in a temp directory for testing.
fn test_db(dir: &std::path::Path) -> Database {
    let db_path = dir.join("test-brain.db");
    Database::create_brain_db(&db_path).expect("create brain db")
}

/// Seed a persona so agent creation can pass FK validation.
fn seed_persona(db: &Database) {
    db.set_persona(
        &PersonaName::new("test-persona"),
        &Description::new("A test persona"),
        &Prompt::new("You are a test."),
    )
    .expect("seed persona");
}

fn test_source() -> Source {
    Source::default()
}

const ALL_PROJECTIONS: &[&[oneiros_db::Projection]] =
    &[crate::projections::AGENT, crate::projections::LEVEL];

fn project_scope(db: &Database) -> ProjectScope<'_> {
    ProjectScope::new(db, test_source(), ALL_PROJECTIONS)
}

fn test_service_state(db: Database) -> ServiceState {
    ServiceState::new(db, test_source(), ALL_PROJECTIONS)
}

/// Build the composed app router — multiple resources, one router.
fn test_app(state: ServiceState) -> Router {
    Router::new()
        .nest("/agents", AgentResource::http_router())
        .nest("/levels", LevelResource::http_router())
        .with_state(state)
}

/// Parse a JSON response body.
async fn json_body<T: serde::de::DeserializeOwned>(response: axum::http::Response<Body>) -> T {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

/// Convenience: disambiguated fulfill for Agent requests.
async fn fulfill_agent(
    scope: &ProjectScope<'_>,
    request: AgentRequests,
) -> Result<AgentResponses, ProjectScopeError> {
    Fulfill::<AgentResource>::fulfill(scope, request).await
}

/// Convenience: disambiguated fulfill for Level requests.
async fn fulfill_level(
    scope: &ProjectScope<'_>,
    request: LevelRequests,
) -> Result<LevelResponses, ProjectScopeError> {
    Fulfill::<LevelResource>::fulfill(scope, request).await
}

// ── Fulfill (domain logic) tests ──────────────────────────────────

#[tokio::test]
async fn create_agent_through_fulfill() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let scope = project_scope(&db);

    let request = AgentRequests::CreateAgent(CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::new("The governor agent"),
        prompt: Prompt::new("You govern."),
    });

    let response = fulfill_agent(&scope, request).await.expect("create agent");

    match response {
        AgentResponses::AgentCreated(agent) => {
            assert_eq!(agent.name, AgentName::new("governor"));
            assert_eq!(agent.persona, PersonaName::new("test-persona"));
        }
        other => panic!("Expected AgentCreated, got {other:?}"),
    }
}

#[tokio::test]
async fn list_agents_through_fulfill() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);
    let scope = project_scope(&db);

    fulfill_agent(&scope, AgentRequests::CreateAgent(CreateAgentRequest {
        name: AgentName::new("alice"),
        persona: PersonaName::new("test-persona"),
        description: Description::default(),
        prompt: Prompt::default(),
    })).await.unwrap();

    fulfill_agent(&scope, AgentRequests::CreateAgent(CreateAgentRequest {
        name: AgentName::new("bob"),
        persona: PersonaName::new("test-persona"),
        description: Description::default(),
        prompt: Prompt::default(),
    })).await.unwrap();

    let response = fulfill_agent(&scope, AgentRequests::ListAgents(ListAgentsRequest))
        .await.expect("list agents");

    match response {
        AgentResponses::AgentsListed(agents) => {
            assert_eq!(agents.len(), 2);
            let names: Vec<_> = agents.iter().map(|a| a.name.as_ref()).collect();
            assert!(names.contains(&"alice"));
            assert!(names.contains(&"bob"));
        }
        other => panic!("Expected AgentsListed, got {other:?}"),
    }
}

#[tokio::test]
async fn get_agent_through_fulfill() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);
    let scope = project_scope(&db);

    fulfill_agent(&scope, AgentRequests::CreateAgent(CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::new("Gov"),
        prompt: Prompt::default(),
    })).await.unwrap();

    let response = fulfill_agent(&scope, AgentRequests::GetAgent(GetAgentRequest {
        name: AgentName::new("governor"),
    })).await.expect("get agent");

    match response {
        AgentResponses::AgentFound(agent) => {
            assert_eq!(agent.name, AgentName::new("governor"));
            assert_eq!(agent.description, Description::new("Gov"));
        }
        other => panic!("Expected AgentFound, got {other:?}"),
    }
}

#[tokio::test]
async fn get_agent_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    let scope = project_scope(&db);

    let result = fulfill_agent(&scope, AgentRequests::GetAgent(GetAgentRequest {
        name: AgentName::new("nonexistent"),
    })).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, ProjectScopeError::NotFound(NotFound::Agent(_))),
        "Expected NotFound::Agent, got {err:?}"
    );
}

#[tokio::test]
async fn update_agent_through_fulfill() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);
    let scope = project_scope(&db);

    fulfill_agent(&scope, AgentRequests::CreateAgent(CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::new("Original"),
        prompt: Prompt::default(),
    })).await.unwrap();

    let response = fulfill_agent(&scope, AgentRequests::UpdateAgent(UpdateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::new("Updated"),
        prompt: Prompt::new("New prompt"),
    })).await.expect("update agent");

    match response {
        AgentResponses::AgentUpdated(agent) => {
            assert_eq!(agent.description, Description::new("Updated"));
            assert_eq!(agent.prompt, Prompt::new("New prompt"));
        }
        other => panic!("Expected AgentUpdated, got {other:?}"),
    }
}

#[tokio::test]
async fn remove_agent_through_fulfill() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);
    let scope = project_scope(&db);

    fulfill_agent(&scope, AgentRequests::CreateAgent(CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::default(),
        prompt: Prompt::default(),
    })).await.unwrap();

    let response = fulfill_agent(&scope, AgentRequests::RemoveAgent(RemoveAgentRequest {
        name: AgentName::new("governor"),
    })).await.expect("remove agent");

    assert!(matches!(response, AgentResponses::AgentRemoved));

    // Verify it's gone
    let result = fulfill_agent(&scope, AgentRequests::GetAgent(GetAgentRequest {
        name: AgentName::new("governor"),
    })).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn create_agent_conflict() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);
    let scope = project_scope(&db);

    fulfill_agent(&scope, AgentRequests::CreateAgent(CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::default(),
        prompt: Prompt::default(),
    })).await.unwrap();

    let result = fulfill_agent(&scope, AgentRequests::CreateAgent(CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::default(),
        prompt: Prompt::default(),
    })).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, ProjectScopeError::Conflict(_)),
        "Expected Conflict, got {err:?}"
    );
}

#[tokio::test]
async fn create_agent_missing_persona() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    // Deliberately NOT seeding persona
    let scope = project_scope(&db);

    let result = fulfill_agent(&scope, AgentRequests::CreateAgent(CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("nonexistent-persona"),
        description: Description::default(),
        prompt: Prompt::default(),
    })).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, ProjectScopeError::NotFound(NotFound::Persona(_))),
        "Expected NotFound::Persona, got {err:?}"
    );
}

// ── HTTP integration tests ────────────────────────────────────────

#[tokio::test]
async fn http_create_agent() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let app = test_app(test_service_state(db));

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/agents")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&CreateAgentRequest {
                        name: AgentName::new("governor"),
                        persona: PersonaName::new("test-persona"),
                        description: Description::new("The governor"),
                        prompt: Prompt::new("You govern."),
                    })
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body: AgentResponses = json_body(response).await;
    match body {
        AgentResponses::AgentCreated(agent) => {
            assert_eq!(agent.name, AgentName::new("governor"));
        }
        other => panic!("Expected AgentCreated, got {other:?}"),
    }
}

#[tokio::test]
async fn http_list_agents() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let state = test_service_state(db);
    // Seed an agent through the service state directly
    state
        .fulfill::<AgentResource>(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("alice"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .unwrap();

    let app = test_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/agents")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: AgentResponses = json_body(response).await;
    match body {
        AgentResponses::AgentsListed(agents) => {
            assert_eq!(agents.len(), 1);
            assert_eq!(agents[0].name, AgentName::new("alice"));
        }
        other => panic!("Expected AgentsListed, got {other:?}"),
    }
}

#[tokio::test]
async fn http_get_agent() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let state = test_service_state(db);
    state
        .fulfill::<AgentResource>(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Gov"),
            prompt: Prompt::default(),
        }))
        .unwrap();

    let app = test_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/agents/governor")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: AgentResponses = json_body(response).await;
    match body {
        AgentResponses::AgentFound(agent) => {
            assert_eq!(agent.name, AgentName::new("governor"));
            assert_eq!(agent.description, Description::new("Gov"));
        }
        other => panic!("Expected AgentFound, got {other:?}"),
    }
}

#[tokio::test]
async fn http_get_agent_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());

    let app = test_app(test_service_state(db));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/agents/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn http_update_agent() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let state = test_service_state(db);
    state
        .fulfill::<AgentResource>(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Original"),
            prompt: Prompt::default(),
        }))
        .unwrap();

    let app = test_app(state);

    let request: Request<Body> = Request::builder()
        .method("PUT")
        .uri("/agents/governor")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&UpdateAgentRequest {
                name: AgentName::new("ignored"),
                persona: PersonaName::new("test-persona"),
                description: Description::new("Updated"),
                prompt: Prompt::new("New prompt"),
            })
            .unwrap(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: AgentResponses = json_body(response).await;
    match body {
        AgentResponses::AgentUpdated(agent) => {
            assert_eq!(agent.description, Description::new("Updated"));
            // Name comes from path, not body
            assert_eq!(agent.name, AgentName::new("governor"));
        }
        other => panic!("Expected AgentUpdated, got {other:?}"),
    }
}

#[tokio::test]
async fn http_remove_agent() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let state = test_service_state(db);
    state
        .fulfill::<AgentResource>(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .unwrap();

    let app = test_app(state.clone());

    let request: Request<Body> = Request::builder()
        .method("DELETE")
        .uri("/agents/governor")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Verify it's gone via a second request
    let app = test_app(state);

    let request: Request<Body> = Request::builder()
        .uri("/agents/governor")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ── Level Fulfill tests ───────────────────────────────────────────

#[tokio::test]
async fn set_level_through_fulfill() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());

    let scope = project_scope(&db);

    let level = oneiros_model::Level::init("working", "Active work", "Short-term");

    let response = Fulfill::<LevelResource>::fulfill(&scope, LevelRequests::SetLevel(level))
        .await
        .expect("set level");

    match response {
        LevelResponses::LevelSet(level) => {
            assert_eq!(level.name, LevelName::new("working"));
        }
        other => panic!("Expected LevelSet, got {other:?}"),
    }
}

#[tokio::test]
async fn list_levels_through_fulfill() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());

    let scope = project_scope(&db);

    Fulfill::<LevelResource>::fulfill(
        &scope,
        LevelRequests::SetLevel(oneiros_model::Level::init("working", "Active", "Short-term")),
    )
    .await
    .unwrap();

    Fulfill::<LevelResource>::fulfill(
        &scope,
        LevelRequests::SetLevel(oneiros_model::Level::init("archival", "Long-term", "Permanent")),
    )
    .await
    .unwrap();

    let response =
        Fulfill::<LevelResource>::fulfill(&scope, LevelRequests::ListLevels(ListLevelsRequest))
            .await
            .expect("list levels");

    match response {
        LevelResponses::LevelsListed(levels) => {
            assert_eq!(levels.len(), 2);
        }
        other => panic!("Expected LevelsListed, got {other:?}"),
    }
}

#[tokio::test]
async fn get_level_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());

    let scope = project_scope(&db);

    let result = Fulfill::<LevelResource>::fulfill(
        &scope,
        LevelRequests::GetLevel(GetLevelRequest {
            name: LevelName::new("nonexistent"),
        }),
    )
    .await;

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ProjectScopeError::NotFound(NotFound::Level(_))
    ));
}

// ── Level HTTP tests ──────────────────────────────────────────────

#[tokio::test]
async fn http_set_level() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());

    let app = test_app(test_service_state(db));

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/levels/working")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&oneiros_model::Level::init(
                        "ignored",
                        "Active work",
                        "Short-term context",
                    ))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: LevelResponses = json_body(response).await;
    match body {
        LevelResponses::LevelSet(level) => {
            // Name comes from path, not body
            assert_eq!(level.name, LevelName::new("working"));
        }
        other => panic!("Expected LevelSet, got {other:?}"),
    }
}

#[tokio::test]
async fn http_list_levels() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());

    let state = test_service_state(db);
    state
        .fulfill::<LevelResource>(LevelRequests::SetLevel(oneiros_model::Level::init(
            "working",
            "Active",
            "Short-term",
        )))
        .unwrap();

    let app = test_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/levels")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: LevelResponses = json_body(response).await;
    match body {
        LevelResponses::LevelsListed(levels) => {
            assert_eq!(levels.len(), 1);
            assert_eq!(levels[0].name, LevelName::new("working"));
        }
        other => panic!("Expected LevelsListed, got {other:?}"),
    }
}

#[tokio::test]
async fn http_get_level() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());

    let state = test_service_state(db);
    state
        .fulfill::<LevelResource>(LevelRequests::SetLevel(oneiros_model::Level::init(
            "working",
            "Active work",
            "Short-term",
        )))
        .unwrap();

    let app = test_app(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/levels/working")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: LevelResponses = json_body(response).await;
    match body {
        LevelResponses::LevelFound(level) => {
            assert_eq!(level.name, LevelName::new("working"));
            assert_eq!(level.description, Description::new("Active work"));
        }
        other => panic!("Expected LevelFound, got {other:?}"),
    }
}

#[tokio::test]
async fn http_remove_level() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());

    let state = test_service_state(db);
    state
        .fulfill::<LevelResource>(LevelRequests::SetLevel(oneiros_model::Level::init(
            "working",
            "Active",
            "Short-term",
        )))
        .unwrap();

    let app = test_app(state.clone());

    let request: Request<Body> = Request::builder()
        .method("DELETE")
        .uri("/levels/working")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify it's gone
    let app = test_app(state);

    let request: Request<Body> = Request::builder()
        .uri("/levels/working")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ── Composed router test ──────────────────────────────────────────
//
// The real proof: both resources coexist in one router,
// each owning their own routes, sharing one ServiceState.

#[tokio::test]
async fn composed_router_serves_both_resources() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let state = test_service_state(db);

    // Create an agent
    state
        .fulfill::<AgentResource>(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Gov"),
            prompt: Prompt::default(),
        }))
        .unwrap();

    // Set a level
    state
        .fulfill::<LevelResource>(LevelRequests::SetLevel(oneiros_model::Level::init(
            "working",
            "Active",
            "Short-term",
        )))
        .unwrap();

    let app = test_app(state);

    // Hit the agent endpoint
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/agents/governor")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: AgentResponses = json_body(response).await;
    assert!(matches!(body, AgentResponses::AgentFound(_)));

    // Hit the level endpoint — same app, same state
    let response = app
        .oneshot(
            Request::builder()
                .uri("/levels/working")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: LevelResponses = json_body(response).await;
    assert!(matches!(body, LevelResponses::LevelFound(_)));
}
