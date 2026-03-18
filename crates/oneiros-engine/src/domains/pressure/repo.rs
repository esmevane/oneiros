//! Pressure repo — cross-domain derived state.
//!
//! Pressure is computed from queries across cognition, memory, and events.
//! The projection gathers inputs and computes urgency. The repo handles
//! both the pressure table and the cross-domain queries.

use rusqlite::{Connection, params};
use chrono::Utc;

use crate::store::{StoredEvent, StoreError};

use super::model::Pressure;

pub struct PressureRepo<'a> {
    conn: &'a Connection,
}

impl<'a> PressureRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Recompute pressure for all agents on any event.
    ///
    /// This is a simplified version — the real system has 4 heuristic
    /// gauges (introspect, catharsis, recollect, retrospect). For the
    /// engine POC, we compute a simple metric based on recent activity.
    pub fn handle(&self, _event: &StoredEvent) -> Result<(), StoreError> {
        // Get all agents
        let mut stmt = self.conn.prepare("SELECT name FROM agents")?;
        let agents: Vec<String> = stmt.query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;

        // Get all urges
        let mut urge_stmt = self.conn.prepare("SELECT name FROM urges")?;
        let urges: Vec<String> = urge_stmt.query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;

        let now = Utc::now().to_rfc3339();

        for agent in &agents {
            for urge in &urges {
                // Simple heuristic: count cognitions as activity proxy
                let cognition_count: i64 = self.conn.query_row(
                    "SELECT COUNT(*) FROM cognitions WHERE agent_id = ?1",
                    params![agent],
                    |row| row.get(0),
                ).unwrap_or(0);

                // More cognitions = lower pressure to introspect (already active)
                // Fewer cognitions = higher pressure
                let percent = ((100.0 - (cognition_count as f64 * 10.0).min(100.0)) as u8).max(0);

                self.conn.execute(
                    "INSERT OR REPLACE INTO pressures (agent, urge, percent, updated_at) VALUES (?1, ?2, ?3, ?4)",
                    params![agent, urge, percent, now],
                )?;
            }
        }

        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM pressures", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS pressures (
                agent TEXT NOT NULL,
                urge TEXT NOT NULL,
                percent INTEGER NOT NULL DEFAULT 0,
                updated_at TEXT NOT NULL,
                PRIMARY KEY (agent, urge)
            )"
        )?;
        Ok(())
    }

    pub fn get(&self, agent: &str) -> Result<Vec<Pressure>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT agent, urge, percent, updated_at FROM pressures WHERE agent = ?1",
        )?;

        let pressures = stmt.query_map(params![agent], |row| {
            Ok(Pressure {
                agent: row.get(0)?,
                urge: row.get(1)?,
                percent: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(pressures)
    }

    pub fn list(&self) -> Result<Vec<Pressure>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT agent, urge, percent, updated_at FROM pressures ORDER BY agent, urge",
        )?;

        let pressures = stmt.query_map([], |row| {
            Ok(Pressure {
                agent: row.get(0)?,
                urge: row.get(1)?,
                percent: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(pressures)
    }
}
