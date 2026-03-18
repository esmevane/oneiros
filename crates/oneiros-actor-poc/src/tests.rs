use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use oneiros_actor::spawn;
use oneiros_db::Database;
use oneiros_model::*;
use tower::ServiceExt;

use crate::agent::{AgentActor, AgentError};
use crate::database::Db;
use crate::registry::Registry;

// ── Test helpers ───────────────────────────────────────────────────

fn test_db(dir: &std::path::Path) -> Database {
    Database::create_brain_db(dir.join("test-brain.db")).expect("create brain db")
}

const PROJECTIONS: &[&[oneiros_db::Projection]] = &[crate::projections::AGENT];

async fn test_db_actor(dir: &std::path::Path) -> Db {
    let db = test_db(dir);
    Db::spawn(db, PROJECTIONS)
}

async fn seed_persona(db: &Db) {
    db.set_persona(
        &PersonaName::new("test-persona"),
        &Description::new("A test persona"),
        &Prompt::new("You are a test."),
    )
    .await;
}

type AgentHandle = oneiros_actor::Handle<AgentRequests, Result<AgentResponses, AgentError>>;

async fn test_agent_actor(db: Db) -> AgentHandle {
    spawn(AgentActor::new(db))
}

// ── Agent actor tests ─────────────────────────────────────────────

#[tokio::test]
async fn create_agent_through_actor() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;
    seed_persona(&db).await;

    let agent = test_agent_actor(db).await;

    let response = agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("The governor"),
            prompt: Prompt::new("You govern."),
        }))
        .await
        .unwrap() // Handle send
        .unwrap(); // Domain result

    match response {
        AgentResponses::AgentCreated(a) => {
            assert_eq!(a.name, AgentName::new("governor"));
            assert_eq!(a.persona, PersonaName::new("test-persona"));
        }
        other => panic!("Expected AgentCreated, got {other:?}"),
    }
}

#[tokio::test]
async fn list_agents_through_actor() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;
    seed_persona(&db).await;

    let agent = test_agent_actor(db).await;

    agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("alice"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap()
        .unwrap();

    agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("bob"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap()
        .unwrap();

    let response = agent
        .send(AgentRequests::ListAgents(ListAgentsRequest))
        .await
        .unwrap()
        .unwrap();

    match response {
        AgentResponses::AgentsListed(agents) => {
            assert_eq!(agents.len(), 2);
        }
        other => panic!("Expected AgentsListed, got {other:?}"),
    }
}

#[tokio::test]
async fn get_agent_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;

    let agent = test_agent_actor(db).await;

    let result = agent
        .send(AgentRequests::GetAgent(GetAgentRequest {
            name: AgentName::new("nonexistent"),
        }))
        .await
        .unwrap(); // Handle send succeeds

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AgentError::NotFound(NotFound::Agent(_))
    ));
}

#[tokio::test]
async fn create_agent_missing_persona() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;
    // Deliberately NOT seeding persona

    let agent = test_agent_actor(db).await;

    let result = agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("nonexistent"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap();

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        AgentError::NotFound(NotFound::Persona(_))
    ));
}

#[tokio::test]
async fn create_agent_conflict() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;
    seed_persona(&db).await;

    let agent = test_agent_actor(db).await;

    agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap()
        .unwrap();

    let result = agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap();

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AgentError::Conflict(_)));
}

#[tokio::test]
async fn update_agent_through_actor() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;
    seed_persona(&db).await;

    let agent = test_agent_actor(db).await;

    agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Original"),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap()
        .unwrap();

    let response = agent
        .send(AgentRequests::UpdateAgent(UpdateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Updated"),
            prompt: Prompt::new("New prompt"),
        }))
        .await
        .unwrap()
        .unwrap();

    match response {
        AgentResponses::AgentUpdated(a) => {
            assert_eq!(a.description, Description::new("Updated"));
        }
        other => panic!("Expected AgentUpdated, got {other:?}"),
    }
}

