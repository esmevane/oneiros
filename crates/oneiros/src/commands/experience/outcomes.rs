use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum ExperienceOutcomes {
    #[outcome(transparent)]
    Create(#[from] CreateExperienceOutcomes),
    #[outcome(transparent)]
    List(#[from] ListExperiencesOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowExperienceOutcomes),
    #[outcome(transparent)]
    RefAdd(#[from] RefAddOutcomes),
    #[outcome(transparent)]
    Update(#[from] UpdateExperienceOutcomes),
}
