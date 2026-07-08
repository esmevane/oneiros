use rusqlite::params;

use crate::*;

pub(crate) struct NatureRepo<'a> {
    scope: &'a Scope<AtBookmark>,
}

impl<'a> NatureRepo<'a> {
    pub(crate) fn new(scope: &'a Scope<AtBookmark>) -> Self {
        Self { scope }
    }

    /// Eventually-consistent variant of [`get`]. Polls until the
    /// nature appears or the configured patience window expires.
    ///
    /// [`get`]: NatureRepo::get
    pub(crate) async fn fetch(&self, name: &NatureName) -> Result<Option<Nature>, EventError> {
        self.scope.config().fetch.eventual(|| self.get(name)).await
    }

    pub(crate) async fn get(&self, name: &NatureName) -> Result<Option<Nature>, EventError> {
        let db = self.scope.bookmark_db().await?;

        let result = db.query_row(
            "SELECT name, description, prompt FROM natures WHERE name = ?1",
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
                Nature::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )),
            Err(DbError::Rusqlite(rusqlite::Error::QueryReturnedNoRows)) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn list(&self, filters: &SearchFilters) -> Result<Listed<Nature>, EventError> {
        let db = self.scope.bookmark_db().await?;

        let total = db.query_row("SELECT COUNT(*) FROM natures", [], |row| {
            row.get::<_, usize>(0)
        })?;

        let items = db
            .query_map(
                "SELECT name, description, prompt FROM natures ORDER BY name LIMIT ?1 OFFSET ?2",
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
