use rusqlite::{Connection, params};

use crate::*;

pub struct SearchRepo<'a> {
    conn: &'a Connection,
}

impl<'a> SearchRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "create virtual table if not exists search_index
             using fts5(resource_ref, kind, content, agent)",
        )?;
        Ok(())
    }

    pub fn search(
        &self,
        query: &str,
        agent: Option<&AgentId>,
    ) -> Result<Vec<SearchResult>, EventError> {
        let base =
            "select resource_ref, kind, content from search_index where search_index match ?1";

        match agent {
            Some(agent_filter) => {
                let sql = format!("{base} and agent = ?2 order by rank");
                let mut statement = self.conn.prepare(&sql)?;
                Ok(statement
                    .query_map(params![query, agent_filter.to_string()], Self::map_row)?
                    .collect::<Result<Vec<_>, _>>()?)
            }
            None => {
                let sql = format!("{base} order by rank");
                let mut statement = self.conn.prepare(&sql)?;
                Ok(statement
                    .query_map(params![query], Self::map_row)?
                    .collect::<Result<Vec<_>, _>>()?)
            }
        }
    }

    fn map_row(row: &rusqlite::Row) -> rusqlite::Result<SearchResult> {
        let ref_json: String = row.get(0)?;
        let resource_ref: Ref = serde_json::from_str(&ref_json).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
        })?;

        Ok(SearchResult::builder()
            .resource_ref(resource_ref)
            .kind(row.get::<_, String>(1)?)
            .content(row.get::<_, String>(2)?)
            .build())
    }
}
