use oneiros_outcomes::Outcome;
use std::path::PathBuf;

#[derive(Clone, Outcome)]
pub enum RunServiceOutcomes {
    #[outcome(message("Service starting on {}.", .0.display()))]
    ServiceStarting(PathBuf),
    #[outcome(message("Service stopped."))]
    ServiceStopped,
}
