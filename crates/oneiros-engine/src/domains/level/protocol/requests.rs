use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum SetLevel {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: LevelName,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub description: Description,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub prompt: Prompt,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum GetLevel {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: ResourceKey<LevelName>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum RemoveLevel {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: LevelName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListLevels {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub filters: SearchFilters,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = LevelRequestType, display = "kebab-case")]
pub enum LevelRequest {
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
