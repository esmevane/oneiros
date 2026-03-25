use rusqlite::params;

use crate::*;

/// Search repo — async read queries over the FTS5 search index.
pub struct SearchRepo<'a> {
    context: &'a ProjectContext,
}

impl<'a> SearchRepo<'a> {
    pub fn new(context: &'a ProjectContext) -> Self {
        Self { context }
    }

    // ── Read queries ────────────────────────────────────────────

    pub async fn search(
        &self,
        query: &str,
        agent: Option<&AgentId>,
    ) -> Result<Vec<Expression>, EventError> {
        let db = self.context.db()?;
        let base =
            "select resource_ref, kind, content from search_index where search_index match ?1";

        match agent {
            Some(agent_filter) => {
                let sql = format!("{base} and agent = ?2 order by rank");
                let mut statement = db.prepare(&sql)?;
                Ok(statement
                    .query_map(params![query, agent_filter.to_string()], Self::map_row)?
                    .collect::<Result<Vec<_>, _>>()?)
            }
            None => {
                let sql = format!("{base} order by rank");
                let mut statement = db.prepare(&sql)?;
                Ok(statement
                    .query_map(params![query], Self::map_row)?
                    .collect::<Result<Vec<_>, _>>()?)
            }
        }
    }

    // ── Helpers ──────────────────────────────────────────────────

    fn map_row(row: &rusqlite::Row) -> rusqlite::Result<Expression> {
        let ref_json: String = row.get(0)?;
        let resource_ref: Ref = serde_json::from_str(&ref_json).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
        })?;

        Ok(Expression::builder()
            .resource_ref(resource_ref)
            .kind(row.get::<_, String>(1)?)
            .content(row.get::<_, String>(2)?)
            .build())
    }
}
