use oneiros_model::Introspection;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum IntrospectOutcomes {
    #[outcome(message("Introspecting as '{}'...", .0.agent.name), prompt("{}", .0.prompt))]
    Introspecting(Introspection),
}