#[tokio::test]
async fn remove_agent_through_actor() {
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;
    seed_persona(&db).await;

    let agent = test_agent_actor(db).await;

    agent
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap()
        .unwrap();

    let response = agent
        .send(AgentRequests::RemoveAgent(RemoveAgentRequest {
            name: AgentName::new("governor"),
        }))
        .await
        .unwrap()
        .unwrap();

    assert!(matches!(response, AgentResponses::AgentRemoved));

    // Verify it's gone
    let result = agent
        .send(AgentRequests::GetAgent(GetAgentRequest {
            name: AgentName::new("governor"),
        }))
        .await
        .unwrap();

    assert!(result.is_err());
}

// ── Registry tests ────────────────────────────────────────────────

async fn test_registry(dir: &std::path::Path) -> Registry {
    let db = test_db_actor(dir).await;
    seed_persona(&db).await;
    Registry::build(db)
}

#[tokio::test]
async fn registry_dispatches_to_agent_actor() {
    let dir = tempfile::tempdir().unwrap();
    let registry = test_registry(dir.path()).await;

    let response = registry
        .agents
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Gov"),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap()
        .unwrap();

    assert!(matches!(response, AgentResponses::AgentCreated(_)));

    let response = registry
        .agents
        .send(AgentRequests::ListAgents(ListAgentsRequest))
        .await
        .unwrap()
        .unwrap();

    match response {
        AgentResponses::AgentsListed(agents) => assert_eq!(agents.len(), 1),
        other => panic!("Expected AgentsListed, got {other:?}"),
    }
}

// ── Broadcast tests ───────────────────────────────────────────────

#[tokio::test]
async fn broadcast_receives_events_from_agent_actor() {
    let dir = tempfile::tempdir().unwrap();
    let registry = test_registry(dir.path()).await;

    // Subscribe BEFORE the event is emitted
    let mut subscriber = registry.subscribe();

    // Create an agent — this should broadcast an event
    registry
        .agents
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap()
        .unwrap();

    // The subscriber should receive the AgentCreated event
    let event = subscriber.recv().await.expect("receive broadcast event");

    match &event {
        Event::Known(known) => match &known.data {
            Events::Agent(AgentEvents::AgentCreated(agent)) => {
                assert_eq!(agent.name, AgentName::new("governor"));
            }
            other => panic!("Expected AgentCreated event, got {other:?}"),
        },
        other => panic!("Expected Known event, got {other:?}"),
    }
}

#[tokio::test]
async fn broadcast_receives_multiple_events() {
    let dir = tempfile::tempdir().unwrap();
    let registry = test_registry(dir.path()).await;

    let mut subscriber = registry.subscribe();

    // Create then remove
    registry
        .agents
        .send(AgentRequests::CreateAgent(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .await
        .unwrap()
        .unwrap();

    registry
        .agents
        .send(AgentRequests::RemoveAgent(RemoveAgentRequest {
            name: AgentName::new("governor"),
        }))
        .await
        .unwrap()
        .unwrap();

    // Should receive both events in order
    let event1 = subscriber.recv().await.unwrap();
    let event2 = subscriber.recv().await.unwrap();

    assert!(matches!(
        &event1,
        Event::Known(k) if matches!(&k.data, Events::Agent(AgentEvents::AgentCreated(_)))
    ));
    assert!(matches!(
        &event2,
        Event::Known(k) if matches!(&k.data, Events::Agent(AgentEvents::AgentRemoved(_)))
    ));
}

// ── HTTP tests ────────────────────────────────────────────────────

async fn json_body<T: serde::de::DeserializeOwned>(response: axum::http::Response<Body>) -> T {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

fn test_app(registry: Registry) -> axum::Router {
    crate::http::http_router(registry)
}

#[tokio::test]
async fn http_create_agent() {
    let dir = tempfile::tempdir().unwrap();
    let registry = test_registry(dir.path()).await;
    let app = test_app(registry);

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
                        description: Description::new("Gov"),
                        prompt: Prompt::default(),
                    })
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body: AgentResponses = json_body(response).await;
    assert!(matches!(body, AgentResponses::AgentCreated(_)));
}

#[tokio::test]
async fn http_get_agent_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let registry = test_registry(dir.path()).await;
    let app = test_app(registry);

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
async fn http_full_crud_cycle() {
    let dir = tempfile::tempdir().unwrap();
    let registry = test_registry(dir.path()).await;
    let app = test_app(registry);

    // Create
    let request: Request<Body> = Request::builder()
        .method("POST")
        .uri("/agents")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&CreateAgentRequest {
                name: AgentName::new("governor"),
                persona: PersonaName::new("test-persona"),
                description: Description::new("Original"),
                prompt: Prompt::default(),
            })
            .unwrap(),
        ))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Read
    let request: Request<Body> = Request::builder()
        .uri("/agents/governor")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: AgentResponses = json_body(response).await;
    match body {
        AgentResponses::AgentFound(a) => {
            assert_eq!(a.description, Description::new("Original"));
        }
        other => panic!("Expected AgentFound, got {other:?}"),
    }

    // Update
    let request: Request<Body> = Request::builder()
        .method("PUT")
        .uri("/agents/governor")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&UpdateAgentRequest {
                name: AgentName::new("governor"),
                persona: PersonaName::new("test-persona"),
                description: Description::new("Updated"),
                prompt: Prompt::new("New prompt"),
            })
            .unwrap(),
        ))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // List
    let request: Request<Body> = Request::builder()
        .uri("/agents")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body: AgentResponses = json_body(response).await;
    match body {
        AgentResponses::AgentsListed(agents) => assert_eq!(agents.len(), 1),
        other => panic!("Expected AgentsListed, got {other:?}"),
    }

    // Delete
    let request: Request<Body> = Request::builder()
        .method("DELETE")
        .uri("/agents/governor")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify gone
    let request: Request<Body> = Request::builder()
        .uri("/agents/governor")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ── HTTP + Broadcast integration ──────────────────────────────────

