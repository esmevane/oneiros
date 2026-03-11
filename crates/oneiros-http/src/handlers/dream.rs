use axum::{
    Json, Router,
    extract::{Path, Query},
    routing,
};
use oneiros_model::*;
use serde::Deserialize;

use crate::*;

pub(crate) fn router() -> Router<OneirosService> {
    Router::new().route("/{agent_name}", routing::post(create))
}

async fn create(
    ticket: OneirosContext,
    Path(agent): Path<AgentName>,
    Query(params): Query<DreamParams>,
) -> Result<Json<Response>, Error> {
    Ok(Json(ticket.dispatch(DreamingRequests::Dream(
        DreamRequest {
            agent,
            config: params.into(),
        },
    ))?))
}

#[derive(Debug, Deserialize, Default)]
struct DreamParams {
    pub recent_window: Option<usize>,
    pub dream_depth: Option<usize>,
    pub cognition_size: Option<usize>,
    pub recollection_level: Option<LevelName>,
    pub recollection_size: Option<usize>,
    pub experience_size: Option<usize>,
}

impl From<DreamParams> for DreamConfig {
    fn from(params: DreamParams) -> Self {
        let mut cfg = DreamConfig::default();
        if let Some(v) = params.recent_window {
            cfg.recent_window = v;
        }
        if let Some(v) = params.dream_depth {
            cfg.dream_depth = Some(v);
        }
        if let Some(v) = params.cognition_size {
            cfg.cognition_size = Some(v);
        }
        if let Some(v) = params.recollection_level {
            cfg.recollection_level = Some(v);
        }
        if let Some(v) = params.recollection_size {
            cfg.recollection_size = Some(v);
        }
        if let Some(v) = params.experience_size {
            cfg.experience_size = Some(v);
        }
        cfg
    }
}
