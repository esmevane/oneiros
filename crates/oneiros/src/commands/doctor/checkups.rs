use std::path::PathBuf;

#[derive(Clone)]
pub(crate) enum Checkups {
    ProjectDetected(String, PathBuf),
    NoProjectDetected,
    Initialized,
    NotInitialized,
    DatabaseOk(PathBuf),
    NoDatabaseFound(PathBuf, String),
    EventLogReady(usize),
    NoEventLog(String),
    ConfigOk(PathBuf),
    NoConfigFound(PathBuf),
    NoContextAvailable,
}

impl oneiros_outcomes::Reportable for Checkups {
    fn level(&self) -> tracing::Level {
        match self {
            Self::NoContextAvailable => tracing::Level::ERROR,
            Self::NoProjectDetected
            | Self::NotInitialized
            | Self::NoDatabaseFound(_, _)
            | Self::NoEventLog(_)
            | Self::NoConfigFound(_) => tracing::Level::WARN,
            Self::ProjectDetected(_, _)
            | Self::Initialized
            | Self::DatabaseOk(_)
            | Self::EventLogReady(_)
            | Self::ConfigOk(_) => tracing::Level::INFO,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::NoContextAvailable => {
                "No context available. Please run this command within a valid project directory."
                    .into()
            }
            Self::ProjectDetected(name, root) => {
                format!("Project '{name}' detected at '{}'.", root.display())
            }
            Self::NoProjectDetected => "No project detected.".into(),
            Self::Initialized => "System is initialized.".into(),
            Self::NotInitialized => "System is not initialized.".into(),
            Self::DatabaseOk(path) => format!("Database found at '{}'.", path.display()),
            Self::NoDatabaseFound(path, error) => {
                format!("Database not found at '{}': {error}", path.display())
            }
            Self::EventLogReady(count) => {
                format!("Event log is ready with {count} events.")
            }
            Self::NoEventLog(error) => format!("Event log error: {error}"),
            Self::ConfigOk(path) => format!("Config file found at '{}'.", path.display()),
            Self::NoConfigFound(path) => {
                format!("Config file not found at '{}'.", path.display())
            }
        }
    }
}
