use std::path::PathBuf;

use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = McpConfigResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum McpConfigResponse {
    McpConfigWritten(McpConfigWrittenResponse),
    McpConfigExists(McpConfigExistsResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum McpConfigWrittenResponse {
        V1 => {
            pub path: PathBuf,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum McpConfigExistsResponse {
        V1 => {
            pub path: PathBuf,
        }
    }
}
