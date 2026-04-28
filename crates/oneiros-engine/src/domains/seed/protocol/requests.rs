use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum SeedCore {
        V1 => {}
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum SeedAgents {
        V1 => {}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = SeedRequestType, display = "kebab-case")]
pub enum SeedRequest {
    SeedCore(SeedCore),
    SeedAgents(SeedAgents),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (SeedRequestType::SeedCore, "seed-core"),
            (SeedRequestType::SeedAgents, "seed-agents"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
