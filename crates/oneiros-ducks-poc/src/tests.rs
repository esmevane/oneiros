use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use oneiros_db::Database;
use oneiros_model::*;
use tower::ServiceExt;

use crate::domains::agent::{AgentError, AgentService};
use crate::ports::AppContext;

// ── Helpers ───────────────────────────────────────────────────────

fn test_db(dir: &std::path::Path) -> Database {
    Database::create_brain_db(&dir.join("test-brain.db")).expect("create brain db")
}

const PROJECTIONS: &[&[oneiros_db::Projection]] = &[crate::domains::agent::PROJECTIONS];

fn test_ctx(db: Database) -> AppContext {
    AppContext::new(db, PROJECTIONS)
}

fn seed_persona(ctx: &AppContext) {
    ctx.with_db(|db| {
        db.set_persona(
            &PersonaName::new("test-persona"),
            &Description::new("A test persona"),
            &Prompt::new("You are a test."),
        )
        .expect("seed persona")
    });
}

async fn json_body<T: serde::de::DeserializeOwned>(response: axum::http::Response<Body>) -> T {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

fn test_app(ctx: AppContext) -> axum::Router {
    crate::layers::http_router(ctx)
}

// ── Domain service tests ──────────────────────────────────────────

#[test]
fn create_agent() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));
    seed_persona(&ctx);

    let response = AgentService::create(&ctx, CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::new("Gov"),
        prompt: Prompt::new("You govern."),
    })
    .unwrap();

    match response {
        AgentResponses::AgentCreated(a) => {
            assert_eq!(a.name, AgentName::new("governor"));
        }
        other => panic!("Expected AgentCreated, got {other:?}"),
    }
}

#[test]
fn list_agents() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));
    seed_persona(&ctx);

    AgentService::create(&ctx, CreateAgentRequest {
        name: AgentName::new("alice"),
        persona: PersonaName::new("test-persona"),
        description: Description::default(),
        prompt: Prompt::default(),
    }).unwrap();

    AgentService::create(&ctx, CreateAgentRequest {
        name: AgentName::new("bob"),
        persona: PersonaName::new("test-persona"),
        description: Description::default(),
        prompt: Prompt::default(),
    }).unwrap();

    let response = AgentService::list(&ctx).unwrap();
    match response {
        AgentResponses::AgentsListed(agents) => assert_eq!(agents.len(), 2),
        other => panic!("Expected AgentsListed, got {other:?}"),
    }
}

#[test]
fn get_agent_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));

    let result = AgentService::get(&ctx, &AgentName::new("nonexistent"));
    assert!(matches!(result.unwrap_err(), AgentError::NotFound(NotFound::Agent(_))));
}

#[test]
fn create_agent_missing_persona() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));

    let result = AgentService::create(&ctx, CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("nonexistent"),
        description: Description::default(),
        prompt: Prompt::default(),
    });

    assert!(matches!(result.unwrap_err(), AgentError::NotFound(NotFound::Persona(_))));
}

#[test]
fn create_agent_conflict() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));
    seed_persona(&ctx);

    AgentService::create(&ctx, CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::default(),
        prompt: Prompt::default(),
    }).unwrap();

    let result = AgentService::create(&ctx, CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::default(),
        prompt: Prompt::default(),
    });

    assert!(matches!(result.unwrap_err(), AgentError::Conflict(_)));
}

#[test]
fn update_agent() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));
    seed_persona(&ctx);

    AgentService::create(&ctx, CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::new("Original"),
        prompt: Prompt::default(),
    }).unwrap();

    let response = AgentService::update(&ctx, UpdateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::new("Updated"),
        prompt: Prompt::new("New prompt"),
    }).unwrap();

    match response {
        AgentResponses::AgentUpdated(a) => {
            assert_eq!(a.description, Description::new("Updated"));
        }
        other => panic!("Expected AgentUpdated, got {other:?}"),
    }
}

#[test]
fn remove_agent() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));
    seed_persona(&ctx);

    AgentService::create(&ctx, CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::default(),
        prompt: Prompt::default(),
    }).unwrap();

    AgentService::remove(&ctx, AgentName::new("governor")).unwrap();

    let result = AgentService::get(&ctx, &AgentName::new("governor"));
    assert!(result.is_err());
}

// ── Broadcast tests ───────────────────────────────────────────────

#[test]
fn broadcast_receives_events() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));
    seed_persona(&ctx);

    let mut subscriber = ctx.subscribe();

    AgentService::create(&ctx, CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::default(),
        prompt: Prompt::default(),
    }).unwrap();

    let event = subscriber.try_recv().expect("receive broadcast");
    assert!(matches!(
        &event,
        Event::Known(k) if matches!(&k.data, Events::Agent(AgentEvents::AgentCreated(_)))
    ));
}

