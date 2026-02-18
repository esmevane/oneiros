use oneiros_model::Agent;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SleepOutcomes {
    #[outcome(message("'{}' is sleeping.", .0.name))]
    Sleeping(Agent),
}
