use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use oneiros_db::Database;
use oneiros_model::*;
use oneiros_resource::Fulfill;
use tower::ServiceExt;

// crate re-exports AgentResource/LevelResource to avoid collision with oneiros_model types.
use crate::{
    AgentCliArgs, AgentResource, AppBuilder, HttpScope, HttpScopeError, LevelCliArgs,
    LevelResource, ProjectScope, ProjectScopeError, ServiceState, ToolError, dispatch_tool,
};

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

// Use resource-provided projections. The functions are const-compatible
// (they return static slices), but we use a fn to avoid const-eval complexity.
fn all_projections() -> &'static [&'static [oneiros_db::Projection]] {
    &[crate::projections::AGENT, crate::projections::LEVEL]
}

fn project_scope(db: &Database) -> ProjectScope<'_> {
    ProjectScope::new(db, test_source(), all_projections())
}

fn test_service_state(db: Database) -> ServiceState {
    ServiceState::new(db, test_source(), all_projections())
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

/// Convenience: disambiguated fulfill for Agent requests via HttpScope.
async fn http_fulfill_agent(
    scope: &HttpScope,
    request: AgentRequests,
) -> Result<AgentResponses, HttpScopeError> {
    Fulfill::<AgentResource>::fulfill(scope, request).await
}

/// Convenience: disambiguated fulfill for Level requests via HttpScope.
async fn http_fulfill_level(
    scope: &HttpScope,
    request: LevelRequests,
) -> Result<LevelResponses, HttpScopeError> {
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

    fulfill_agent(
        &scope,
        AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("alice"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }),
    )
    .await
    .unwrap();

    fulfill_agent(
        &scope,
        AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("bob"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }),
    )
    .await
    .unwrap();

    let response = fulfill_agent(&scope, AgentRequests::ListAgents(ListAgentsRequest))
        .await
        .expect("list agents");

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

    fulfill_agent(
        &scope,
        AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Gov"),
            prompt: Prompt::default(),
        }),
    )
    .await
    .unwrap();

    let response = fulfill_agent(
        &scope,
        AgentRequests::GetAgent(GetAgentRequest {
            name: AgentName::new("governor"),
        }),
    )
    .await
    .expect("get agent");

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

    let result = fulfill_agent(
        &scope,
        AgentRequests::GetAgent(GetAgentRequest {
            name: AgentName::new("nonexistent"),
        }),
    )
    .await;

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

    fulfill_agent(
        &scope,
        AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Original"),
            prompt: Prompt::default(),
        }),
    )
    .await
    .unwrap();

    let response = fulfill_agent(
        &scope,
        AgentRequests::UpdateAgent(UpdateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Updated"),
            prompt: Prompt::new("New prompt"),
        }),
    )
    .await
    .expect("update agent");

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

    fulfill_agent(
        &scope,
        AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }),
    )
    .await
    .unwrap();

    let response = fulfill_agent(
        &scope,
        AgentRequests::RemoveAgent(RemoveAgentRequest {
            name: AgentName::new("governor"),
        }),
    )
    .await
    .expect("remove agent");

    assert!(matches!(response, AgentResponses::AgentRemoved));

    // Verify it's gone
    let result = fulfill_agent(
        &scope,
        AgentRequests::GetAgent(GetAgentRequest {
            name: AgentName::new("governor"),
        }),
    )
    .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn create_agent_conflict() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);
    let scope = project_scope(&db);

    fulfill_agent(
        &scope,
        AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }),
    )
    .await
    .unwrap();

    let result = fulfill_agent(
        &scope,
        AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }),
    )
    .await;

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

    let result = fulfill_agent(
        &scope,
        AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("nonexistent-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }),
    )
    .await;

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
        LevelRequests::SetLevel(oneiros_model::Level::init(
            "working",
            "Active",
            "Short-term",
        )),
    )
    .await
    .unwrap();

    Fulfill::<LevelResource>::fulfill(
        &scope,
        LevelRequests::SetLevel(oneiros_model::Level::init(
            "archival",
            "Long-term",
            "Permanent",
        )),
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

// ── HttpScope tests ───────────────────────────────────────────────
//
// The mirror proof: Fulfill<Agent> works identically whether the
// backend is a database (ProjectScope) or HTTP calls (HttpScope).
// Same trait, same request types, different fulfillment mechanism.

fn test_http_scope(state: ServiceState) -> HttpScope {
    let router = test_app(state);
    HttpScope::new(router)
}

#[tokio::test]
async fn http_scope_create_and_get_agent() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);
    let scope = test_http_scope(test_service_state(db));

    let response = http_fulfill_agent(
        &scope,
        AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("The governor"),
            prompt: Prompt::new("You govern."),
        }),
    )
    .await
    .expect("create agent via http");

    match response {
        AgentResponses::AgentCreated(agent) => {
            assert_eq!(agent.name, AgentName::new("governor"));
        }
        other => panic!("Expected AgentCreated, got {other:?}"),
    }

    // Get through the same scope — proves round-trip
    let response = http_fulfill_agent(
        &scope,
        AgentRequests::GetAgent(GetAgentRequest {
            name: AgentName::new("governor"),
        }),
    )
    .await
    .expect("get agent via http");

    match response {
        AgentResponses::AgentFound(agent) => {
            assert_eq!(agent.name, AgentName::new("governor"));
            assert_eq!(agent.description, Description::new("The governor"));
        }
        other => panic!("Expected AgentFound, got {other:?}"),
    }
}

