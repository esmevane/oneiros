use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(identifier): Path<String>,
) -> Result<Json<Record<Identity<ExperienceId, Experience>>>, Error> {
    let by_id = ticket.db.get_experience(&identifier)?;

    let experience = if let Some(e) = by_id {
        e
    } else if let Ok(link) = identifier.parse::<Link>() {
        ticket
            .db
            .get_experience_by_link(link.to_string())?
            .ok_or(NotFound::Experience(identifier))?
    } else {
        return Err(NotFound::Experience(identifier).into());
    };

    let record = Record::new(experience)?;
    Ok(Json(record))
}
