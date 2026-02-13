use oneiros_outcomes::Outcome;
use std::path::PathBuf;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RunServiceOutcomes {
    #[outcome(message("Service starting on {}.", .0.display()))]
    ServiceStarting(PathBuf),
    #[outcome(message("Service stopped."))]
    ServiceStopped,
}
