use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct SetUrge {
    #[builder(into)]
    pub name: UrgeName,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub description: Description,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub prompt: Prompt,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetUrge {
    #[builder(into)]
    pub name: UrgeName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct RemoveUrge {
    #[builder(into)]
    pub name: UrgeName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct ListUrges {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub filters: SearchFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = UrgeRequestType, display = "kebab-case")]
pub enum UrgeRequest {
    SetUrge(SetUrge),
    GetUrge(GetUrge),
    ListUrges(ListUrges),
    RemoveUrge(RemoveUrge),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (UrgeRequestType::SetUrge, "set-urge"),
            (UrgeRequestType::GetUrge, "get-urge"),
            (UrgeRequestType::ListUrges, "list-urges"),
            (UrgeRequestType::RemoveUrge, "remove-urge"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
