use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum AddCognition {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub agent: AgentName,
            #[builder(into)] pub texture: TextureName,
            #[builder(into)] pub content: Content,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum GetCognition {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: ResourceKey<CognitionId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListCognitions {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            pub agent: Option<AgentName>,
            #[arg(long)]
            pub texture: Option<TextureName>,
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub filters: SearchFilters,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = CognitionRequestType, display = "kebab-case")]
pub enum CognitionRequest {
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
