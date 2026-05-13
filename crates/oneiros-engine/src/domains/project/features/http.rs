use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::*;

pub(crate) struct ProjectRouter;

impl ProjectRouter {
    pub(crate) fn routes(&self) -> ApiRouter<ServerState> {
        ApiRouter::new()
            .api_route(
                "/projects",
                routing::get_with(list, |op| resource_op!(op, ProjectDocs::List)).post_with(
                    create,
                    |op| {
                        resource_op!(op, ProjectDocs::Create)
                            .response::<201, Json<ProjectResponse>>()
                    },
                ),
            )
            .api_route(
                "/projects/{name}",
                routing::get_with(show, |op| resource_op!(op, ProjectDocs::Show)),
            )
            .route("/summary", axum::routing::get(summary))
    }
}

async fn create(
    scope: Scope<AtHost>,
    mailbox: Mailbox,
    Json(body): Json<CreateProject>,
) -> Result<(StatusCode, Json<ProjectResponse>), ProjectError> {
    let response = ProjectService::create(&scope, &mailbox, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn list(
    scope: Scope<AtHost>,
    Query(params): Query<ListProjects>,
) -> Result<Json<ProjectResponse>, ProjectError> {
    Ok(Json(ProjectService::list(&scope, &params).await?))
}

async fn show(
    scope: Scope<AtHost>,
    Path(key): Path<ResourceKey<ProjectName>>,
) -> Result<Json<ProjectResponse>, ProjectError> {
    Ok(Json(
        ProjectService::get(&scope, &GetProject::builder_v1().key(key).build().into()).await?,
    ))
}

async fn summary(scope: Scope<AtBookmark>) -> Result<Json<ProjectSummary>, ProjectError> {
    let db = BookmarkDb::open(&scope).await?;

    let agents = AgentStore::new(&db).list().unwrap_or_default();
    let agent_count = agents.len();

    let cognitions = CognitionStore::new(&db)
        .list(None, None)
        .unwrap_or_default();
    let cognition_count = cognitions.len();

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

    let summary = ProjectSummary {
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
