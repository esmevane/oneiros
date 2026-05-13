use crate::Config;

use super::{Migration, MigrationError};

/// Rename the legacy `system.db` file (plus its WAL/SHM sidecars) to
/// `host.db` in the data-dir. Idempotent: a no-op once the rename has
/// happened.
pub(crate) struct SystemDbToHostDb;

impl Migration for SystemDbToHostDb {
    fn name(&self) -> &'static str {
        "system.db → host.db"
    }

    fn is_required(&self, config: &Config) -> Result<bool, MigrationError> {
        let platform = config.platform();
        Ok(platform.legacy_host_db_path().exists() && !platform.host_db_path().exists())
    }

    fn apply(&self, config: &Config) -> Result<(), MigrationError> {
        let platform = config.platform();
        let legacy = platform.legacy_host_db_path();
        let current = platform.host_db_path();

        // Fold any pending WAL into the main DB before renaming so we
        // can move a single file safely. The sidecars are then idle and
        // can be cleaned up.
        let conn = rusqlite::Connection::open(&legacy)?;
        conn.pragma_update(None, "journal_mode", "wal")?;
        conn.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |_| Ok(()))?;
        drop(conn);

        platform.rename(&legacy, &current)?;

        for suffix in ["db-wal", "db-shm"] {
            let sidecar = legacy.with_extension(suffix);
            if sidecar.exists() {
                platform.remove_file(&sidecar)?;
            }
        }

        Ok(())
    }
}
