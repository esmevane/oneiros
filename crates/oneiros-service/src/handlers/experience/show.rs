use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(key): Path<Key<ExperienceId, ExperienceLink>>,
) -> Result<Json<Record<Identity<ExperienceId, Experience>>>, Error> {
    let experience = ticket
        .db
        .get_experience_by_key(&key)?
        .ok_or(NotFound::Experience(key))?;

    let record = Record::new(experience)?;
    Ok(Json(record))
}
