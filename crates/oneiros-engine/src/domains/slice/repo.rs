use rusqlite::params;

use crate::*;

pub(crate) struct SliceRepo<'a> {
    scope: &'a Scope<AtHost>,
}

impl<'a> SliceRepo<'a> {
    pub(crate) fn new(scope: &'a Scope<AtHost>) -> Self {
        Self { scope }
    }

    pub(crate) async fn fetch(&self, name: &SliceName) -> Result<Option<Slice>, EventError> {
        self.scope.config().fetch.eventual(|| self.get(name)).await
    }

    pub(crate) async fn get(&self, name: &SliceName) -> Result<Option<Slice>, EventError> {
        let db = HostDb::open(self.scope).await?;
        let mut stmt = db.prepare(
            "SELECT s.name, s.lens_expr, s.created_at,
                    COALESCE(c.event_count, 0) AS event_count
             FROM slices s
             LEFT JOIN (
                 SELECT slice_name, COUNT(*) AS event_count
                 FROM slice_chronicle
                 GROUP BY slice_name
             ) c ON c.slice_name = s.name
             WHERE s.name = ?1",
        )?;
        let result = stmt.query_row(params![name.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
            ))
        });
        match result {
            Ok((name, lens_expr, created_at, event_count)) => Ok(Some(
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

    /// Fetch just the lens expression for a named slice.
    pub(crate) async fn get_lens_expression(
        &self,
        name: &SliceName,
    ) -> Result<Option<String>, EventError> {
        let db = HostDb::open(self.scope).await?;
        let mut stmt = db.prepare("SELECT lens_expr FROM slices WHERE name = ?1")?;
        let result = stmt.query_row(rusqlite::params![name.to_string()], |row| {
            row.get::<_, String>(0)
        });
        match result {
            Ok(expr) => Ok(Some(expr)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn list(&self) -> Result<Listed<Slice>, EventError> {
        let db = HostDb::open(self.scope).await?;
        let total = {
            let mut stmt = db.prepare("SELECT COUNT(*) FROM slices")?;
            stmt.query_row([], |row| row.get::<_, usize>(0))?
        };
        let mut stmt = db.prepare(
            "SELECT s.name, s.lens_expr, s.created_at,
                    COALESCE(c.event_count, 0) AS event_count
             FROM slices s
             LEFT JOIN (
                 SELECT slice_name, COUNT(*) AS event_count
                 FROM slice_chronicle
                 GROUP BY slice_name
             ) c ON c.slice_name = s.name
             ORDER BY s.created_at DESC",
        )?;
        let items: Vec<Slice> = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|(name, lens_expr, created_at, event_count)| {
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
