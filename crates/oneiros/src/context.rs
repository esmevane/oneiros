use std::path::PathBuf;

use directories::ProjectDirs;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "esmevane";
const APPLICATION: &str = "oneiros";

pub(crate) struct Context {
    pub(crate) project_dir: PathBuf,
    pub(crate) config_dir: PathBuf,
    pub(crate) data_dir: PathBuf,
}

impl Context {
    pub(crate) fn new(cli: impl crate::Cli) -> Option<Self> {
        let project_dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)?;

        Some(Self {
            project_dir: cli.project_dir(),
            config_dir: project_dirs.config_dir().into(),
            data_dir: project_dirs.data_dir().into(),
        })
    }

    /// Path to database.
    pub fn db_path(&self) -> PathBuf {
        self.data_dir.join("oneiros.db")
    }

    /// Path to config.
    pub fn config_path(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    /// Check if initialized.
    pub fn is_initialized(&self) -> bool {
        self.db_path().exists()
    }
}