#[tokio::test]
async fn http_scope_list_agents() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let state = test_service_state(db);
    state
        .fulfill::<AgentResource>(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("alice"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .unwrap();

    let scope = test_http_scope(state);

    let response: AgentResponses =
        http_fulfill_agent(&scope, AgentRequests::ListAgents(ListAgentsRequest))
            .await
            .expect("list agents via http");

    match response {
        AgentResponses::AgentsListed(agents) => {
            assert_eq!(agents.len(), 1);
            assert_eq!(agents[0].name, AgentName::new("alice"));
        }
        other => panic!("Expected AgentsListed, got {other:?}"),
    }
}

#[tokio::test]
async fn http_scope_set_and_get_level() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    let scope = test_http_scope(test_service_state(db));

    let response = http_fulfill_level(
        &scope,
        LevelRequests::SetLevel(oneiros_model::Level::init(
            "working",
            "Active work",
            "Short-term",
        )),
    )
    .await
    .expect("set level via http");

    assert!(matches!(response, LevelResponses::LevelSet(_)));

    let response = http_fulfill_level(
        &scope,
        LevelRequests::GetLevel(GetLevelRequest {
            name: LevelName::new("working"),
        }),
    )
    .await
    .expect("get level via http");

    match response {
        LevelResponses::LevelFound(level) => {
            assert_eq!(level.name, LevelName::new("working"));
            assert_eq!(level.description, Description::new("Active work"));
        }
        other => panic!("Expected LevelFound, got {other:?}"),
    }
}

#[tokio::test]
async fn http_scope_error_propagation() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    let scope = test_http_scope(test_service_state(db));

    let result = http_fulfill_agent(
        &scope,
        AgentRequests::GetAgent(GetAgentRequest {
            name: AgentName::new("nonexistent"),
        }),
    )
    .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, HttpScopeError::Status(404, _)),
        "Expected 404 status error, got {err:?}"
    );
}

// ── MCP tool tests ────────────────────────────────────────────────
//
// Proves that resources can own their MCP tool dispatch logic.

#[tokio::test]
async fn mcp_create_and_list_agents() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);
    let state = test_service_state(db);

    // Create via MCP tool dispatch
    let params = serde_json::to_string(&CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::new("Gov"),
        prompt: Prompt::default(),
    })
    .unwrap();

    let result = dispatch_tool(&state, "create_agent", &params).unwrap();
    assert!(result.content.contains("governor"));

    // List via MCP tool dispatch
    let result = dispatch_tool(&state, "list_agents", "").unwrap();
    assert!(result.content.contains("governor"));
}

#[tokio::test]
async fn mcp_set_and_get_level() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    let state = test_service_state(db);

    let params = serde_json::to_string(&oneiros_model::Level::init(
        "working",
        "Active",
        "Short-term",
    ))
    .unwrap();

    let result = dispatch_tool(&state, "set_level", &params).unwrap();
    assert!(result.content.contains("working"));

    let get_params = serde_json::to_string(&GetLevelRequest {
        name: LevelName::new("working"),
    })
    .unwrap();

    let result = dispatch_tool(&state, "get_level", &get_params).unwrap();
    assert!(result.content.contains("Active"));
}

#[test]
fn mcp_unknown_tool_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    let state = test_service_state(db);

    let result = dispatch_tool(&state, "nonexistent_tool", "");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ToolError::UnknownTool(_)));
}

