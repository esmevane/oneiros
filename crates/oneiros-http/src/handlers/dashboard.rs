use askama::Template;
use axum::extract::{Query, State};
use axum::response::Html;
use oneiros_model::*;
use std::sync::Arc;

use crate::*;

#[derive(serde::Deserialize)]
pub(crate) struct DashboardParams {
    brain: Option<String>,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate<'a> {
    brain: &'a Brain,
    brains: &'a [Brain],
    agents: Vec<Agent>,
    agent_count: usize,
    cognition_count: usize,
    memory_count: usize,
    experience_count: usize,
    connection_count: usize,
    event_count: usize,
    recent_cognitions: Vec<Cognition>,
}

pub(crate) async fn handler(
    State(state): State<Arc<ServiceState>>,
    Query(params): Query<DashboardParams>,
) -> Result<Html<String>, Error> {
    let BrainResponses::BrainsListed(brains) = state.system_service()?.list_brains()? else {
        Err(Error::ProjectExtractionFailure)?
    };

    if brains.is_empty() {
        let html = "<html><body style='background:#0e0e10;color:#c8c8d0;font-family:monospace;padding:48px;text-align:center'>\
            <h1 style='color:#8b7ec8'>oneiros</h1><p>No brains found. Run <code>oneiros project init</code> to create one.</p>\
            </body></html>";
        return Ok(Html(html.to_string()));
    }

    let brain = match params.brain.as_deref() {
        Some(name) => brains
            .iter()
            .find(|b| b.name.as_str() == name)
            .unwrap_or(&brains[0]),
        None => &brains[0],
    };

    let BrainResponses::BrainSummarized(BrainSummary {
        agents,
        cognition_count,
        memory_count,
        experience_count,
        connection_count,
        event_count,
        recent_cognitions,
    }) = state.brain_summary(brain)?
    else {
        return Err(Error::ProjectSummaryFailure);
    };

    let agent_count = agents.len();

    let template = DashboardTemplate {
        brain,
        brains: &brains,
        agents,
        agent_count,
        cognition_count,
        memory_count,
        experience_count,
        connection_count,
        event_count,
        recent_cognitions,
    };

    Ok(Html(template.to_string()))
}
