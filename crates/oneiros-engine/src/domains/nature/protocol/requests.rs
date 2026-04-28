use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum SetNature {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: NatureName,
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
    pub enum GetNature {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: ResourceKey<NatureName>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum RemoveNature {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: NatureName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListNatures {
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
#[kinded(kind = NatureRequestType, display = "kebab-case")]
pub enum NatureRequest {
    SetNature(SetNature),
    GetNature(GetNature),
    ListNatures(ListNatures),
    RemoveNature(RemoveNature),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (NatureRequestType::SetNature, "set-nature"),
            (NatureRequestType::GetNature, "get-nature"),
            (NatureRequestType::ListNatures, "list-natures"),
            (NatureRequestType::RemoveNature, "remove-nature"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
