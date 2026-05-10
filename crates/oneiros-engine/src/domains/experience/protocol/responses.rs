use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = ExperienceResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum ExperienceResponse {
    ExperienceCreated(ExperienceCreatedResponse),
    ExperienceDetails(ExperienceDetailsResponse),
    Experiences(ExperiencesResponse),
    NoExperiences,
    ExperienceUpdated(ExperienceUpdatedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ExperienceCreatedResponse {
        V1 => { #[serde(flatten)] pub(crate) experience: Experience }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ExperienceDetailsResponse {
        V1 => { #[serde(flatten)] pub(crate) experience: Experience }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ExperiencesResponse {
        V1 => {
            pub(crate) items: Vec<Experience>,
            pub(crate) total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ExperienceUpdatedResponse {
        V1 => { #[serde(flatten)] pub(crate) experience: Experience }
    }
}
