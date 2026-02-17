use oneiros_model::{AgentName, Content, Experience, ExperienceId, Link, SensationName};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceEvents {
    ExperienceCreated(Experience),
    ExperienceRefAdded {
        experience_id: ExperienceId,
        #[serde(alias = "record_ref")]
        link: Link,
    },
    ExperienceDescriptionUpdated {
        experience_id: ExperienceId,
        description: Content,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExperienceRequest {
    pub agent: AgentName,
    pub sensation: SensationName,
    pub description: Content,
    #[serde(default, alias = "refs")]
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddExperienceRefRequest {
    pub link: Link,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExperienceDescriptionRequest {
    pub description: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceRequests {
    CreateExperience(CreateExperienceRequest),
    AddExperienceRef(AddExperienceRefRequest),
    UpdateExperienceDescription(UpdateExperienceDescriptionRequest),
}
