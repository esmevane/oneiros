use axum::{Json, extract::Path, http::StatusCode};
use oneiros_model::{
    Events, Experience, ExperienceEvents, ExperienceId, Label, RecordKind, RecordRef, SensationName,
};
use oneiros_protocol::UpdateExperienceDescriptionRequest;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
    Json(request): Json<UpdateExperienceDescriptionRequest>,
) -> Result<(StatusCode, Json<Experience>), Error> {
    // Validate that the experience exists.
    let (exp_id, agent_id, sensation, _, created_at) = ticket
        .db
        .get_experience(id.to_string())?
        .ok_or(NotFound::Experience(id))?;

    let event = Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated {
        experience_id: id,
        description: request.description.clone(),
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
        description: request.description,
        refs: record_refs,
        created_at: created_at.parse().unwrap_or_default(),
    };

    Ok((StatusCode::OK, Json(experience)))
}
