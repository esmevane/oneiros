use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceEvents {
    ExperienceCreated(Experience),
    ExperienceDescriptionUpdated(ExperienceDescriptionUpdate),
    ExperienceSensationUpdated(ExperienceSensationUpdate),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceDescriptionUpdate {
    pub id: ExperienceId,
    pub description: Description,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceSensationUpdate {
    pub id: ExperienceId,
    pub sensation: SensationName,
}
