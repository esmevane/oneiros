use axum::{Json, extract::Path};
use oneiros_model::{Description, Level, LevelName, Prompt};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<LevelName>,
) -> Result<Json<Level>, Error> {
    let (name, desc, prompt) = ticket
        .db
        .get_level(&given_name)?
        .ok_or(NotFound::Level(given_name))?;

    Ok(Json(Level {
        name: LevelName::new(name),
        description: Description::new(desc),
        prompt: Prompt::new(prompt),
    }))
}