// ── CLI command tests ─────────────────────────────────────────────
//
// Proves that CLI commands go through HttpScope (client layer),
// which round-trips through the HTTP handlers to ProjectScope.
// CLI → HttpScope → HTTP → ServiceState → ProjectScope → DB

#[tokio::test]
async fn cli_create_and_list_agents() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);
    let scope = test_http_scope(test_service_state(db));

    // Create via CLI dispatch
    let output = AgentResource::cli_run(
        &scope,
        "create",
        AgentCliArgs {
            name: Some(AgentName::new("governor")),
            persona: Some(PersonaName::new("test-persona")),
            description: Some(Description::new("Gov")),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    assert!(output.messages[0].contains("governor"));
    assert!(output.messages[0].contains("created"));

    // List via CLI dispatch — goes through HttpScope
    let output = AgentResource::cli_run(&scope, "list", AgentCliArgs::default())
        .await
        .unwrap();

    assert_eq!(output.messages.len(), 1);
    assert!(output.messages[0].contains("governor"));
}

#[tokio::test]
async fn cli_set_and_list_levels() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    let scope = test_http_scope(test_service_state(db));

    let output = LevelResource::cli_run(
        &scope,
        "set",
        LevelCliArgs {
            name: Some(LevelName::new("working")),
            description: Some(Description::new("Active")),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    assert!(output.messages[0].contains("working"));

    let output = LevelResource::cli_run(&scope, "list", LevelCliArgs::default())
        .await
        .unwrap();

    assert_eq!(output.messages.len(), 1);
    assert!(output.messages[0].contains("working"));
}

// ── AppBuilder tests ──────────────────────────────────────────────
//
// The registry proof: resources mount themselves into the builder,
// the builder produces composed transport surfaces.

#[tokio::test]
async fn app_builder_composes_http_router() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let state = test_service_state(db);

    // Build the app via mounting — no manual .nest() calls
    let app = AppBuilder::new(state)
        .mount(AgentResource)
        .mount(LevelResource)
        .into_router();

    // Create an agent through the composed router
    let request: Request<Body> = Request::builder()
        .method("POST")
        .uri("/agents")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&CreateAgentRequest {
                name: AgentName::new("governor"),
                persona: PersonaName::new("test-persona"),
                description: Description::new("Gov"),
                prompt: Prompt::default(),
            })
            .unwrap(),
        ))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Set a level through the same composed router
    let request: Request<Body> = Request::builder()
        .method("PUT")
        .uri("/levels/working")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&oneiros_model::Level::init(
                "working",
                "Active",
                "Short-term",
            ))
            .unwrap(),
        ))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Read both back — proves both resources are mounted and served
    let request: Request<Body> = Request::builder()
        .uri("/agents/governor")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: AgentResponses = json_body(response).await;
    assert!(matches!(body, AgentResponses::AgentFound(_)));

    let request: Request<Body> = Request::builder()
        .uri("/levels/working")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: LevelResponses = json_body(response).await;
    assert!(matches!(body, LevelResponses::LevelFound(_)));
}

#[tokio::test]
async fn app_builder_composes_mcp_tools() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());
    seed_persona(&db);

    let state = test_service_state(db);
    let app = AppBuilder::new(state)
        .mount(AgentResource)
        .mount(LevelResource);

    // Create via app's tool dispatch — not the free function
    let params = serde_json::to_string(&CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::new("Gov"),
        prompt: Prompt::default(),
    })
    .unwrap();

    let result = app.dispatch_tool("create_agent", &params).unwrap();
    assert!(result.content.contains("governor"));

    // List levels — different resource, same dispatch
    let params = serde_json::to_string(&oneiros_model::Level::init(
        "working",
        "Active",
        "Short-term",
    ))
    .unwrap();

    let result = app.dispatch_tool("set_level", &params).unwrap();
    assert!(result.content.contains("working"));

    // Unknown tool returns error
    let result = app.dispatch_tool("nonexistent", "");
    assert!(matches!(result.unwrap_err(), ToolError::UnknownTool(_)));
}

#[test]
fn app_builder_collects_projections() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db(dir.path());

    let state = test_service_state(db);
    let app = AppBuilder::new(state)
        .mount(AgentResource)
        .mount(LevelResource);

    let projections = app.projection_slices();

    // Two resources mounted, each contributes one projection slice
    assert_eq!(projections.len(), 2);

    // Agent has 3 projections (created, updated, removed)
    assert_eq!(projections[0].len(), 3);

    // Level has 2 projections (set, removed)
    assert_eq!(projections[1].len(), 2);
}
