use oneiros_model::Persona;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ListPersonasOutcomes {
    #[outcome(message("No personas configured."))]
    NoPersonas,

    #[outcome(message("Personas: {0:?}"))]
    Personas(Vec<Persona>),
}
