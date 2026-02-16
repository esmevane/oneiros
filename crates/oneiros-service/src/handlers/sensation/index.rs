use axum::Json;
use oneiros_model::{Description, Prompt, Sensation, SensationName};

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<Sensation>>, Error> {
    let sensations = ticket
        .db
        .list_sensations()?
        .into_iter()
        .map(|(name, desc, prompt)| Sensation {
            name: SensationName::new(name),
            description: Description::new(desc),
            prompt: Prompt::new(prompt),
        })
        .collect::<Vec<_>>();

    Ok(Json(sensations))
}
