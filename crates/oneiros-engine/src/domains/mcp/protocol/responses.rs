use std::path::PathBuf;

use kinded::Kinded;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = McpConfigResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum McpConfigResponse {
    McpConfigWritten(PathBuf),
    McpConfigExists(PathBuf),
}
