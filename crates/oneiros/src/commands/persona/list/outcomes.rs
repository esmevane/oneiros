use oneiros_model::Persona;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListPersonasOutcomes {
    #[outcome(message("No personas configured."))]
    NoPersonas,

    #[outcome(message("Personas: {0:?}"))]
    Personas(Vec<Persona>),
}
