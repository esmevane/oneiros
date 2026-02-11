use oneiros_model::PersonaName;

#[derive(Clone)]
pub enum SetPersonaOutcomes {
    PersonaSet(PersonaName),
}

impl oneiros_outcomes::Reportable for SetPersonaOutcomes {
    fn level(&self) -> tracing::Level {
        match self {
            Self::PersonaSet(_) => tracing::Level::INFO,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::PersonaSet(name) => format!("Persona '{name}' set."),
        }
    }
}