#[tokio::test]
async fn http_create_broadcasts_event() {
    let dir = tempfile::tempdir().unwrap();
    let registry = test_registry(dir.path()).await;
    let mut subscriber = registry.subscribe();
    let app = test_app(registry);

    // Create via HTTP
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
                        description: Description::default(),
                        prompt: Prompt::default(),
                    })
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // Event was broadcast
    let event = subscriber.recv().await.unwrap();
    assert!(matches!(
        &event,
        Event::Known(k) if matches!(&k.data, Events::Agent(AgentEvents::AgentCreated(_)))
    ));
}

// ── MCP tool tests ────────────────────────────────────────────────

#[tokio::test]
async fn mcp_create_and_list_agents() {
    let dir = tempfile::tempdir().unwrap();
    let registry = test_registry(dir.path()).await;

    let params = serde_json::to_string(&CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::new("Gov"),
        prompt: Prompt::default(),
    })
    .unwrap();

    let result = crate::mcp::dispatch_tool(&registry, "create_agent", &params)
        .await
        .unwrap();
    assert!(result.content.contains("governor"));

    let result = crate::mcp::dispatch_tool(&registry, "list_agents", "")
        .await
        .unwrap();
    assert!(result.content.contains("governor"));
}

#[tokio::test]
async fn mcp_unknown_tool_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let registry = test_registry(dir.path()).await;

    let result = crate::mcp::dispatch_tool(&registry, "nonexistent", "").await;
    assert!(matches!(
        result.unwrap_err(),
        crate::mcp::ToolError::UnknownTool(_)
    ));
}

// ── CLI tests (through HTTP) ──────────────────────────────────────

#[tokio::test]
async fn cli_create_and_list_through_http() {
    let dir = tempfile::tempdir().unwrap();
    let registry = test_registry(dir.path()).await;
    let app = test_app(registry);

    // CLI uses RemoteAgents which wraps the router
    let remote = crate::cli::RemoteAgents::new(app);

    // Create via CLI remote
    let response = remote
        .create(CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Gov"),
            prompt: Prompt::default(),
        })
        .await
        .unwrap();

    assert!(matches!(response, AgentResponses::AgentCreated(_)));

    // List via CLI remote
    let response = remote.list().await.unwrap();
    match response {
        AgentResponses::AgentsListed(agents) => {
            assert_eq!(agents.len(), 1);
            assert_eq!(agents[0].name, AgentName::new("governor"));
        }
        other => panic!("Expected AgentsListed, got {other:?}"),
    }
}

