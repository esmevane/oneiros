use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = ExperienceResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ExperienceResponse {
    ExperienceCreated(ExperienceCreatedResponse),
    ExperienceDetails(ExperienceDetailsResponse),
    Experiences(ExperiencesResponse),
    NoExperiences,
    ExperienceUpdated(ExperienceUpdatedResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ExperienceCreatedResponse {
        V1 => { #[serde(flatten)] pub experience: Experience }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ExperienceDetailsResponse {
        V1 => { #[serde(flatten)] pub experience: Experience }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ExperiencesResponse {
        V1 => {
            pub items: Vec<Experience>,
            pub total: usize,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ExperienceUpdatedResponse {
        V1 => { #[serde(flatten)] pub experience: Experience }
    }
}
