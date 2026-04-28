use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum SetSensation {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: SensationName,
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
    pub enum GetSensation {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: ResourceKey<SensationName>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum RemoveSensation {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: SensationName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListSensations {
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
#[kinded(kind = SensationRequestType, display = "kebab-case")]
pub enum SensationRequest {
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
