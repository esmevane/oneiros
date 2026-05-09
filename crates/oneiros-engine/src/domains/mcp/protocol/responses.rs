use std::path::PathBuf;

use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = McpConfigResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum McpResponses {
    McpConfigWritten(McpConfigWrittenResponse),
    McpConfigExists(McpConfigExistsResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum McpConfigWrittenResponse {
        V1 => {
            pub(crate) path: PathBuf,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum McpConfigExistsResponse {
        V1 => {
            pub(crate) path: PathBuf,
        }
    }
}
