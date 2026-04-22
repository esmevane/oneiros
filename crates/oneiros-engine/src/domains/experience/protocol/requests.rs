use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct CreateExperience {
    #[builder(into)]
    pub agent: AgentName,
    #[builder(into)]
    pub sensation: SensationName,
    #[builder(into)]
    pub description: Description,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetExperience {
    #[builder(into)]
    pub key: ResourceKey<ExperienceId>,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct ListExperiences {
    #[arg(long)]
    pub agent: Option<AgentName>,
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub filters: SearchFilters,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct UpdateExperienceDescription {
    #[builder(into)]
    pub id: ExperienceId,
    #[builder(into)]
    pub description: Description,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct UpdateExperienceSensation {
    #[builder(into)]
    pub id: ExperienceId,
    #[builder(into)]
    pub sensation: SensationName,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = ExperienceRequestType, display = "kebab-case")]
pub enum ExperienceRequest {
    CreateExperience(CreateExperience),
    GetExperience(GetExperience),
    ListExperiences(ListExperiences),
    UpdateExperienceDescription(UpdateExperienceDescription),
    UpdateExperienceSensation(UpdateExperienceSensation),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (ExperienceRequestType::CreateExperience, "create-experience"),
            (ExperienceRequestType::GetExperience, "get-experience"),
            (ExperienceRequestType::ListExperiences, "list-experiences"),
            (
                ExperienceRequestType::UpdateExperienceDescription,
                "update-experience-description",
            ),
            (
                ExperienceRequestType::UpdateExperienceSensation,
                "update-experience-sensation",
            ),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
