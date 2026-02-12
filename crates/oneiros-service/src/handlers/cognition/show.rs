use axum::{Json, extract::Path};
use oneiros_model::{Cognition, CognitionId, Content, TextureName};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<CognitionId>,
) -> Result<Json<Cognition>, Error> {
    let (cid, agent_id, texture, content, created_at) = ticket
        .db
        .get_cognition(id.to_string())?
        .ok_or(NotFound::Cognition(id))?;

    Ok(Json(Cognition {
        id: cid.parse().unwrap_or_default(),
        agent_id: agent_id.parse().unwrap_or_default(),
        texture: TextureName::new(texture),
        content: Content::new(content),
        created_at: created_at.parse().unwrap_or_default(),
    }))
}
