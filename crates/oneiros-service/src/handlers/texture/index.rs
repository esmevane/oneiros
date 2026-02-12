use axum::Json;
use oneiros_model::{Description, Prompt, Texture, TextureName};

use crate::*;

pub(crate) async fn handler(ticket: ActorContext) -> Result<Json<Vec<Texture>>, Error> {
    let textures = ticket
        .db
        .list_textures()?
        .into_iter()
        .map(|(name, desc, prompt)| Texture {
            name: TextureName::new(name),
            description: Description::new(desc),
            prompt: Prompt::new(prompt),
        })
        .collect::<Vec<_>>();

    Ok(Json(textures))
}
