use axum::{Json, extract::Path};
use oneiros_model::{
    Content, Experience, ExperienceId, Label, RecordKind, RecordRef, SensationName,
};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
) -> Result<Json<Experience>, Error> {
    let (exp_id, agent_id, sensation, description, created_at) = ticket
        .db
        .get_experience(id.to_string())?
        .ok_or(NotFound::Experience(id))?;

    let refs = ticket.db.list_experience_refs(&exp_id)?;

    let record_refs = refs
        .into_iter()
        .map(|(_, record_id, record_kind, role, _)| RecordRef {
            id: record_id.parse().unwrap_or_default(),
            kind: record_kind.parse().unwrap_or(RecordKind::Storage),
            role: role.map(Label::new),
        })
        .collect();

    Ok(Json(Experience {
        id: exp_id.parse().unwrap_or_default(),
        agent_id: agent_id.parse().unwrap_or_default(),
        sensation: SensationName::new(sensation),
        description: Content::new(description),
        refs: record_refs,
        created_at: created_at.parse().unwrap_or_default(),
    }))
}
