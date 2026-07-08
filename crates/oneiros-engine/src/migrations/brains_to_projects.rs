use crate::{Config, Databases, DbError, DbHandle, DbKey};

use super::{Migration, MigrationError};

/// Rename the host-DB tables and columns that still carry the legacy
/// `brain*` vocabulary onto the current `project*` names. Each rename
/// is idempotent (guarded by an existence check) and SQLite auto-commits
/// DDL, so renames are applied directly against the host handle rather
/// than wrapped in an explicit transaction. A partially-applied run is
/// safe: re-running `apply` completes the remaining guarded steps.
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

        let databases = Databases::new(config.clone());
        let conn = databases.handle_sync(DbKey::Host)?;
        let needs_rename = table_exists(&conn, "brains")?
            || column_exists(&conn, "bookmarks", "brain")?
            || column_exists(&conn, "follows", "brain")?
            || column_exists(&conn, "tickets", "brain_name")?
            || column_exists(&conn, "tickets", "brain_id")?;

        Ok(needs_rename)
    }

    fn apply(&self, config: &Config) -> Result<(), MigrationError> {
        let databases = Databases::new(config.clone());
        let conn = databases.handle_sync(DbKey::Host)?;

        if table_exists(&conn, "brains")? && !table_exists(&conn, "projects")? {
            conn.execute_batch("ALTER TABLE brains RENAME TO projects")?;
        }

        if column_exists(&conn, "bookmarks", "brain")?
            && !column_exists(&conn, "bookmarks", "project")?
        {
            conn.execute_batch("ALTER TABLE bookmarks RENAME COLUMN brain TO project")?;
        }

        if column_exists(&conn, "follows", "brain")? && !column_exists(&conn, "follows", "project")?
        {
            conn.execute_batch("ALTER TABLE follows RENAME COLUMN brain TO project")?;
        }

        if column_exists(&conn, "tickets", "brain_name")?
            && !column_exists(&conn, "tickets", "project_name")?
        {
            conn.execute_batch("ALTER TABLE tickets RENAME COLUMN brain_name TO project_name")?;
        }

        if column_exists(&conn, "tickets", "brain_id")?
            && !column_exists(&conn, "tickets", "project_id")?
        {
            conn.execute_batch("ALTER TABLE tickets RENAME COLUMN brain_id TO project_id")?;
        }

        Ok(())
    }
}

fn table_exists(conn: &DbHandle, name: &str) -> Result<bool, DbError> {
    let count: i64 = conn.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
        rusqlite::params![name],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

fn column_exists(conn: &DbHandle, table: &str, column: &str) -> Result<bool, DbError> {
    if !table_exists(conn, table)? {
        return Ok(false);
    }
    let sql = format!("PRAGMA table_info({table})");
    let names: Vec<String> = conn.query_map(&sql, [], |row| row.get::<_, String>(1))?;
    Ok(names.iter().any(|name| name == column))
}
