use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum UninstallServiceOutcomes {
    #[outcome(message("Service uninstalled."))]
    ServiceUninstalled,
}
