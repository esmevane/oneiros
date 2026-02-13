use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum StopServiceOutcomes {
    #[outcome(message("Service stopped."))]
    ServiceStopped,
}
