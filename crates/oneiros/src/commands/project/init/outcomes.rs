use std::path::PathBuf;

use oneiros_model::BrainName;

#[derive(Clone)]
pub enum InitProjectOutcomes {
    BrainCreated(BrainName, PathBuf),
    BrainAlreadyExists(BrainName),
}

impl oneiros_outcomes::Reportable for InitProjectOutcomes {
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
