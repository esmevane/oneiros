use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ConnectionResponse {
    Created(Connection),
    Found(Connection),
    Listed(Vec<Connection>),
    Removed,
}
