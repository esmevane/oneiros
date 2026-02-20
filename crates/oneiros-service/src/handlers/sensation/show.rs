use axum::{Json, extract::Path};
use oneiros_model::{Link, Record, Sensation, SensationName};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(identifier): Path<String>,
) -> Result<Json<Record<Sensation>>, Error> {
    let by_name = ticket.db.get_sensation(SensationName::new(&identifier))?;

    let sensation = if let Some(s) = by_name {
        s
    } else if let Ok(link) = identifier.parse::<Link>() {
        ticket
            .db
            .get_sensation_by_link(link.to_string())?
            .ok_or(NotFound::Sensation(SensationName::new(&identifier)))?
    } else {
        return Err(NotFound::Sensation(SensationName::new(&identifier)).into());
    };

    let record = Record::new(sensation)?;
    Ok(Json(record))
}
