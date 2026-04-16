use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, schemars::JsonSchema)]
#[kinded(kind = ExperienceResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ExperienceResponse {
    ExperienceCreated(Response<Experience>),
    ExperienceDetails(Response<Experience>),
    Experiences(Listed<Response<Experience>>),
    NoExperiences,
    ExperienceUpdated(Response<Experience>),
}
