use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ServiceStatusOutcomes {
    #[outcome(message("Service is running."))]
    ServiceRunning,
    #[outcome(message("Service is not running: {0}"), level = "warn")]
    ServiceNotRunning(String),
}
