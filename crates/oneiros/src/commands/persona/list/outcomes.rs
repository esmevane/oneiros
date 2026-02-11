use oneiros_model::Persona;

#[derive(Clone)]
pub enum ListPersonasOutcomes {
    NoPersonas,
    Personas(Vec<Persona>),
}

impl oneiros_outcomes::Reportable for ListPersonasOutcomes {
    fn level(&self) -> tracing::Level {
        match self {
            Self::NoPersonas => tracing::Level::INFO,
            Self::Personas(_) => tracing::Level::INFO,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::NoPersonas => "No personas configured.".into(),
            Self::Personas(personas) => {
                let names: Vec<&str> = personas.iter().map(|p| p.name.as_str()).collect();
                format!("Personas: {}", names.join(", "))
            }
        }
    }
}
