use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::{
    Content, Events, Experience, ExperienceEvents, ExperienceId, Label, RecordKind, RecordRef,
    SensationName,
};
use oneiros_protocol::AddExperienceRefRequest;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
    Json(request): Json<AddExperienceRefRequest>,
) -> Result<(StatusCode, Json<Experience>), Error> {
    // Validate that the experience exists.
    let (exp_id, agent_id, sensation, description, created_at) = ticket
        .db
        .get_experience(id.to_string())?
        .ok_or(NotFound::Experience(id))?;

    let record_ref = RecordRef {
        id: request.record_id,
        kind: request.record_kind,
        role: request.role,
    };

    let event = Events::Experience(ExperienceEvents::ExperienceRefAdded {
        experience_id: id,
        record_ref: record_ref.clone(),
    });

    ticket
        .db
        .log_event(&event, projections::BRAIN_PROJECTIONS)?;

    // Fetch all refs to build the full Experience.
    let refs = ticket.db.list_experience_refs(&exp_id)?;

    let record_refs = refs
        .into_iter()
        .map(|(_, record_id, record_kind, role, _)| RecordRef {
            id: record_id.parse().unwrap_or_default(),
            kind: record_kind.parse().unwrap_or(RecordKind::Storage),
            role: role.map(Label::new),
        })
        .collect();

    let experience = Experience {
        id: exp_id.parse().unwrap_or_default(),
        agent_id: agent_id.parse().unwrap_or_default(),
        sensation: SensationName::new(sensation),
        description: Content::new(description),
        refs: record_refs,
        created_at: created_at.parse().unwrap_or_default(),
    };

    Ok((StatusCode::OK, Json(experience)))
}
