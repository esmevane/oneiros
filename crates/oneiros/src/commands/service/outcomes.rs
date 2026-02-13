use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum ServiceOutcomes {
    #[outcome(transparent)]
    Install(#[from] InstallServiceOutcomes),
    #[outcome(transparent)]
    Uninstall(#[from] UninstallServiceOutcomes),
    #[outcome(transparent)]
    Start(#[from] StartServiceOutcomes),
    #[outcome(transparent)]
    Stop(#[from] StopServiceOutcomes),
    #[outcome(transparent)]
    Run(#[from] RunServiceOutcomes),
    #[outcome(transparent)]
    Status(#[from] ServiceStatusOutcomes),
}
