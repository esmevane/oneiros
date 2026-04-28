use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = PressureResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum PressureResponse {
    Readings(ReadingsResponse),
    AllReadings(AllReadingsResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ReadingsResponse {
        V1 => {
            #[builder(into)] pub agent: AgentName,
            pub pressures: Vec<Pressure>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum AllReadingsResponse {
        V1 => {
            pub pressures: Vec<Pressure>,
        }
    }
}
