use oneiros_model::Dream;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum GuidebookOutcomes {
    #[outcome(message("Guidebook for '{}'", .0.context.agent.name), prompt("{}", .0.prompt))]
    Guidebook(Dream),
}
