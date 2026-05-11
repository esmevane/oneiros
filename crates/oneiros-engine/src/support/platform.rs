#![allow(clippy::disallowed_methods)]

use etcetera::app_strategy::{AppStrategy, AppStrategyArgs, choose_app_strategy};
use std::path::{Path, PathBuf};

use crate::*;

const TLD: &str = "com";
const AUTHOR: &str = "esmevane";
const APP: &str = "oneiros";

const CONFIG_FILE: &str = "config.toml";
const SYSTEM_DB: &str = "system.db";
const TICKETS_DIR: &str = "tickets";
const BOOKMARKS_DIR: &str = "bookmarks";
const EVENTS_DB: &str = "events.db";

#[derive(Debug, thiserror::Error)]
pub(crate) enum PlatformError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Platform — the steward of where things live and the keeper of the
/// directories they live in.
///
/// Holds a single `data_dir` and answers all layout questions about
/// the host: where the system DB lives, where each brain's events log
/// goes, where bookmark databases sit. Also owns the side of the
/// substrate that touches the filesystem to ensure those directories
/// exist.
///
/// Construct explicitly with [`Platform::new`] or via OS detection
/// with [`Platform::resolve`] / [`Platform::default`].
#[derive(Clone, Debug)]
pub(crate) struct Platform {
    data_dir: PathBuf,
}

impl Platform {
    /// Construct a Platform bound to an explicit data directory.
    pub(crate) fn new(data_dir: impl Into<PathBuf>) -> Self {
        Self {
            data_dir: data_dir.into(),
        }
    }

    /// Resolve a Platform from OS conventions (XDG / Apple / Windows).
    pub(crate) fn resolve() -> Self {
        let args = AppStrategyArgs {
            top_level_domain: TLD.into(),
            author: AUTHOR.into(),
            app_name: APP.into(),
        };

        let strategy = choose_app_strategy(args).expect("unable to determine home directory");

        Self {
            data_dir: strategy.data_dir(),
        }
    }

    /// The application's data directory.
    pub(crate) fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// Path to the user-editable config file.
    pub(crate) fn config_path(&self) -> PathBuf {
        self.data_dir.join(CONFIG_FILE)
    }

    /// Path to the host's projection database.
    pub(crate) fn system_db_path(&self) -> PathBuf {
        self.data_dir.join(SYSTEM_DB)
    }

    /// Directory where issued ticket tokens are persisted.
    pub(crate) fn tickets_dir(&self) -> PathBuf {
        self.data_dir.join(TICKETS_DIR)
    }

    /// Path to the cached token for a given brain.
    pub(crate) fn token_path(&self, brain: &BrainName) -> PathBuf {
        self.tickets_dir().join(format!("{brain}.token"))
    }

    /// Directory holding all data for a given brain.
    pub(crate) fn brain_dir(&self, brain: &BrainName) -> PathBuf {
        self.data_dir.join(brain.as_str())
    }

    /// Path to a brain's append-only event log database.
    pub(crate) fn events_db_path(&self, brain: &BrainName) -> PathBuf {
        self.brain_dir(brain).join(EVENTS_DB)
    }

    /// Directory holding all bookmark databases for a given brain.
    pub(crate) fn bookmarks_dir(&self, brain: &BrainName) -> PathBuf {
        self.brain_dir(brain).join(BOOKMARKS_DIR)
    }

    /// Path to a specific bookmark's projection database.
    pub(crate) fn bookmark_db_path(&self, brain: &BrainName, bookmark: &BookmarkName) -> PathBuf {
        self.bookmarks_dir(brain).join(format!("{bookmark}.db"))
    }

    /// Ensure the data directory exists.
    pub(crate) fn ensure_data_dir(&self) -> Result<(), PlatformError> {
        std::fs::create_dir_all(&self.data_dir)?;
        Ok(())
    }

    /// Read a UTF-8 file at the given path.
    pub(crate) fn read_to_string(&self, path: impl AsRef<Path>) -> std::io::Result<String> {
        std::fs::read_to_string(path)
    }

    /// Read the raw bytes of a file at the given path.
    pub(crate) fn read(&self, path: impl AsRef<Path>) -> std::io::Result<Vec<u8>> {
        std::fs::read(path)
    }

    /// Write bytes (or a string) to a file at the given path, creating or truncating it.
    pub(crate) fn write(
        &self,
        path: impl AsRef<Path>,
        contents: impl AsRef<[u8]>,
    ) -> std::io::Result<()> {
        std::fs::write(path, contents)
    }

    /// Ensure a directory exists at the given path, creating parents as needed.
    pub(crate) fn ensure_dir(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        std::fs::create_dir_all(path)
    }

