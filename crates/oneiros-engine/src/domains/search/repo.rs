use rusqlite::{Connection, params};

use crate::*;

/// Search read model — queries only.
///
/// The search_index FTS5 table is populated by other domains' projections
/// (cognition, memory, experience, agent). This repo is read-only —
/// it has no handle(), no reset(), and no projection registration.
pub struct SearchRepo<'a> {
    conn: &'a Connection,
}

impl<'a> SearchRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Create the FTS5 virtual table.
    ///
    /// Called during initialization. Other domains' projections insert into
    /// this table via their own `handle()` calls. The columns are:
    /// - `kind`: entity type (e.g. "cognition", "memory")
    /// - `entity_id`: the entity's id
    /// - `content`: the searchable text
    /// - `agent`: the owning agent name (for filtering)
    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE VIRTUAL TABLE IF NOT EXISTS search_index
             USING fts5(kind, entity_id, content, agent)",
        )?;
        Ok(())
    }

    /// Full-text search across all indexed entities.
    ///
    /// When `agent` is provided the results are filtered to that agent.
    /// Results are ordered by FTS5 rank (best match first).
    pub fn search(
        &self,
        query: &str,
        agent: Option<&str>,
    ) -> Result<Vec<SearchResult>, StoreError> {
        match agent {
            Some(agent_filter) => {
                let mut stmt = self.conn.prepare(
                    "SELECT kind, entity_id, content, rank
                     FROM search_index
                     WHERE search_index MATCH ?1
                       AND agent = ?2
                     ORDER BY rank",
                )?;

                let results = stmt
                    .query_map(params![query, agent_filter], |row| {
                        Ok(SearchResult {
                            kind: row.get(0)?,
                            id: row.get(1)?,
                            content: row.get(2)?,
                            rank: row.get(3)?,
                        })
                    })?
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(results)
            }
            None => {
                let mut stmt = self.conn.prepare(
                    "SELECT kind, entity_id, content, rank
                     FROM search_index
                     WHERE search_index MATCH ?1
                     ORDER BY rank",
                )?;

                let results = stmt
                    .query_map(params![query], |row| {
                        Ok(SearchResult {
                            kind: row.get(0)?,
                            id: row.get(1)?,
                            content: row.get(2)?,
                            rank: row.get(3)?,
                        })
                    })?
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(results)
            }
        }
    }
}
