use std::path::PathBuf;

/// Result of a migration sweep. `AlreadyCurrent` means no migration had
/// work to do. `Migrated` carries the list of applied step names and the
/// path of the backup snapshot taken before mutation.
#[derive(Debug, Clone)]
pub(crate) enum MigrationOutcome {
    AlreadyCurrent,
    Migrated {
        applied: Vec<&'static str>,
        backup_path: PathBuf,
    },
}
