use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SelectExperienceById {
    pub id: ExperienceId,
}

// ── Event data types ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ExperienceDescriptionUpdate {
    pub experience_id: ExperienceId,
    pub description: Description,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ExperienceSensationUpdate {
    pub experience_id: ExperienceId,
    pub sensation: SensationName,
}

// ── Request types ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CreateExperienceRequest {
    pub agent: AgentName,
    pub sensation: SensationName,
    pub description: Description,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct UpdateExperienceDescriptionRequest {
    #[serde(default)]
    pub id: ExperienceId,
    pub description: Description,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct UpdateExperienceSensationRequest {
    #[serde(default)]
    pub id: ExperienceId,
    pub sensation: SensationName,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetExperienceRequest {
    pub id: ExperienceId,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ListExperiencesRequest {
    #[serde(default)]
    pub agent: Option<AgentName>,
    #[serde(default)]
    pub sensation: Option<SensationName>,
}

// ── Protocol enums ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceEvents {
    ExperienceCreated(Experience),
    ExperienceDescriptionUpdated(ExperienceDescriptionUpdate),
    ExperienceSensationUpdated(ExperienceSensationUpdate),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceRequests {
    CreateExperience(CreateExperienceRequest),
    UpdateExperienceDescription(UpdateExperienceDescriptionRequest),
    UpdateExperienceSensation(UpdateExperienceSensationRequest),
    GetExperience(GetExperienceRequest),
    ListExperiences(ListExperiencesRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceResponses {
    ExperienceCreated(Experience),
    ExperienceUpdated(Experience),
    ExperienceFound(Experience),
    ExperiencesListed(Vec<Experience>),
}
