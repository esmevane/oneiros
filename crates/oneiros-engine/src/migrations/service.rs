use crate::Config;

use super::{
    Backup, BrainsToProjects, Migration, MigrationError, MigrationOutcome, SystemDbToHostDb,
};

/// Orchestrates the layout-migration sweep. Owns the registry of known
/// migrations and runs them in declaration order whenever at least one
/// is required.
pub(crate) struct MigrationService;

impl MigrationService {
    /// Bring the data-dir up to the layout the current engine expects.
    ///
    /// Runs every registered migration's `is_required` check, snapshots
    /// the data-dir if any migration has work, then applies the required
    /// ones in order. Returns `AlreadyCurrent` when nothing was needed —
    /// the common path on every subsequent boot.
    pub(crate) fn ensure_current(config: &Config) -> Result<MigrationOutcome, MigrationError> {
        let registry = Self::registry();
        let mut backup_path: Option<std::path::PathBuf> = None;
        let mut applied: Vec<&'static str> = Vec::new();

        // Single pass, check-then-apply per migration in declaration order.
        // Migrations earlier in the registry can change the state that
        // later migrations check against (e.g. SystemDbToHostDb renames
        // the file BrainsToProjects then inspects), so `is_required`
        // always runs against the post-prior-step layout. The backup is
        // taken lazily on the first migration that has work — pristine
        // boots never snapshot.
        for migration in &registry {
            if !migration.is_required(config)? {
                continue;
            }

            if backup_path.is_none() {
                backup_path = Some(Backup::snapshot(config)?);
            }

            migration
                .apply(config)
                .map_err(|err| MigrationError::Step {
                    name: migration.name(),
                    reason: err.to_string(),
                })?;
            applied.push(migration.name());
        }

        match backup_path {
            Some(backup_path) => Ok(MigrationOutcome::Migrated {
                applied,
                backup_path,
            }),
            None => Ok(MigrationOutcome::AlreadyCurrent),
        }
    }

    /// Inventory in declaration order. Adding a new migration is one line
    /// here plus the module that owns it.
    fn registry() -> Vec<Box<dyn Migration>> {
        vec![Box::new(SystemDbToHostDb), Box::new(BrainsToProjects)]
    }
}
