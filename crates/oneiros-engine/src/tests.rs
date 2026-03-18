use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use rusqlite::Connection;
use tower::ServiceExt;

use crate::contexts::ProjectContext;
use crate::domains;
use crate::store;

// ── Helpers ───────────────────────────────────────────────────────

fn project_ctx() -> ProjectContext {
    let conn = Connection::open_in_memory().expect("open db");
    store::initialize(&conn).expect("init store");

    // Migrate all project-scoped repos
    domains::level::repo::LevelRepo::new(&conn).migrate().unwrap();
    domains::texture::repo::TextureRepo::new(&conn).migrate().unwrap();
    domains::sensation::repo::SensationRepo::new(&conn).migrate().unwrap();
    domains::nature::repo::NatureRepo::new(&conn).migrate().unwrap();
    domains::persona::repo::PersonaRepo::new(&conn).migrate().unwrap();
    domains::urge::repo::UrgeRepo::new(&conn).migrate().unwrap();
    domains::agent::repo::AgentRepo::new(&conn).migrate().unwrap();
    domains::cognition::repo::CognitionRepo::new(&conn).migrate().unwrap();
    domains::memory::repo::MemoryRepo::new(&conn).migrate().unwrap();
    domains::experience::repo::ExperienceRepo::new(&conn).migrate().unwrap();
    domains::connection::repo::ConnectionRepo::new(&conn).migrate().unwrap();
    domains::pressure::repo::PressureRepo::new(&conn).migrate().unwrap();
    domains::search::repo::SearchRepo::new(&conn).migrate().unwrap();

    static PROJECTIONS: &[&[store::Projection]] = &[
        domains::level::PROJECTIONS,
        domains::texture::PROJECTIONS,
        domains::sensation::PROJECTIONS,
        domains::nature::PROJECTIONS,
        domains::persona::PROJECTIONS,
        domains::urge::PROJECTIONS,
        domains::agent::PROJECTIONS,
        domains::cognition::PROJECTIONS,
        domains::memory::PROJECTIONS,
        domains::experience::PROJECTIONS,
        domains::connection::PROJECTIONS,
        domains::pressure::PROJECTIONS,
    ];

    ProjectContext::new(conn, PROJECTIONS)
}

fn seed_persona(ctx: &ProjectContext) {
    use domains::persona::{model::Persona, service::PersonaService};
    PersonaService::set(ctx, Persona {
        name: "test-persona".into(),
        description: "A test persona".into(),
        prompt: "You are a test.".into(),
    }).unwrap();
}

fn seed_agent(ctx: &ProjectContext) {
    use domains::agent::service::AgentService;
    AgentService::create(ctx, "gov".into(), "test-persona".into(), "Governor".into(), "You govern.".into()).unwrap();
}

