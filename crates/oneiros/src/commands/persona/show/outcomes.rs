use oneiros_model::Persona;

#[derive(Clone)]
pub enum ShowPersonaOutcomes {
    PersonaDetails(Persona),
}

impl oneiros_outcomes::Reportable for ShowPersonaOutcomes {
    fn level(&self) -> tracing::Level {
        match self {
            Self::PersonaDetails(_) => tracing::Level::INFO,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::PersonaDetails(info) => {
                format!(
                    "Persona '{}'\n  Description: {}\n  Prompt: {}",
                    info.name, info.description, info.prompt,
                )
            }
        }
    }
}
