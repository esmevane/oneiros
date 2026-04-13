use rusqlite::params;

use crate::*;

pub(crate) struct PersonaRepo<'a> {
    context: &'a ProjectContext,
}

impl<'a> PersonaRepo<'a> {
    pub(crate) fn new(context: &'a ProjectContext) -> Self {
        Self { context }
    }

    pub(crate) async fn get(&self, name: &PersonaName) -> Result<Option<Persona>, EventError> {
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

    pub(crate) async fn list(&self, filters: &SearchFilters) -> Result<Listed<Persona>, EventError> {
        let db = self.context.db()?;

        let total = {
            let mut stmt = db.prepare("SELECT COUNT(*) FROM personas")?;
            stmt.query_row([], |row| row.get::<_, usize>(0))?
        };

        let mut stmt = db.prepare(
            "SELECT name, description, prompt FROM personas ORDER BY name LIMIT ?1 OFFSET ?2",
        )?;

        let items = stmt
            .query_map(rusqlite::params![filters.limit, filters.offset], |row| {
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

        Ok(Listed::new(items, total))
    }
}
