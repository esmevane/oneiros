use std::path::{Path, PathBuf};

use chrono::Utc;

use crate::{Config, Platform};

use super::MigrationError;

/// Take a timestamped backup of the entire `data-dir` before any
/// migration mutates it. The backup is a sibling directory so a
/// failed migration can be rolled back with `mv data-dir.backup-X data-dir`.
pub(crate) struct Backup;

impl Backup {
    pub(crate) fn snapshot(config: &Config) -> Result<PathBuf, MigrationError> {
        let source = &config.data_dir;
        let stamp = Utc::now().format("%Y%m%dT%H%M%SZ");
        let target = source.with_file_name(format!(
            "{}.backup-{stamp}",
            source
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("data-dir")
        ));

        let platform = config.platform();
        copy_tree(&platform, source, &target).map_err(|err| MigrationError::Backup {
            reason: format!("copy {source:?} -> {target:?}: {err}"),
        })?;

        Ok(target)
    }
}

fn copy_tree(platform: &Platform, source: &Path, target: &Path) -> std::io::Result<()> {
    platform.ensure_dir(target)?;

    for entry in platform.read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let from = entry.path();
        let to = target.join(entry.file_name());

        if file_type.is_dir() {
            copy_tree(platform, &from, &to)?;
        } else if file_type.is_file() {
            platform.copy(&from, &to)?;
        }
    }

    Ok(())
}
