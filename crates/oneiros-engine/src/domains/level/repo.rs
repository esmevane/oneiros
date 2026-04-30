use rusqlite::params;

use crate::*;

pub struct LevelRepo<'a> {
    scope: &'a Scope<AtBookmark>,
}

impl<'a> LevelRepo<'a> {
    pub fn new(scope: &'a Scope<AtBookmark>) -> Self {
        Self { scope }
    }

    pub async fn get(&self, name: &LevelName) -> Result<Option<Level>, EventError> {
        let db = self.scope.bookmark_db().await?;
        let mut stmt =
            db.prepare("SELECT name, description, prompt FROM levels WHERE name = ?1")?;

        let result = stmt.query_row(params![name.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        });

        match result {
            Ok((name, description, prompt)) => Ok(Some(
                Level::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn list(&self, filters: &SearchFilters) -> Result<Listed<Level>, EventError> {
        let db = self.scope.bookmark_db().await?;

        let total = {
            let mut stmt = db.prepare("SELECT COUNT(*) FROM levels")?;
            stmt.query_row([], |row| row.get::<_, usize>(0))?
        };

        let mut stmt = db.prepare(
            "SELECT name, description, prompt FROM levels ORDER BY name LIMIT ?1 OFFSET ?2",
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
                Level::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build()
            })
            .collect();

        Ok(Listed::new(items, total))
    }
}
