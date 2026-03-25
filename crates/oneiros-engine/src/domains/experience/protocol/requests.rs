use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct CreateExperience {
    pub agent: AgentName,
    pub sensation: SensationName,
    pub description: Description,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetExperience {
    pub id: ExperienceId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct ListExperiences {
    #[arg(long)]
    pub agent: Option<AgentName>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct UpdateExperienceDescription {
    pub id: ExperienceId,
    pub description: Description,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct UpdateExperienceSensation {
    pub id: ExperienceId,
    pub sensation: SensationName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ExperienceRequest {
    CreateExperience(CreateExperience),
    GetExperience(GetExperience),
    ListExperiences(ListExperiences),
    UpdateExperienceDescription(UpdateExperienceDescription),
    UpdateExperienceSensation(UpdateExperienceSensation),
}
