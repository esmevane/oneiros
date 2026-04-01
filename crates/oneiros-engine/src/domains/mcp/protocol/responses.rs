use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum McpConfigResponse {
    McpConfigWritten(PathBuf),
    McpConfigExists(PathBuf),
}
