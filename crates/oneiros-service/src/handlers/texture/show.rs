use axum::{Json, extract::Path};
use oneiros_model::{Link, Record, Texture, TextureName};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(identifier): Path<String>,
) -> Result<Json<Record<Texture>>, Error> {
    let by_name = ticket.db.get_texture(TextureName::new(&identifier))?;

    let texture = if let Some(t) = by_name {
        t
    } else if let Ok(link) = identifier.parse::<Link>() {
        ticket
            .db
            .get_texture_by_link(link.to_string())?
            .ok_or(NotFound::Texture(TextureName::new(&identifier)))?
    } else {
        return Err(NotFound::Texture(TextureName::new(&identifier)).into());
    };

    let record = Record::new(texture)?;
    Ok(Json(record))
}
