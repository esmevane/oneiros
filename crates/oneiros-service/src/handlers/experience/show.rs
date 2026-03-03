use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
) -> Result<Json<Experience>, Error> {
    let experience = ticket.service().get_experience(&id)?;

    Ok(Json(experience))
}
