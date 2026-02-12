use axum::{Json, extract::Path};
use oneiros_model::{Description, Prompt, Texture, TextureName};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(given_name): Path<TextureName>,
) -> Result<Json<Texture>, Error> {
    let (name, desc, prompt) = ticket
        .db
        .get_texture(&given_name)?
        .ok_or(NotFound::Texture(given_name))?;

    Ok(Json(Texture {
        name: TextureName::new(name),
        description: Description::new(desc),
        prompt: Prompt::new(prompt),
    }))
}
