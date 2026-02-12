use oneiros_model::PersonaName;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum RemovePersonaOutcomes {
    #[outcome(message("Persona '{0}' removed."))]
    PersonaRemoved(PersonaName),
}
