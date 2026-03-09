use axum::{Json, extract::Query};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Query(request): Query<ListExperiencesRequest>,
) -> Result<Json<ExperienceResponses>, Error> {
    let response = ticket
        .service()
        .dispatch_experience(ExperienceRequests::ListExperiences(request))?;

    Ok(Json(response))
}