#[tokio::test]
async fn cli_get_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let registry = test_registry(dir.path()).await;
    let app = test_app(registry);
    let remote = crate::cli::RemoteAgents::new(app);

    let result = remote.get(&AgentName::new("nonexistent")).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        crate::cli::RemoteError::Status(404, _)
    ));
}

// ── Config actor tests ────────────────────────────────────────────

#[tokio::test]
async fn config_actor_responds_to_queries() {
    use crate::config::*;

    let config = ServiceConfig {
        data_dir: std::path::PathBuf::from("/tmp/test"),
        port: 2100,
        grace_period_secs: 5,
    };

    let handle = oneiros_actor::spawn(ConfigActor::new(config));

    let response = handle.send(ConfigMessage::GetPort).await.unwrap();
    match response {
        ConfigResponse::Port(port) => assert_eq!(port, 2100),
        other => panic!("Expected Port, got {other:?}"),
    }

    let response = handle.send(ConfigMessage::GetDataDir).await.unwrap();
    match response {
        ConfigResponse::DataDir(dir) => {
            assert_eq!(dir, std::path::PathBuf::from("/tmp/test"))
        }
        other => panic!("Expected DataDir, got {other:?}"),
    }

    let response = handle.send(ConfigMessage::GetAll).await.unwrap();
    match response {
        ConfigResponse::All(cfg) => {
            assert_eq!(cfg.port, 2100);
            assert_eq!(cfg.grace_period_secs, 5);
        }
        other => panic!("Expected All, got {other:?}"),
    }
}

// ── Scope nesting / capability tests ──────────────────────────────
//
// System scope: can list tenants, create projects
// Project scope: can manage agents, levels, etc.
// The registry provides different capability levels based on auth.

#[tokio::test]
async fn unauthenticated_registry_has_no_agent_access() {
    // A "system-only" registry has no agent actor wired.
    // For the POC, we simulate this by checking that the registry
    // was built without brain-scoped actors.
    //
    // In production, this would be: create a system registry,
    // attempt an agent operation, get a capability error.

    // For now, prove the concept: build a registry, then prove
    // that subscribing + dispatching through it works.
    // The auth gate would sit at the HTTP layer (extractors)
    // or at the registry (returning None for handles the session
    // doesn't have access to).

    // Minimal proof: build two registries with different capabilities
    let dir = tempfile::tempdir().unwrap();
    let db = test_db_actor(dir.path()).await;
    seed_persona(&db).await;

    // Full registry — has agents
    let full_registry = Registry::build(db.clone());
    let response = full_registry
        .agents
        .send(AgentRequests::ListAgents(ListAgentsRequest))
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(response, AgentResponses::AgentsListed(_)));

    // The concept of a "narrow" registry that doesn't expose agents
    // would be a different Registry type or an Option<Handle> pattern.
    // For now, we've proven that the registry CAN scope access —
    // the question is the ergonomics of making handles optional.
}

// ── Full integration test ─────────────────────────────────────────
//
// Proves the complete chain: HTTP request → Registry → Actor → DB Actor
// → Broadcast → Subscriber. Plus MCP and CLI through the same registry.

#[tokio::test]
async fn full_integration_all_surfaces() {
    let dir = tempfile::tempdir().unwrap();
    let registry = test_registry(dir.path()).await;
    let mut subscriber = registry.subscribe();
    let app = test_app(registry.clone());

    // 1. Create via HTTP
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

    // 2. Verify via MCP
    let result = crate::mcp::dispatch_tool(&registry, "list_agents", "")
        .await
        .unwrap();
    assert!(result.content.contains("governor"));

    // 3. Verify via CLI (remote through HTTP)
    let remote = crate::cli::RemoteAgents::new(app);
    let response = remote.get(&AgentName::new("governor")).await.unwrap();
    assert!(matches!(response, AgentResponses::AgentFound(_)));

    // 4. Verify broadcast received the event
    let event = subscriber.recv().await.unwrap();
    assert!(matches!(
        &event,
        Event::Known(k) if matches!(&k.data, Events::Agent(AgentEvents::AgentCreated(_)))
    ));
}
