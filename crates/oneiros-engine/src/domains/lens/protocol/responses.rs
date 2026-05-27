use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// All responses the lens domain can produce.
#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = LensResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum LensResponse {
    Parsed(ParsedLensResponse),
    Explained(ExplainedLensResponse),
    Queried(QueriedLensResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ParsedLensResponse {
        V1 => {
            pub(crate) source: String,
            pub(crate) display: String,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ExplainedLensResponse {
        V1 => {
            pub(crate) source: String,
            pub(crate) display: String,
            pub(crate) plan: String,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum QueriedLensResponse {
        V1 => {
            pub(crate) source: String,
            pub(crate) hits: Vec<Hit>,
            pub(crate) total: usize,
        }
    }
}
