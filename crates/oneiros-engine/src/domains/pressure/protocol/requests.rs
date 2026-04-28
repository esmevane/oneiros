use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum GetPressure {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub agent: AgentName,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = PressureRequestType, display = "kebab-case")]
pub enum PressureRequest {
    GetPressure(GetPressure),
    ListPressures,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (PressureRequestType::GetPressure, "get-pressure"),
            (PressureRequestType::ListPressures, "list-pressures"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
