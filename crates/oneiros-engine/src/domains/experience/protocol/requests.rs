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
            /// Lens expression — replaces ad-hoc filters with the unified
            /// query language. When set, agent/sensation/query are ignored
            /// and the lens drives selection end-to-end.
            #[arg(long)]
            #[builder(into)]
            pub(crate) lens: Option<String>,
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

resource_requests! {
    CreateExperience => |this, client| {
        client.post("/experiences", this).await
    },
    GetExperience => |this, client| {
        let GetExperience::V1(lookup) = this;
        client.get(&format!("/experiences/{}", lookup.key)).await
    },
    ListExperiences => |this, client| {
        let ListExperiences::V1(listing) = this;
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(agent_name) = &listing.agent {
            params.push(("agent", agent_name.to_string()));
        }

        if let Some(sensation_name) = &listing.sensation {
            params.push(("sensation", sensation_name.to_string()));
        }

        if let Some(query) = &listing.query {
            params.push(("query", query.clone()));
        }

        if let Some(lens) = &listing.lens {
            params.push(("lens", lens.clone()));
        }

        params.push(("limit", listing.filters.limit.to_string()));
        params.push(("offset", listing.filters.offset.to_string()));

        let query = params
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join("&");

        client.get(&format!("/experiences?{query}")).await
    },
    UpdateExperienceDescription => |this, client| {
        let UpdateExperienceDescription::V1(update) = this;
        client
            .put(
                &format!("/experiences/{}/description", update.id),
                &serde_json::json!({ "description": update.description }),
            )
            .await
    },
    UpdateExperienceSensation => |this, client| {
        let UpdateExperienceSensation::V1(update) = this;
        client
            .put(
                &format!("/experiences/{}/sensation", update.id),
                &serde_json::json!({ "sensation": update.sensation }),
            )
            .await
    },
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
