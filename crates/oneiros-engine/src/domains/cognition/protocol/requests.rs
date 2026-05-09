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
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
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
