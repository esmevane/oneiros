use oneiros_model::Reflection;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ReflectOutcomes {
    #[outcome(message("Reflecting as '{}'...", .0.agent.name), prompt("{}", .0.prompt))]
    Reflecting(Reflection),
}
