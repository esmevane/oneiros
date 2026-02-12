use oneiros_model::Persona;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ShowPersonaOutcomes {
    #[outcome(message("Persona '{}'\n  Description: {}\n  Prompt: {}", .0.name, .0.description, .0.prompt))]
    PersonaDetails(Persona),
}
