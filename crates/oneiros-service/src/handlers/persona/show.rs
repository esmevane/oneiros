use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(key): Path<Key<PersonaName, PersonaLink>>,
) -> Result<Json<Record<Persona>>, Error> {
    let persona = ticket
        .db
        .get_persona_by_key(&key)?
        .ok_or(NotFound::Persona(key))?;

    let record = Record::new(persona)?;
    Ok(Json(record))
}
