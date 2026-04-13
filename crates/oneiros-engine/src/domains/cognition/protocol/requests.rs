use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct AddCognition {
    #[builder(into)]
    pub(crate) agent: AgentName,
    #[builder(into)]
    pub(crate) texture: TextureName,
    #[builder(into)]
    pub(crate) content: Content,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct GetCognition {
    #[builder(into)]
    pub(crate) id: CognitionId,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct ListCognitions {
    #[arg(long)]
    pub(crate) agent: Option<AgentName>,
    #[arg(long)]
    pub(crate) texture: Option<TextureName>,
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub(crate) filters: SearchFilters,
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
