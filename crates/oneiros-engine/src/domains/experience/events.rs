use serde::{Deserialize, Serialize};

use super::model::Experience;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceEvents {
    ExperienceCreated(Experience),
    ExperienceDescriptionUpdated(ExperienceDescriptionUpdate),
    ExperienceSensationUpdated(ExperienceSensationUpdate),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceDescriptionUpdate {
    pub id: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceSensationUpdate {
    pub id: String,
    pub sensation: String,
}
