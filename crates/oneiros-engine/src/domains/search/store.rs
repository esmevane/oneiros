use rusqlite::params;

use crate::*;

/// Search projection store — owns the FTS5 index lifecycle and write primitives.
///
/// Content-bearing domains (cognition, memory, experience, agent) call
/// [`SearchStore::index_expression`] and [`SearchStore::remove_by_ref`] from
/// their own event handlers. The search projection's `apply` is a no-op —
/// search owns the substrate, domains own the meaning of their events.
pub struct SearchStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> SearchStore<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "create virtual table if not exists search_index
             using fts5(
                 resource_ref unindexed,
                 kind unindexed,
                 content,
                 agent_id unindexed,
                 texture unindexed,
                 level unindexed,
                 sensation unindexed,
                 persona unindexed,
                 created_at unindexed
             )",
        )?;
        Ok(())
    }

    pub fn handle(&self, _event: &StoredEvent) -> Result<(), EventError> {
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("delete from search_index", [])?;
        Ok(())
    }

    // ── Write operations ─────────────────────────────────────────

    /// Index an expression row. Called by content-bearing domains from their
    /// own event handlers.
    pub fn index_expression(&self, expression: &Expression) -> Result<(), EventError> {
        let ref_json = serde_json::to_string(&expression.resource_ref)?;
        self.conn.execute(
            "insert into search_index (
                 resource_ref, kind, content,
                 agent_id, texture, level, sensation, persona, created_at
             ) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                ref_json,
                expression.kind.as_str(),
                expression.content.as_str(),
                expression
                    .agent
                    .as_ref()
                    .map(|a| a.to_string())
                    .unwrap_or_default(),
                expression
                    .texture
                    .as_ref()
                    .map(|t| t.to_string())
                    .unwrap_or_default(),
                expression
                    .level
                    .as_ref()
                    .map(|l| l.to_string())
                    .unwrap_or_default(),
                expression
                    .sensation
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                expression
                    .persona
                    .as_ref()
                    .map(|p| p.to_string())
                    .unwrap_or_default(),
                expression
                    .created_at
                    .as_ref()
                    .map(|t| t.as_string())
                    .unwrap_or_default(),
            ],
        )?;
        Ok(())
    }

    pub fn remove_by_ref(&self, resource_ref: &Ref) -> Result<(), EventError> {
        let ref_json = serde_json::to_string(resource_ref)?;
        self.conn.execute(
            "delete from search_index where resource_ref = ?1",
            params![ref_json],
        )?;
        Ok(())
    }

    pub fn remove_by_agent_id(&self, agent_id: &AgentId) -> Result<(), EventError> {
        self.conn.execute(
            "delete from search_index where agent_id = ?1 and kind = 'agent'",
            params![agent_id.to_string()],
        )?;
        Ok(())
    }
}
