use rusqlite::params;

use crate::*;

pub struct NatureRepo<'a> {
    scope: &'a Scope<AtBookmark>,
}

impl<'a> NatureRepo<'a> {
    pub fn new(scope: &'a Scope<AtBookmark>) -> Self {
        Self { scope }
    }

    pub async fn get(&self, name: &NatureName) -> Result<Option<Nature>, EventError> {
        let db = self.scope.bookmark_db()?;
        let mut stmt =
            db.prepare("SELECT name, description, prompt FROM natures WHERE name = ?1")?;

        let result = stmt.query_row(params![name.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        });

        match result {
            Ok((name, description, prompt)) => Ok(Some(
                Nature::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn list(&self, filters: &SearchFilters) -> Result<Listed<Nature>, EventError> {
        let db = self.scope.bookmark_db()?;

        let total = {
            let mut stmt = db.prepare("SELECT COUNT(*) FROM natures")?;
            stmt.query_row([], |row| row.get::<_, usize>(0))?
        };

        let mut stmt = db.prepare(
            "SELECT name, description, prompt FROM natures ORDER BY name LIMIT ?1 OFFSET ?2",
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
                Nature::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build()
            })
            .collect();

        Ok(Listed::new(items, total))
    }
}
