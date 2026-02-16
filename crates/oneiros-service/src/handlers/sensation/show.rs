use axum::{Json, extract::Path};
use oneiros_model::{Description, Prompt, Sensation, SensationName};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<SensationName>,
) -> Result<Json<Sensation>, Error> {
    let (name, desc, prompt) = ticket
        .db
        .get_sensation(&given_name)?
        .ok_or(NotFound::Sensation(given_name))?;

    Ok(Json(Sensation {
        name: SensationName::new(name),
        description: Description::new(desc),
        prompt: Prompt::new(prompt),
    }))
}
