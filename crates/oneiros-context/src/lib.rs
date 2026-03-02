use directories::ProjectDirs;
use oneiros_config::Config;
use oneiros_db::{Database, DatabaseError};
use oneiros_detect_project_name::{ProjectDetector, ProjectRoot};
use oneiros_fs::FileOps;
use oneiros_model::Token;
use oneiros_terminal::TerminalOps;
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "esmevane";
const APPLICATION: &str = "oneiros";

const HEALTH_CHECK_DELAYS: &[Duration] = &[
    Duration::from_millis(200),
    Duration::from_millis(400),
    Duration::from_millis(800),
    Duration::from_millis(1600),
];

#[derive(thiserror::Error, Debug)]
pub enum ContextError {
    #[error("No system context available.")]
    NoContext,
    #[error("Unable to parse project name")]
    NoProject,
    #[error("Malformed or missing token file: {0}")]
    MalformedTokenFile(#[from] std::io::Error),
    #[error("Project directory not available")]
    NoProjectDir,
    #[error("Configuration error: {0}")]
    Config(#[from] oneiros_config::ConfigError),
}

pub struct Context {
    /// The detected project (name and root path), if any.
    project: Option<ProjectRoot>,
    config_dir: PathBuf,
    data_dir: PathBuf,
    config: Config,
}

impl Context {
    /// Discover context from the current working directory.
    pub fn init() -> Result<Self, ContextError> {
        let project_dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
            .ok_or(ContextError::NoProjectDir)?;
        let detector = ProjectDetector::default_chain();
        let cwd = std::env::current_dir()?;
        let project = detector.detect(&cwd);

        let config_dir: PathBuf = project_dirs.config_dir().into();
        let config = Config::load(&config_dir.join("config.toml"))?;

        Ok(Self {
            project,
            config_dir,
            data_dir: project_dirs.data_dir().into(),
            config,
        })
    }

    /// Construct a Context with explicit paths (for testing).
    pub fn with_paths(data_dir: PathBuf, config_dir: PathBuf) -> Self {
        Self {
            project: None,
            config_dir,
            data_dir,
            config: Config::default(),
        }
    }

    /// The loaded configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// The data directory.
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// The config directory.
    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    /// The detected project name, if any.
    pub fn project_name(&self) -> Option<&str> {
        self.project.as_ref().map(|p| p.name.as_str())
    }

    /// The detected project root path, if any.
    pub fn project_root(&self) -> Option<&Path> {
        self.project.as_ref().map(|p| p.path.as_path())
    }

    /// Path to database.
    pub fn db_path(&self) -> PathBuf {
        self.data_dir.join("oneiros.db")
    }

    /// Path to config.
    pub fn config_path(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    /// Path to the token file for a given brain name.
    pub fn ticket_path(&self, brain_name: &str) -> PathBuf {
        self.data_dir
            .join("tickets")
            .join(format!("{brain_name}.token"))
    }

    /// Store a ticket token for a brain.
    pub fn store_ticket(&self, brain_name: &str, token: &str) -> Result<(), std::io::Error> {
        let files = self.files();
        let path = self.ticket_path(brain_name);
        if let Some(parent) = path.parent() {
            files.ensure_dir(parent)?;
        }
        files.write(path, token)
    }

    /// Retrieve the ticket token for the current project's brain.
    pub fn ticket_token(&self) -> Result<Token, ContextError> {
        let name = self.project_name().ok_or(ContextError::NoProject)?;
        Ok(self
            .files()
            .read_to_string(self.ticket_path(name))
            .map(Token)?)
    }

    /// The service manager label, derived from the same qualifier/org/app
    /// constants used for platform directory resolution.
    pub fn service_label(&self) -> String {
        format!("{QUALIFIER}.{ORGANIZATION}.{APPLICATION}")
    }

    /// Path to the log directory for service stdout/stderr.
    pub fn log_dir(&self) -> PathBuf {
        self.data_dir.join("logs")
    }

    /// Path to the current executable.
    pub fn current_exe(&self) -> Result<PathBuf, std::io::Error> {
        std::env::current_exe()
    }

    /// Retry delays for health check polling after service start.
    pub fn health_check_delays(&self) -> &[Duration] {
        HEALTH_CHECK_DELAYS
    }

    /// Check if initialized.
    pub fn is_initialized(&self) -> bool {
        self.db_path().exists()
    }

    pub fn database(&self) -> Result<Database, DatabaseError> {
        Ok(if self.db_path().exists() {
            Database::open(self.db_path())?
        } else {
            Database::create(self.db_path())?
        })
    }

    /// Construct a Client configured for the current service endpoint.
    pub fn client(&self) -> oneiros_client::Client {
        oneiros_client::Client::new(self.config.service_addr())
    }

    pub fn files(&self) -> FileOps {
        FileOps
    }

    pub fn terminal(&self) -> TerminalOps {
        TerminalOps
    }
}
