use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum InstallServiceOutcomes {
    #[outcome(message("Service installed as '{0}'."))]
    ServiceInstalled(String),
}
