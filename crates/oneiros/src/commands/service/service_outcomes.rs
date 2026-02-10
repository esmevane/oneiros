use std::path::PathBuf;

#[derive(Clone)]
pub(crate) enum RunOutcomes {
    ServiceStarting(PathBuf),
    ServiceStopped,
}

impl oneiros_outcomes::Reportable for RunOutcomes {
    fn level(&self) -> tracing::Level {
        match self {
            Self::ServiceStarting(_) => tracing::Level::INFO,
            Self::ServiceStopped => tracing::Level::INFO,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::ServiceStarting(path) => {
                format!("Service starting on {}.", path.display())
            }
            Self::ServiceStopped => "Service stopped.".into(),
        }
    }
}

#[derive(Clone)]
pub(crate) enum StatusOutcomes {
    ServiceRunning,
    ServiceNotRunning(String),
}

impl oneiros_outcomes::Reportable for StatusOutcomes {
    fn level(&self) -> tracing::Level {
        match self {
            Self::ServiceRunning => tracing::Level::INFO,
            Self::ServiceNotRunning(_) => tracing::Level::WARN,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::ServiceRunning => "Service is running.".into(),
            Self::ServiceNotRunning(reason) => {
                format!("Service is not running: {reason}")
            }
        }
    }
}