    /// Create a directory at the given path (parents must already exist).
    #[allow(dead_code)]
    pub(crate) fn create_dir(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        std::fs::create_dir(path)
    }

    /// Remove a file at the given path.
    pub(crate) fn remove_file(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        std::fs::remove_file(path)
    }

    /// Recursively remove a directory and its contents.
    #[allow(dead_code)]
    pub(crate) fn remove_dir_all(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        std::fs::remove_dir_all(path)
    }

    /// Iterate the entries of a directory.
    #[allow(dead_code)]
    pub(crate) fn read_dir(&self, path: impl AsRef<Path>) -> std::io::Result<std::fs::ReadDir> {
        std::fs::read_dir(path)
    }

    /// Fetch metadata for the entry at the given path.
    #[allow(dead_code)]
    pub(crate) fn metadata(&self, path: impl AsRef<Path>) -> std::io::Result<std::fs::Metadata> {
        std::fs::metadata(path)
    }

    /// Open a file for reading.
    pub(crate) fn open_file(&self, path: impl AsRef<Path>) -> std::io::Result<std::fs::File> {
        std::fs::File::open(path)
    }

    /// Create (or truncate) a file for writing.
    #[allow(dead_code)]
    pub(crate) fn create_file(&self, path: impl AsRef<Path>) -> std::io::Result<std::fs::File> {
        std::fs::File::create(path)
    }

    /// Open a file with custom [`std::fs::OpenOptions`].
    pub(crate) fn open_with(
        &self,
        path: impl AsRef<Path>,
        options: &std::fs::OpenOptions,
    ) -> std::io::Result<std::fs::File> {
        options.open(path)
    }

    /// Ensure a brain's directory exists.
    pub(crate) fn ensure_brain_dir(&self, brain: &BrainName) -> Result<(), PlatformError> {
        std::fs::create_dir_all(self.brain_dir(brain))?;
        Ok(())
    }

    /// Ensure a brain's bookmarks directory exists.
    pub(crate) fn ensure_bookmarks_dir(&self, brain: &BrainName) -> Result<(), PlatformError> {
        std::fs::create_dir_all(self.bookmarks_dir(brain))?;
        Ok(())
    }

    /// The service label for OS registration (e.g., `com.esmevane.oneiros`).
    pub(crate) fn service_label(&self) -> String {
        format!("{TLD}.{AUTHOR}.{APP}")
    }
}

impl Default for Platform {
    fn default() -> Self {
        Self::resolve()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolved_data_dir_ends_with_app_name() {
        let platform = Platform::resolve();
        assert!(
            platform.data_dir().ends_with(APP),
            "data_dir should end with app name, got: {}",
            platform.data_dir().display()
        );
    }

    #[test]
    fn service_label_is_reverse_domain() {
        assert_eq!(Platform::resolve().service_label(), "com.esmevane.oneiros");
    }

    #[test]
    fn explicit_data_dir_drives_layout() {
        let platform = Platform::new("/tmp/oneiros-test");
        let brain = BrainName::new("alpha");
        let bookmark = BookmarkName::main();

        assert_eq!(
            platform.system_db_path(),
            Path::new("/tmp/oneiros-test/system.db")
        );
        assert_eq!(
            platform.brain_dir(&brain),
            Path::new("/tmp/oneiros-test/alpha")
        );
        assert_eq!(
            platform.events_db_path(&brain),
            Path::new("/tmp/oneiros-test/alpha/events.db")
        );
        assert_eq!(
            platform.bookmark_db_path(&brain, &bookmark),
            Path::new("/tmp/oneiros-test/alpha/bookmarks/main.db")
        );
    }

    #[test]
    fn ensure_data_dir_creates_missing_directory() -> Result<(), PlatformError> {
        let dir = tempfile::tempdir().unwrap();
        let nested = dir.path().join("a/b/c");
        let platform = Platform::new(&nested);

        assert!(!nested.exists());
        platform.ensure_data_dir()?;
        assert!(nested.is_dir());
        Ok(())
    }

    #[test]
    fn ensure_brain_and_bookmarks_dirs_are_idempotent() -> Result<(), PlatformError> {
        let dir = tempfile::tempdir().unwrap();
        let platform = Platform::new(dir.path());
        let brain = BrainName::new("alpha");

        platform.ensure_brain_dir(&brain)?;
        platform.ensure_brain_dir(&brain)?; // idempotent
        platform.ensure_bookmarks_dir(&brain)?;
        platform.ensure_bookmarks_dir(&brain)?; // idempotent

        assert!(platform.brain_dir(&brain).is_dir());
        assert!(platform.bookmarks_dir(&brain).is_dir());
        Ok(())
    }
}
