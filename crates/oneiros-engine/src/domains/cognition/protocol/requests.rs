use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum AddCognition {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
            #[builder(into)] pub(crate) texture: TextureName,
            #[builder(into)] pub(crate) content: Content,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetCognition {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<CognitionId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListCognitions {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            pub(crate) agent: Option<AgentName>,
            #[arg(long)]
            pub(crate) texture: Option<TextureName>,
            /// Full-text query against cognition content. When present, hits
            /// are FTS5-ranked; absent, the listing browses by filters alone.
            #[arg(long)]
            #[builder(into)]
            pub(crate) query: Option<String>,
            /// Lens expression — replaces ad-hoc filters with the unified
            /// query language. When set, agent/texture/query are ignored
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

resource_requests! {
    AddCognition => |this, client| { client.post("/cognitions", this).await },
    GetCognition => |this, client| {
        let GetCognition::V1(lookup) = this;
        client.get(&format!("/cognitions/{}", lookup.key)).await
    },
    ListCognitions => |this, client| {
        let ListCognitions::V1(listing) = this;
        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(agent_name) = &listing.agent {
            params.push(("agent", agent_name.to_string()));
        }

        if let Some(texture_name) = &listing.texture {
            params.push(("texture", texture_name.to_string()));
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

        client.get(&format!("/cognitions?{query}")).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = CognitionRequestType, display = "kebab-case")]
pub(crate) enum CognitionRequest {
    AddCognition(AddCognition),
    GetCognition(GetCognition),
    ListCognitions(ListCognitions),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (CognitionRequestType::AddCognition, "add-cognition"),
            (CognitionRequestType::GetCognition, "get-cognition"),
            (CognitionRequestType::ListCognitions, "list-cognitions"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
