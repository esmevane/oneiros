use aide::axum::{ApiRouter, routing};
use axum::{Json, http::StatusCode};

use crate::*;

pub struct ProjectRouter;

impl ProjectRouter {
    pub fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new()
            // System-scoped — no auth needed (creating a brain is how you get a token)
            .api_route(
                "/projects",
                routing::post_with(init, |op| {
                    resource_op!(op, ProjectDocs::Init).response::<201, Json<ProjectResponse>>()
                }),
            )
            // Brain-scoped — BrainSummary lacks OperationOutput, use plain route()
            .route("/summary", axum::routing::get(summary))
    }
}

async fn init(
    context: HostLog,
    Json(body): Json<InitProject>,
) -> Result<(StatusCode, Json<ProjectResponse>), ProjectError> {
    let response = ProjectService::init(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn summary(context: ProjectLog) -> Result<Json<BrainSummary>, ProjectError> {
    let db = context.db()?;

    let agents = AgentStore::new(&db).list().unwrap_or_default();
    let agent_count = agents.len();

    let cognitions = CognitionStore::new(&db)
        .list(None, None)
        .unwrap_or_default();
    let cognition_count = cognitions.len();

    // Get recent cognitions (last 30, newest first)
    let recent_cognitions = {
        let mut recent = cognitions;
        recent.sort_by_key(|b| std::cmp::Reverse(b.created_at));
        recent.truncate(30);
        recent
    };

    let memory_count = MemoryStore::new(&db).list(None).unwrap_or_default().len();
    let experience_count = ExperienceStore::new(&db)
        .list(None)
        .unwrap_or_default()
        .len();
    let connection_count = ConnectionStore::new(&db)
        .list(None)
        .unwrap_or_default()
        .len();

    let event_count = db
        .query_row("SELECT COUNT(*) FROM events", [], |row| {
            row.get::<_, i64>(0)
        })
        .unwrap_or(0) as usize;

    let summary = BrainSummary {
        agents,
        agent_count,
        cognition_count,
        memory_count,
        experience_count,
        connection_count,
        event_count,
        recent_cognitions,
    };

    Ok(Json(summary))
}
