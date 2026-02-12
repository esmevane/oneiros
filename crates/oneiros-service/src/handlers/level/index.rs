use axum::Json;
use oneiros_model::{Description, Level, LevelName, Prompt};

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<Level>>, Error> {
    let levels = ticket
        .db
        .list_levels()?
        .into_iter()
        .map(|(name, desc, prompt)| Level {
            name: LevelName::new(name),
            description: Description::new(desc),
            prompt: Prompt::new(prompt),
        })
        .collect::<Vec<_>>();

    Ok(Json(levels))
}
