use axum::{
    Json,
    extract::{Path, Query},
};
use oneiros_model::*;
use serde::Deserialize;

use super::collector::{DreamCollector, DreamConfig};
use crate::*;

#[derive(Debug, Deserialize, Default)]
pub(crate) struct DreamParams {
    recent_window: Option<usize>,
    dream_depth: Option<usize>,
    cognition_size: Option<usize>,
    recollection_level: Option<LevelName>,
    recollection_size: Option<usize>,
    experience_size: Option<usize>,
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
    Query(params): Query<DreamParams>,
) -> Result<Json<DreamContext>, Error> {
    let agent = ticket
        .db
        .get_agent(&agent_name)?
        .ok_or(NotFound::Agent(agent_name.clone()))?;

    let begun = Events::Dreaming(DreamingEvents::DreamBegun {
        agent: agent.name.clone(),
    });

    ticket.db.log_event(&begun, &[])?;

    let config = {
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
    };

    let context = DreamCollector::new(&ticket.db, config).collect(&agent)?;

    let complete = Events::Dreaming(DreamingEvents::DreamComplete {
        agent: context.agent.clone(),
    });

    ticket.db.log_event(&complete, &[])?;

    Ok(Json(context))
}
