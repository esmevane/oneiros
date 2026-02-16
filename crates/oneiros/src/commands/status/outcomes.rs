use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize)]
pub struct StatusResult {
    #[serde(skip)]
    pub dashboard: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum StatusOutcomes {
    #[outcome(message("Status retrieved."), prompt("{}", .0.dashboard))]
    Status(StatusResult),
}
