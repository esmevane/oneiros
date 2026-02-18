use oneiros_model::Observation;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SenseOutcomes {
    #[outcome(message("Sensing as '{}'...", .0.agent.name), prompt("{}", .0.prompt))]
    Sensing(Observation),
}
