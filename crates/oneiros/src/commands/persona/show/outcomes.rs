use oneiros_model::PersonaRecord;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowPersonaOutcomes {
    #[outcome(message("Persona '{}'\n  Description: {}\n  Prompt: {}", .0.name, .0.description, .0.prompt))]
    PersonaDetails(PersonaRecord),
}
