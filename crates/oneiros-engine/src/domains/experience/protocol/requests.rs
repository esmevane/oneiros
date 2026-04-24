use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum CreateExperience {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub agent: AgentName,
            #[builder(into)] pub sensation: SensationName,
            #[builder(into)] pub description: Description,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum GetExperience {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: ResourceKey<ExperienceId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListExperiences {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            pub agent: Option<AgentName>,
            #[arg(long)]
            pub sensation: Option<SensationName>,
            /// Full-text query against experience description. When present,
            /// hits are FTS5-ranked; absent, the listing browses by filters
            /// alone.
            #[arg(long)]
            #[builder(into)]
            pub query: Option<String>,
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub filters: SearchFilters,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum UpdateExperienceDescription {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub id: ExperienceId,
            #[builder(into)] pub description: Description,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum UpdateExperienceSensation {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub id: ExperienceId,
            #[builder(into)] pub sensation: SensationName,
        }
    }
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
