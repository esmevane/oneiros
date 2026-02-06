use directories::ProjectDirs;
use oneiros_db::{Database, DatabaseError};
use oneiros_detect_project_name::{ProjectDetector, ProjectRoot};
use oneiros_fs::FileOps;
use oneiros_terminal::TerminalOps;
use std::path::{Path, PathBuf};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "esmevane";
const APPLICATION: &str = "oneiros";

pub(crate) struct Context {
    /// The detected project (name and root path), if any.
    pub(crate) project: Option<ProjectRoot>,
    pub(crate) config_dir: PathBuf,
    pub(crate) data_dir: PathBuf,
}

impl Context {
    /// Discover context from the current working directory.
    pub(crate) fn discover() -> Option<Self> {
        let project_dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)?;
        let detector = ProjectDetector::default_chain();
        let cwd = std::env::current_dir().ok()?;
        let project = detector.detect(&cwd);

        Some(Self {
            project,
            config_dir: project_dirs.config_dir().into(),
            data_dir: project_dirs.data_dir().into(),
        })
    }

    /// The detected project name, if any.
    pub(crate) fn project_name(&self) -> Option<&str> {
        self.project.as_ref().map(|p| p.name.as_str())
    }

    /// The detected project root path, if any.
    pub(crate) fn project_root(&self) -> Option<&Path> {
        self.project.as_ref().map(|p| p.path.as_path())
    }

    /// Path to database.
    pub(crate) fn db_path(&self) -> PathBuf {
        self.data_dir.join("oneiros.db")
    }

    /// Path to config.
    pub(crate) fn config_path(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    /// Check if initialized.
    pub(crate) fn is_initialized(&self) -> bool {
        self.db_path().exists()
    }

    pub(crate) fn database(&self) -> Result<Database, DatabaseError> {
        Ok(if self.db_path().exists() {
            Database::open(self.db_path())?
        } else {
            Database::create(self.db_path())?
        })
    }

    pub(crate) fn files(&self) -> FileOps {
        FileOps
    }

    pub(crate) fn terminal(&self) -> TerminalOps {
        TerminalOps
    }
}
