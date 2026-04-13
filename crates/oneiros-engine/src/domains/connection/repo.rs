use rusqlite::params;

use crate::*;

/// Connection read model — async queries over the projection read model.
pub(crate) struct ConnectionRepo<'a> {
    context: &'a ProjectContext,
}

impl<'a> ConnectionRepo<'a> {
    pub(crate) fn new(context: &'a ProjectContext) -> Self {
        Self { context }
    }

    // ── Read queries ────────────────────────────────────────────

    pub(crate) async fn get(&self, id: &ConnectionId) -> Result<Option<Connection>, EventError> {
        let db = self.context.db()?;
        let mut stmt = db.prepare(
            "SELECT id, from_ref, to_ref, nature, created_at
             FROM connections WHERE id = ?1",
        )?;

        let result = stmt.query_row(params![id.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        });

        match result {
            Ok((id, from_ref, to_ref, nature, created_at)) => Ok(Some(
                Connection::builder()
                    .id(id.parse()?)
                    .from_ref(serde_json::from_str(&from_ref)?)
                    .to_ref(serde_json::from_str(&to_ref)?)
                    .nature(nature)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn list(
        &self,
        entity_ref: Option<&str>,
        filters: &SearchFilters,
    ) -> Result<Listed<Connection>, EventError> {
        let db = self.context.db()?;

        // Build WHERE clause from filters.
        let mut conditions = Vec::new();
        let mut bind_values: Vec<String> = Vec::new();

        if let Some(e) = entity_ref {
            bind_values.push(e.to_string());
            let idx = bind_values.len();
            conditions.push(format!("(from_ref = ?{idx} OR to_ref = ?{idx})"));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", conditions.join(" AND "))
        };

        // Count total matching rows.
        let count_sql = format!("SELECT COUNT(*) FROM connections{where_clause}");
        let total = {
            let mut stmt = db.prepare(&count_sql)?;
            let params: Vec<&dyn rusqlite::ToSql> = bind_values
                .iter()
                .map(|v| v as &dyn rusqlite::ToSql)
                .collect();
            stmt.query_row(&*params, |row| row.get::<_, usize>(0))?
        };

        // Fetch the bounded window.
        let select_sql = format!(
            "SELECT id, from_ref, to_ref, nature, created_at
             FROM connections{where_clause}
             ORDER BY created_at DESC
             LIMIT ?{} OFFSET ?{}",
            bind_values.len() + 1,
            bind_values.len() + 2,
        );

        let mut stmt = db.prepare(&select_sql)?;

        let map_row = |row: &rusqlite::Row<'_>| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        };

        let mut all_params: Vec<Box<dyn rusqlite::ToSql>> = bind_values
            .into_iter()
            .map(|v| Box::new(v) as Box<dyn rusqlite::ToSql>)
            .collect();
        all_params.push(Box::new(filters.limit));
        all_params.push(Box::new(filters.offset));

        let param_refs: Vec<&dyn rusqlite::ToSql> = all_params.iter().map(|p| p.as_ref()).collect();

        let raw = stmt
            .query_map(&*param_refs, map_row)?
            .collect::<Result<Vec<_>, _>>()?;

        let mut connections = vec![];
        for (id, from_ref, to_ref, nature, created_at) in raw {
            connections.push(
                Connection::builder()
                    .id(id.parse()?)
                    .from_ref(serde_json::from_str(&from_ref)?)
                    .to_ref(serde_json::from_str(&to_ref)?)
                    .nature(nature)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            );
        }

        Ok(Listed::new(connections, total))
    }
}
