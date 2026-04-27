use aide::axum::{ApiRouter, routing};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::sse::{Event as SseEvent, KeepAlive, Sse},
};
use std::convert::Infallible;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

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
            // SSE streaming — kept as route() since SSE doesn't map to OpenAPI
            .route("/activity", axum::routing::get(activity))
    }
}

async fn init(
    context: SystemContext,
    Json(body): Json<InitProject>,
) -> Result<(StatusCode, Json<ProjectResponse>), ProjectError> {
    let response = ProjectService::init(&context, &body).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

async fn summary(context: ProjectContext) -> Result<Json<BrainSummary>, ProjectError> {
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
        recent.sort_by_key(|b| std::cmp::Reverse(b.created_at()));
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

/// SSE activity stream — live events from the host-wide broadcast
/// channel. Unauthenticated (`ServerState` rather than `ProjectContext`)
/// because the channel is already shared across every brain on the
/// host and events are not brain-labeled on the wire. Brain attribution
/// on the feed is a separate revision.
async fn activity(
    State(state): State<ServerState>,
    headers: HeaderMap,
) -> Sse<impl tokio_stream::Stream<Item = Result<SseEvent, Infallible>>> {
    let last_event_id: Option<i64> = headers
        .get("last-event-id")
        .and_then(|header| header.to_str().ok())
        .and_then(|given_value| given_value.parse().ok());

    let rx = state.broadcast().subscribe();
    let mut last_sent = last_event_id.unwrap_or(0);

    let stream = BroadcastStream::new(rx).filter_map(move |result| match result {
        Ok(event) => {
            if event.sequence <= last_sent {
                return None;
            }
            last_sent = event.sequence;

            let json = serde_json::to_string(&event).unwrap_or_default();
            Some(Ok(SseEvent::default()
                .id(event.sequence.to_string())
                .data(json)))
        }
        Err(_) => None,
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}
