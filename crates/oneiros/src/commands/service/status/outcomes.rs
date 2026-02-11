#[derive(Clone)]
pub enum ServiceStatusOutcomes {
    ServiceRunning,
    ServiceNotRunning(String),
}

impl oneiros_outcomes::Reportable for ServiceStatusOutcomes {
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
