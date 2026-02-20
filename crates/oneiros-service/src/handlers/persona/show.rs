use axum::{Json, extract::Path};
use oneiros_model::{Link, Persona, PersonaName, Record};

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(identifier): Path<String>,
) -> Result<Json<Record<Persona>>, Error> {
    let by_name = ticket.db.get_persona(PersonaName::new(&identifier))?;

    let persona = if let Some(p) = by_name {
        p
    } else if let Ok(link) = identifier.parse::<Link>() {
        ticket
            .db
            .get_persona_by_link(link.to_string())?
            .ok_or(NotFound::Persona(PersonaName::new(&identifier)))?
    } else {
        return Err(NotFound::Persona(PersonaName::new(&identifier)).into());
    };

    let record = Record::new(persona)?;
    Ok(Json(record))
}
