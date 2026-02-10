use std::path::PathBuf;

use oneiros_model::Label;

#[derive(Clone)]
pub(crate) enum ProjectInitOutcomes {
    BrainCreated(Label, PathBuf),
    BrainAlreadyExists(Label),
}

impl oneiros_outcomes::Reportable for ProjectInitOutcomes {
    fn level(&self) -> tracing::Level {
        match self {
            Self::BrainCreated(_, _) => tracing::Level::INFO,
            Self::BrainAlreadyExists(_) => tracing::Level::INFO,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::BrainCreated(name, path) => {
                format!("Brain '{name}' created at {}.", path.display())
            }
            Self::BrainAlreadyExists(name) => {
                format!("Brain '{name}' already exists.")
            }
        }
    }
}
