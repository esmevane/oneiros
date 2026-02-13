use oneiros_model::PersonaName;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemovePersonaOutcomes {
    #[outcome(message("Persona '{0}' removed."))]
    PersonaRemoved(PersonaName),
}
