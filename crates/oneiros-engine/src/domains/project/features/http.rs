use std::convert::Infallible;

use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::sse::{Event as SseEvent, KeepAlive, Sse};
use axum::{Json, Router, routing};
use serde::Serialize;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

use crate::*;

pub struct ProjectRouter;

impl ProjectRouter {
    pub fn routes(&self) -> Router<ProjectContext> {
        Router::new()
            .route("/summary", routing::get(summary))
            .route("/activity", routing::get(activity))
            .route("/health", routing::get(health))
    }
}

/// Brain summary — counts and recent data for the dashboard.
#[derive(Debug, Clone, Serialize)]
pub struct BrainSummary {
    pub agents: Vec<Agent>,
    pub agent_count: usize,
    pub cognition_count: usize,
    pub memory_count: usize,
    pub experience_count: usize,
    pub connection_count: usize,
    pub event_count: usize,
    pub recent_cognitions: Vec<Cognition>,
}

async fn summary(
    State(context): State<ProjectContext>,
) -> Result<Json<BrainSummary>, ProjectError> {
    let summary = context.with_db(|conn| {
        let agents = AgentRepo::new(conn).list().unwrap_or_default();
        let agent_count = agents.len();

        let cognitions = CognitionRepo::new(conn)
            .list(None, None)
            .unwrap_or_default();
        let cognition_count = cognitions.len();

        // Get recent cognitions (last 30, newest first)
        let recent_cognitions = {
            let mut recent = cognitions;
            recent.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            recent.truncate(30);
            recent
        };

        let memory_count = MemoryRepo::new(conn).list(None).unwrap_or_default().len();
        let experience_count = ExperienceRepo::new(conn)
            .list(None)
            .unwrap_or_default()
            .len();
        let connection_count = ConnectionRepo::new(conn)
            .list(None)
            .unwrap_or_default()
            .len();

        let event_count = conn
            .query_row("SELECT COUNT(*) FROM events", [], |row| {
                row.get::<_, i64>(0)
            })
            .unwrap_or(0) as usize;

        BrainSummary {
            agents,
            agent_count,
            cognition_count,
            memory_count,
            experience_count,
            connection_count,
            event_count,
            recent_cognitions,
        }
    });

    Ok(Json(summary))
}

/// SSE activity stream — live events from the broadcast channel.
async fn activity(
    State(context): State<ProjectContext>,
    headers: HeaderMap,
) -> Sse<impl tokio_stream::Stream<Item = Result<SseEvent, Infallible>>> {
    let last_event_id: Option<i64> = headers
        .get("last-event-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok());

    let rx = context.subscribe();
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

/// Simple health check endpoint.
async fn health() -> &'static str {
    "ok"
}
