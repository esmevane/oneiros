use rusqlite::params;

use crate::*;

pub struct PersonaRepo<'a> {
    context: &'a ProjectContext,
}

impl<'a> PersonaRepo<'a> {
    pub fn new(context: &'a ProjectContext) -> Self {
        Self { context }
    }

    pub async fn get(&self, name: &PersonaName) -> Result<Option<Persona>, EventError> {
        let db = self.context.db()?;
        let mut stmt =
            db.prepare("SELECT name, description, prompt FROM personas WHERE name = ?1")?;

        let result = stmt.query_row(params![name.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        });

        match result {
            Ok((name, description, prompt)) => Ok(Some(
                Persona::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn list(&self) -> Result<Vec<Persona>, EventError> {
        let db = self.context.db()?;
        let mut stmt =
            db.prepare("SELECT name, description, prompt FROM personas ORDER BY name")?;

        let personas = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })?
            .collect::<Result<Vec<(String, String, String)>, _>>()?
            .into_iter()
            .map(|(name, description, prompt)| {
                Persona::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build()
            })
            .collect();

        Ok(personas)
    }
}
