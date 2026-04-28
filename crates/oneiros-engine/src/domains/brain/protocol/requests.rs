use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum CreateBrain {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: BrainName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum GetBrain {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: ResourceKey<BrainName>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListBrains {
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
#[kinded(kind = BrainRequestType, display = "kebab-case")]
pub enum BrainRequest {
    CreateBrain(CreateBrain),
    GetBrain(GetBrain),
    ListBrains(ListBrains),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (BrainRequestType::CreateBrain, "create-brain"),
            (BrainRequestType::GetBrain, "get-brain"),
            (BrainRequestType::ListBrains, "list-brains"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
