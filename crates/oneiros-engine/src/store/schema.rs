//! Database schema initialization.

use rusqlite::Connection;

use super::StoreError;

/// Initialize the event store schema.
pub fn initialize(conn: &Connection) -> Result<(), StoreError> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS events (
            id TEXT PRIMARY KEY,
            event_type TEXT NOT NULL,
            data TEXT NOT NULL,
            source TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL
        );

        -- Domain read model tables are created by domain repos.
        -- This file only creates the universal event store table.
        "
    )?;

    Ok(())
}
