use rusqlite::params;

use crate::*;

pub(crate) struct SliceRepo<'a> {
    scope: &'a Scope<AtBookmark>,
}

impl<'a> SliceRepo<'a> {
    pub(crate) fn new(scope: &'a Scope<AtBookmark>) -> Self {
        Self { scope }
    }

    pub(crate) async fn fetch(&self, name: &SliceName) -> Result<Option<Slice>, EventError> {
        self.scope
            .config()
            .fetch
            .eventual(|| self.get(name))
            .await
    }

    pub(crate) async fn get(&self, name: &SliceName) -> Result<Option<Slice>, EventError> {
        let db = BookmarkDb::open(self.scope).await?;
        let mut stmt = db.prepare(
            "SELECT name, lens_expr, event_count, created_at FROM slices WHERE name = ?1",
        )?;
        let result = stmt.query_row(params![name.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, String>(3)?,
            ))
        });
        match result {
            Ok((name, lens_expr, event_count, created_at)) => Ok(Some(
                Slice::builder()
                    .name(name)
                    .lens_expr(lens_expr)
                    .event_count(event_count as u64)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn list(&self) -> Result<Listed<Slice>, EventError> {
        let db = BookmarkDb::open(self.scope).await?;
        let total = {
            let mut stmt = db.prepare("SELECT COUNT(*) FROM slices")?;
            stmt.query_row([], |row| row.get::<_, usize>(0))?
        };
        let mut stmt = db.prepare(
            "SELECT name, lens_expr, event_count, created_at FROM slices ORDER BY created_at DESC",
        )?;
        let items: Vec<Slice> = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|(name, lens_expr, event_count, created_at)| {
                Ok(Slice::builder()
                    .name(name)
                    .lens_expr(lens_expr)
                    .event_count(event_count as u64)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build())
            })
            .collect::<Result<Vec<_>, EventError>>()?;

        Ok(Listed::new(items, total))
    }
}
