use oneiros_model::AgentName;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum EmergeOutcomes {
    #[outcome(message("'{}' has emerged.", .0))]
    Emerged(AgentName),
}
