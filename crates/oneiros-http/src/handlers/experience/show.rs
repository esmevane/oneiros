use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(id): Path<ExperienceId>,
) -> Result<Json<ExperienceResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_experience(ExperienceRequests::GetExperience(GetExperienceRequest {
            id,
        }))?;

    Ok(Json(response))
}
