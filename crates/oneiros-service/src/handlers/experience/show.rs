use axum::{Json, extract::Path};
use oneiros_model::{Experience, ExperienceId};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
) -> Result<Json<Experience>, Error> {
    let experience = ticket
        .db
        .get_experience(id.to_string())?
        .ok_or(NotFound::Experience(id))?;

    Ok(Json(experience))
}
