use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum SkillOutcomes {
    #[outcome(transparent)]
    Install(#[from] InstallSkillOutcomes),
}
