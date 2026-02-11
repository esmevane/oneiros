mod error;

use directories::ProjectDirs;
use oneiros_db::{Database, DatabaseError};
use oneiros_detect_project_name::{ProjectDetector, ProjectRoot};
use oneiros_fs::FileOps;
use oneiros_model::Token;
use oneiros_terminal::TerminalOps;
use std::path::{Path, PathBuf};

pub(crate) use error::ContextError;

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
    pub(crate) fn init() -> Result<Self, ContextError> {
        let project_dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
            .ok_or(ContextError::NoProjectDir)?;
        let detector = ProjectDetector::default_chain();
        let cwd = std::env::current_dir()?;
        let project = detector.detect(&cwd);

        Ok(Self {
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

    /// Path to the service Unix socket.
    pub(crate) fn socket_path(&self) -> PathBuf {
        self.data_dir.join("oneiros.sock")
    }

    /// Path to the token file for a given brain name.
    pub(crate) fn ticket_path(&self, brain_name: &str) -> PathBuf {
        self.data_dir
            .join("tickets")
            .join(format!("{brain_name}.token"))
    }

    /// Store a ticket token for a brain.
    pub(crate) fn store_ticket(&self, brain_name: &str, token: &str) -> Result<(), std::io::Error> {
        let path = self.ticket_path(brain_name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, token)
    }

    /// Retrieve the ticket token for the current project's brain.
    pub(crate) fn ticket_token(&self) -> Result<Token, error::ContextError> {
        let name = self.project_name().ok_or(error::ContextError::NoProject)?;
        Ok(std::fs::read_to_string(self.ticket_path(name)).map(Token)?)
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
