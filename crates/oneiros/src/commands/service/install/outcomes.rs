use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum InstallServiceOutcomes {
    #[outcome(message("Service installed as '{0}'."))]
    ServiceInstalled(String),
}
