use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum PersonaOutcomes {
    #[outcome(transparent)]
    Set(#[from] SetPersonaOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemovePersonaOutcomes),
    #[outcome(transparent)]
    List(#[from] ListPersonasOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowPersonaOutcomes),
}
