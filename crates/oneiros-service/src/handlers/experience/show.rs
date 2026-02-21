use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
) -> Result<Json<ExperienceRecord>, Error> {
    let experience = ticket
        .db
        .get_experience(id.to_string())?
        .ok_or(NotFound::Experience(id))?;

    Ok(Json(experience))
}
