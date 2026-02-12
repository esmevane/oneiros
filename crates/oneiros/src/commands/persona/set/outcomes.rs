use oneiros_model::PersonaName;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum SetPersonaOutcomes {
    #[outcome(message("Persona '{0}' set."))]
    PersonaSet(PersonaName),
}
