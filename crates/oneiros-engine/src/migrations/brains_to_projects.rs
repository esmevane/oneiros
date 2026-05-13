use crate::Config;

use super::{Migration, MigrationError};

/// Rename the host-DB tables and columns that still carry the legacy
/// `brain*` vocabulary onto the current `project*` names. All renames
/// run in a single transaction so the host DB is either fully migrated
/// or untouched.
pub(crate) struct BrainsToProjects;

impl Migration for BrainsToProjects {
    fn name(&self) -> &'static str {
        "brains → projects (schema renames)"
    }

    fn is_required(&self, config: &Config) -> Result<bool, MigrationError> {
        let host_db = config.platform().host_db_path();
        if !host_db.exists() {
            // Pristine data-dir or freshly initialized — nothing to rename.
            return Ok(false);
        }

        let conn = rusqlite::Connection::open(&host_db)?;
        let needs_rename = table_exists(&conn, "brains")?
            || column_exists(&conn, "bookmarks", "brain")?
            || column_exists(&conn, "follows", "brain")?
            || column_exists(&conn, "tickets", "brain_name")?
            || column_exists(&conn, "tickets", "brain_id")?;

        Ok(needs_rename)
    }

    fn apply(&self, config: &Config) -> Result<(), MigrationError> {
        let host_db = config.platform().host_db_path();
        let mut conn = rusqlite::Connection::open(&host_db)?;
        let tx = conn.transaction()?;

        if table_exists(&tx, "brains")? && !table_exists(&tx, "projects")? {
            tx.execute_batch("ALTER TABLE brains RENAME TO projects")?;
        }

        if column_exists(&tx, "bookmarks", "brain")? && !column_exists(&tx, "bookmarks", "project")?
        {
            tx.execute_batch("ALTER TABLE bookmarks RENAME COLUMN brain TO project")?;
        }

        if column_exists(&tx, "follows", "brain")? && !column_exists(&tx, "follows", "project")? {
            tx.execute_batch("ALTER TABLE follows RENAME COLUMN brain TO project")?;
        }

        if column_exists(&tx, "tickets", "brain_name")?
            && !column_exists(&tx, "tickets", "project_name")?
        {
            tx.execute_batch("ALTER TABLE tickets RENAME COLUMN brain_name TO project_name")?;
        }

        if column_exists(&tx, "tickets", "brain_id")?
            && !column_exists(&tx, "tickets", "project_id")?
        {
            tx.execute_batch("ALTER TABLE tickets RENAME COLUMN brain_id TO project_id")?;
        }

        tx.commit()?;
        Ok(())
    }
}

fn table_exists(conn: &rusqlite::Connection, name: &str) -> Result<bool, rusqlite::Error> {
    let count: i64 = conn.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
        rusqlite::params![name],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

fn column_exists(
    conn: &rusqlite::Connection,
    table: &str,
    column: &str,
) -> Result<bool, rusqlite::Error> {
    if !table_exists(conn, table)? {
        return Ok(false);
    }
    let sql = format!("PRAGMA table_info({table})");
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?;
        if name == column {
            return Ok(true);
        }
    }
    Ok(false)
}
