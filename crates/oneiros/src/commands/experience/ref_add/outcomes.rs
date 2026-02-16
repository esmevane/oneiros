use oneiros_model::ExperienceId;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RefAddOutcomes {
    #[outcome(message("Reference added to experience: {0}"))]
    RefAdded(ExperienceId),
}
