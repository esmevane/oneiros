use rusqlite::Connection;
use std::path::Path;

use crate::Result;

pub(crate) struct Database {
    conn: Connection,
}

impl Database {
    pub(crate) fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    pub(crate) fn event_count(&self) -> Result<usize> {
        let count: i64 = self
            .conn
            .query_row("select count(*) from events", [], |row| row.get(0))?;
        Ok(count as usize)
    }
}
