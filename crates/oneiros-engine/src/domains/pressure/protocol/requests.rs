use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetPressure {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

resource_requests! {
    GetPressure => |this, client| {
        let GetPressure::V1(lookup) = this;
        client.get(&format!("/pressures/{}", lookup.agent)).await
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = PressureRequestType, display = "kebab-case")]
pub(crate) enum PressureRequest {
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
