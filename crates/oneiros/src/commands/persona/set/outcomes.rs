use oneiros_model::PersonaName;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetPersonaOutcomes {
    #[outcome(message("Persona '{0}' set."))]
    PersonaSet(PersonaName),
}
