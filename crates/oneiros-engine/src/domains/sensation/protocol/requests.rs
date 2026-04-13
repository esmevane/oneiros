use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct SetSensation {
    #[builder(into)]
    pub(crate) name: SensationName,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub(crate) description: Description,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub(crate) prompt: Prompt,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct GetSensation {
    #[builder(into)]
    pub(crate) name: SensationName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct RemoveSensation {
    #[builder(into)]
    pub(crate) name: SensationName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct ListSensations {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub(crate) filters: SearchFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = SensationRequestType, display = "kebab-case")]
pub(crate) enum SensationRequest {
    SetSensation(SetSensation),
    GetSensation(GetSensation),
    ListSensations(ListSensations),
    RemoveSensation(RemoveSensation),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (SensationRequestType::SetSensation, "set-sensation"),
            (SensationRequestType::GetSensation, "get-sensation"),
            (SensationRequestType::ListSensations, "list-sensations"),
            (SensationRequestType::RemoveSensation, "remove-sensation"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
