use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum CreateExperience {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
            #[builder(into)] pub(crate) sensation: SensationName,
            #[builder(into)] pub(crate) description: Description,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetExperience {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<ExperienceId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListExperiences {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            pub(crate) agent: Option<AgentName>,
            #[arg(long)]
            pub(crate) sensation: Option<SensationName>,
            /// Full-text query against experience description. When present,
            /// hits are FTS5-ranked; absent, the listing browses by filters
            /// alone.
            #[arg(long)]
            #[builder(into)]
            pub(crate) query: Option<String>,
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum UpdateExperienceDescription {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) id: ExperienceId,
            #[builder(into)] pub(crate) description: Description,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum UpdateExperienceSensation {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) id: ExperienceId,
            #[builder(into)] pub(crate) sensation: SensationName,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = ExperienceRequestType, display = "kebab-case")]
pub(crate) enum ExperienceRequest {
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