async fn json_body<T: serde::de::DeserializeOwned>(response: axum::http::Response<Body>) -> T {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

// ── Vocabulary domain tests ───────────────────────────────────────

#[test]
fn level_crud() {
    use domains::level::{model::Level, service::LevelService, responses::LevelResponse};
    let ctx = project_ctx();

    LevelService::set(&ctx, Level { name: "working".into(), description: "Active".into(), prompt: "".into() }).unwrap();
    assert!(matches!(LevelService::get(&ctx, "working").unwrap(), LevelResponse::Found(_)));

    match LevelService::list(&ctx).unwrap() {
        LevelResponse::Listed(levels) => assert_eq!(levels.len(), 1),
        other => panic!("Expected Listed, got {other:?}"),
    }

    LevelService::remove(&ctx, "working").unwrap();
    assert!(LevelService::get(&ctx, "working").is_err());
}

#[test]
fn persona_crud() {
    use domains::persona::{model::Persona, service::PersonaService, responses::PersonaResponse};
    let ctx = project_ctx();

    PersonaService::set(&ctx, Persona { name: "process".into(), description: "Process agents".into(), prompt: "".into() }).unwrap();
    assert!(matches!(PersonaService::get(&ctx, "process").unwrap(), PersonaResponse::Found(_)));
}

// ── Entity domain tests ──────────────────────────────────────────

#[test]
fn agent_create_and_get() {
    use domains::agent::{service::AgentService, responses::AgentResponse};
    let ctx = project_ctx();
    seed_persona(&ctx);

    let resp = AgentService::create(&ctx, "governor".into(), "test-persona".into(), "The governor".into(), "You govern.".into()).unwrap();
    assert!(matches!(resp, AgentResponse::Created(_)));

    match AgentService::get(&ctx, "governor").unwrap() {
        AgentResponse::Found(a) => {
            assert_eq!(a.name, "governor");
            assert_eq!(a.persona, "test-persona");
        }
        other => panic!("Expected Found, got {other:?}"),
    }
}

#[test]
fn agent_persona_validation() {
    use domains::agent::{service::AgentService, errors::AgentError};
    let ctx = project_ctx();

    let result = AgentService::create(&ctx, "gov".into(), "nonexistent".into(), "".into(), "".into());
    assert!(matches!(result, Err(AgentError::PersonaNotFound(_))));
}

#[test]
fn agent_name_conflict() {
    use domains::agent::{service::AgentService, errors::AgentError};
    let ctx = project_ctx();
    seed_persona(&ctx);

    AgentService::create(&ctx, "gov".into(), "test-persona".into(), "".into(), "".into()).unwrap();
    let result = AgentService::create(&ctx, "gov".into(), "test-persona".into(), "".into(), "".into());
    assert!(matches!(result, Err(AgentError::Conflict(_))));
}

#[test]
fn cognition_add_and_list() {
    use domains::cognition::{service::CognitionService, responses::CognitionResponse};
    let ctx = project_ctx();
    seed_persona(&ctx);
    seed_agent(&ctx);

    let resp = CognitionService::add(&ctx, "gov".into(), "observation".into(), "Something interesting".into()).unwrap();
    assert!(matches!(resp, CognitionResponse::Added(_)));

    match CognitionService::list(&ctx, Some("gov"), None).unwrap() {
        CognitionResponse::Listed(cogs) => assert_eq!(cogs.len(), 1),
        other => panic!("Expected Listed, got {other:?}"),
    }
}

// ── Broadcast test ────────────────────────────────────────────────

#[test]
fn broadcast_receives_events_across_domains() {
    use domains::level::{model::Level, service::LevelService};
    use domains::agent::service::AgentService;
    let ctx = project_ctx();
    seed_persona(&ctx);

    let mut sub = ctx.subscribe();

    LevelService::set(&ctx, Level { name: "working".into(), description: "".into(), prompt: "".into() }).unwrap();
    let event = sub.try_recv().unwrap();
    assert_eq!(event.event_type, "level-set");

    AgentService::create(&ctx, "gov".into(), "test-persona".into(), "".into(), "".into()).unwrap();
    let event = sub.try_recv().unwrap();
    assert_eq!(event.event_type, "agent-created");
}

// ── HTTP collector tests ──────────────────────────────────────────

#[tokio::test]
async fn http_serves_multiple_domains() {
    let ctx = project_ctx();
    seed_persona(&ctx);
    let app = crate::http::project_router(ctx);

    // Set a level
    let req: Request<Body> = Request::builder()
        .method("PUT").uri("/levels/working")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name":"working","description":"Active","prompt":""}"#))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Create an agent
    let req: Request<Body> = Request::builder()
        .method("POST").uri("/agents")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name":"gov","persona":"test-persona","description":"Gov","prompt":""}"#))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Read both back
    let req: Request<Body> = Request::builder().uri("/levels/working").body(Body::empty()).unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let req: Request<Body> = Request::builder().uri("/agents/gov").body(Body::empty()).unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn http_not_found() {
    let ctx = project_ctx();
    let app = crate::http::project_router(ctx);

    let req: Request<Body> = Request::builder().uri("/agents/nope").body(Body::empty()).unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    let req: Request<Body> = Request::builder().uri("/levels/nope").body(Body::empty()).unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ── Full integration ──────────────────────────────────────────────

#[tokio::test]
async fn full_integration() {
    let ctx = project_ctx();
    seed_persona(&ctx);
    let app = crate::http::project_router(ctx.clone());

    // Subscribe AFTER seeding so we only see the agent event
    let mut sub = ctx.subscribe();

    // Create agent via HTTP
    let req: Request<Body> = Request::builder()
        .method("POST").uri("/agents")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name":"gov","persona":"test-persona","description":"Gov","prompt":""}"#))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Read back via HTTP
    let req: Request<Body> = Request::builder().uri("/agents/gov").body(Body::empty()).unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Broadcast received the agent-created event
    let agent_event = sub.try_recv().unwrap();
    assert_eq!(agent_event.event_type, "agent-created");
}
