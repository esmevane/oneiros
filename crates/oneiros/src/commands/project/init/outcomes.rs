use oneiros_model::BrainName;
use oneiros_outcomes::Outcome;
use std::path::PathBuf;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum InitProjectOutcomes {
    #[outcome(message("Brain '{0}' created at {}.", .1.display()))]
    BrainCreated(BrainName, PathBuf),
    #[outcome(message("Brain '{0}' already exists."))]
    BrainAlreadyExists(BrainName),
}
