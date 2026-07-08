use std::pin::Pin;

use crate::{Config, Databases, DbError, DbHandle, DbKey};

use super::{Migration, MigrationError};

/// Rename the host-DB tables and columns that still carry the legacy
/// `brain*` vocabulary onto the current `project*` names. Each rename
/// is idempotent (guarded by an existence check). The renames are
/// wrapped in a transaction so a partial failure rolls back cleanly.
pub(crate) struct BrainsToProjects;

impl Migration for BrainsToProjects {
    fn name(&self) -> &'static str {
        "brains → projects (schema renames)"
    }

    fn is_required<'a>(
        &'a self,
        config: &'a Config,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<bool, MigrationError>> + Send + 'a>> {
        Box::pin(async move {
            let host_db = config.platform().host_db_path();
            if !host_db.exists() {
                return Ok(false);
            }

            let databases = Databases::new(config.clone());
            let conn = databases.handle(DbKey::Host).await?;
            let needs_rename = table_exists(&conn, "brains")?
                || column_exists(&conn, "bookmarks", "brain")?
                || column_exists(&conn, "follows", "brain")?
                || column_exists(&conn, "tickets", "brain_name")?
                || column_exists(&conn, "tickets", "brain_id")?;

            Ok(needs_rename)
        })
    }

    fn apply<'a>(
        &'a self,
        config: &'a Config,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), MigrationError>> + Send + 'a>> {
        Box::pin(async move {
            let databases = Databases::new(config.clone());
            let tx = databases.transaction(DbKey::Host).await?;

            if table_exists(&tx, "brains")? && !table_exists(&tx, "projects")? {
                tx.execute_batch("ALTER TABLE brains RENAME TO projects")?;
            }

            if column_exists(&tx, "bookmarks", "brain")?
                && !column_exists(&tx, "bookmarks", "project")?
            {
                tx.execute_batch("ALTER TABLE bookmarks RENAME COLUMN brain TO project")?;
            }

            if column_exists(&tx, "follows", "brain")?
                && !column_exists(&tx, "follows", "project")?
            {
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
        })
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
