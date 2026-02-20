use axum::{Json, extract::Path};
use oneiros_model::{Level, LevelName, Link, Record};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(identifier): Path<String>,
) -> Result<Json<Record<Level>>, Error> {
    let by_name = ticket.db.get_level(LevelName::new(&identifier))?;

    let level = if let Some(l) = by_name {
        l
    } else if let Ok(link) = identifier.parse::<Link>() {
        ticket
            .db
            .get_level_by_link(link.to_string())?
            .ok_or(NotFound::Level(LevelName::new(&identifier)))?
    } else {
        return Err(NotFound::Level(LevelName::new(&identifier)).into());
    };

    let record = Record::new(level)?;
    Ok(Json(record))
}
