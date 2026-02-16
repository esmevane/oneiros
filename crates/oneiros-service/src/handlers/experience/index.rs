use axum::{Json, extract::Query};
use oneiros_model::{AgentName, Content, Experience, Label, RecordKind, RecordRef, SensationName};
use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
pub(crate) struct ListParams {
    pub agent: Option<AgentName>,
    pub sensation: Option<SensationName>,
}

fn to_experience(
    row: (String, String, String, String, String),
    refs: Vec<(String, String, String, Option<String>, String)>,
) -> Experience {
    let (id, agent_id, sensation, description, created_at) = row;

    let record_refs = refs
        .into_iter()
        .map(|(_, record_id, record_kind, role, _)| RecordRef {
            id: record_id.parse().unwrap_or_default(),
            kind: record_kind.parse().unwrap_or(RecordKind::Storage),
            role: role.map(Label::new),
        })
        .collect();

    Experience {
        id: id.parse().unwrap_or_default(),
        agent_id: agent_id.parse().unwrap_or_default(),
        sensation: SensationName::new(sensation),
        description: Content::new(description),
        refs: record_refs,
        created_at: created_at.parse().unwrap_or_default(),
    }
}

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<Experience>>, Error> {
    let rows = match (params.agent, params.sensation) {
        (Some(agent_name), Some(sensation)) => {
            let (id, _, _, _, _) = ticket
                .db
                .get_agent(&agent_name)?
                .ok_or(NotFound::Agent(agent_name))?;

            ticket
                .db
                .get_sensation(&sensation)?
                .ok_or(NotFound::Sensation(sensation.clone()))?;

            ticket
                .db
                .list_experiences_by_agent(&id)?
                .into_iter()
                .filter(|(_, _, exp_sensation, _, _)| exp_sensation == &sensation.to_string())
                .collect()
        }
        (Some(agent_name), None) => {
            let (id, _, _, _, _) = ticket
                .db
                .get_agent(&agent_name)?
                .ok_or(NotFound::Agent(agent_name))?;

            ticket.db.list_experiences_by_agent(&id)?
        }
        (None, Some(sensation)) => {
            ticket
                .db
                .get_sensation(&sensation)?
                .ok_or(NotFound::Sensation(sensation.clone()))?;

            ticket.db.list_experiences_by_sensation(&sensation)?
        }
        (None, None) => ticket.db.list_experiences()?,
    };

    let mut experiences = Vec::new();
    for row in rows {
        let experience_id = &row.0;
        let refs = ticket.db.list_experience_refs(experience_id)?;
        experiences.push(to_experience(row, refs));
    }

    Ok(Json(experiences))
}
