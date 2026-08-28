use rusqlite::params;

use crate::*;

pub(crate) struct PersonaRepo<'a> {
    scope: &'a Scope<AtBookmark>,
}

impl<'a> PersonaRepo<'a> {
    pub(crate) fn new(scope: &'a Scope<AtBookmark>) -> Self {
        Self { scope }
    }

    /// Eventually-consistent variant of [`get`]. Polls until the
    /// persona appears or the configured patience window expires.
    ///
    /// [`get`]: PersonaRepo::get
    pub(crate) async fn fetch(&self, name: &PersonaName) -> Result<Option<Persona>, EventError> {
        self.scope.config().fetch.eventual(|| self.get(name)).await
    }

    pub(crate) async fn get(&self, name: &PersonaName) -> Result<Option<Persona>, EventError> {
        let db = self.scope.bookmark_db().await?;

        let result = db.query_row(
            "SELECT name, description, prompt FROM personas WHERE name = ?1",
            params![name.to_string()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        );

        match result {
            Ok((name, description, prompt)) => Ok(Some(
                Persona::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )),
            Err(DbError::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn list(
        &self,
        filters: &SearchFilters,
    ) -> Result<Listed<Persona>, EventError> {
        let db = self.scope.bookmark_db().await?;

        let total = db.query_row("SELECT COUNT(*) FROM personas", [], |row| {
            row.get::<_, usize>(0)
        })?;

        let items = db
            .query_map(
                "SELECT name, description, prompt FROM personas ORDER BY name LIMIT ?1 OFFSET ?2",
                rusqlite::params![filters.limit, filters.offset],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                },
            )?
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
