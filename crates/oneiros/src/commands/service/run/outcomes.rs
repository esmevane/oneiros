use std::path::PathBuf;

#[derive(Clone)]
pub enum RunServiceOutcomes {
    ServiceStarting(PathBuf),
    ServiceStopped,
}

impl oneiros_outcomes::Reportable for RunServiceOutcomes {
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
