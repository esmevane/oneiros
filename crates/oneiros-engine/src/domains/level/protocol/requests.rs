use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct SetLevel {
    #[builder(into)]
    pub(crate) name: LevelName,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub(crate) description: Description,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub(crate) prompt: Prompt,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct GetLevel {
    #[builder(into)]
    pub(crate) name: LevelName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct RemoveLevel {
    #[builder(into)]
    pub(crate) name: LevelName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct ListLevels {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub(crate) filters: SearchFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = LevelRequestType, display = "kebab-case")]
pub(crate) enum LevelRequest {
    SetLevel(SetLevel),
    GetLevel(GetLevel),
    ListLevels(ListLevels),
    RemoveLevel(RemoveLevel),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (LevelRequestType::SetLevel, "set-level"),
            (LevelRequestType::GetLevel, "get-level"),
            (LevelRequestType::ListLevels, "list-levels"),
            (LevelRequestType::RemoveLevel, "remove-level"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
