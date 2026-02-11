use oneiros_model::PersonaName;

#[derive(Clone)]
pub enum RemovePersonaOutcomes {
    PersonaRemoved(PersonaName),
}

impl oneiros_outcomes::Reportable for RemovePersonaOutcomes {
    fn level(&self) -> tracing::Level {
        match self {
            Self::PersonaRemoved(_) => tracing::Level::INFO,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::PersonaRemoved(name) => format!("Persona '{name}' removed."),
        }
    }
}
