use axum::{Json, extract::Path};
use oneiros_model::{Link, Nature, NatureName, Record};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(identifier): Path<String>,
) -> Result<Json<Record<Nature>>, Error> {
    let by_name = ticket.db.get_nature(NatureName::new(&identifier))?;

    let nature = if let Some(n) = by_name {
        n
    } else if let Ok(link) = identifier.parse::<Link>() {
        ticket
            .db
            .get_nature_by_link(link.to_string())?
            .ok_or(NotFound::Nature(NatureName::new(&identifier)))?
    } else {
        return Err(NotFound::Nature(NatureName::new(&identifier)).into());
    };

    let record = Record::new(nature)?;
    Ok(Json(record))
}
