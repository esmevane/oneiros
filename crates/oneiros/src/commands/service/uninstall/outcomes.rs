use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum UninstallServiceOutcomes {
    #[outcome(message("Service uninstalled."))]
    ServiceUninstalled,
}
