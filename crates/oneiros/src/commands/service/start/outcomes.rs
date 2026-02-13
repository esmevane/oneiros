use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum StartServiceOutcomes {
    #[outcome(message("Service started."))]
    Started,
    #[outcome(message("Service is running."))]
    Healthy,
    #[outcome(
        message("Service started but health check failed: {0}"),
        level = "warn"
    )]
    StartedButUnhealthy(String),
}