// ── HTTP tests ────────────────────────────────────────────────────

#[tokio::test]
async fn http_create_agent() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));
    seed_persona(&ctx);
    let app = test_app(ctx);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/agents")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&CreateAgentRequest {
                    name: AgentName::new("governor"),
                    persona: PersonaName::new("test-persona"),
                    description: Description::new("Gov"),
                    prompt: Prompt::default(),
                }).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body: AgentResponses = json_body(response).await;
    assert!(matches!(body, AgentResponses::AgentCreated(_)));
}

#[tokio::test]
async fn http_get_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));
    let app = test_app(ctx);

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
    let ctx = test_ctx(test_db(dir.path()));
    seed_persona(&ctx);
    let app = test_app(ctx);

    // Create
    let req: Request<Body> = Request::builder()
        .method("POST").uri("/agents")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Original"),
            prompt: Prompt::default(),
        }).unwrap())).unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Read
    let req: Request<Body> = Request::builder()
        .uri("/agents/governor").body(Body::empty()).unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Update
    let req: Request<Body> = Request::builder()
        .method("PUT").uri("/agents/governor")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&UpdateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Updated"),
            prompt: Prompt::new("New"),
        }).unwrap())).unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Delete
    let req: Request<Body> = Request::builder()
        .method("DELETE").uri("/agents/governor").body(Body::empty()).unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Gone
    let req: Request<Body> = Request::builder()
        .uri("/agents/governor").body(Body::empty()).unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ── MCP tests ─────────────────────────────────────────────────────

#[test]
fn mcp_create_and_list() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));
    seed_persona(&ctx);

    let params = serde_json::to_string(&CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::new("Gov"),
        prompt: Prompt::default(),
    }).unwrap();

    let result = crate::layers::dispatch_tool(&ctx, "create_agent", &params).unwrap();
    assert!(result.content.contains("governor"));

    let result = crate::layers::dispatch_tool(&ctx, "list_agents", "").unwrap();
    assert!(result.content.contains("governor"));
}

#[test]
fn mcp_unknown_tool() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));

    let result = crate::layers::dispatch_tool(&ctx, "nonexistent", "");
    assert!(result.is_err());
}

// ── CLI (remote through HTTP) tests ───────────────────────────────

#[tokio::test]
async fn cli_create_and_list() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));
    seed_persona(&ctx);
    let app = test_app(ctx);
    let remote = crate::domains::agent::cli::RemoteAgents::new(app);

    let response = remote.create(CreateAgentRequest {
        name: AgentName::new("governor"),
        persona: PersonaName::new("test-persona"),
        description: Description::new("Gov"),
        prompt: Prompt::default(),
    }).await.unwrap();

    assert!(matches!(response, AgentResponses::AgentCreated(_)));

    let response = remote.list().await.unwrap();
    match response {
        AgentResponses::AgentsListed(agents) => {
            assert_eq!(agents.len(), 1);
        }
        other => panic!("Expected AgentsListed, got {other:?}"),
    }
}

#[tokio::test]
async fn cli_get_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));
    let app = test_app(ctx);
    let remote = crate::domains::agent::cli::RemoteAgents::new(app);

    let result = remote.get(&AgentName::new("nonexistent")).await;
    assert!(matches!(result.unwrap_err(), crate::domains::agent::cli::CliError::Status(404, _)));
}

// ── Full integration ──────────────────────────────────────────────

#[tokio::test]
async fn full_integration_all_surfaces() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = test_ctx(test_db(dir.path()));
    seed_persona(&ctx);
    let mut subscriber = ctx.subscribe();
    let app = test_app(ctx.clone());

    // 1. Create via HTTP
    let req: Request<Body> = Request::builder()
        .method("POST").uri("/agents")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&CreateAgentRequest {
            name: AgentName::new("governor"),
            persona: PersonaName::new("test-persona"),
            description: Description::new("Gov"),
            prompt: Prompt::default(),
        }).unwrap())).unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // 2. Verify via MCP
    let result = crate::layers::dispatch_tool(&ctx, "list_agents", "").unwrap();
    assert!(result.content.contains("governor"));

    // 3. Verify via CLI (remote)
    let remote = crate::domains::agent::cli::RemoteAgents::new(app);
    let response = remote.get(&AgentName::new("governor")).await.unwrap();
    assert!(matches!(response, AgentResponses::AgentFound(_)));

    // 4. Verify broadcast
    let event = subscriber.try_recv().unwrap();
    assert!(matches!(
        &event,
        Event::Known(k) if matches!(&k.data, Events::Agent(AgentEvents::AgentCreated(_)))
    ));
}
