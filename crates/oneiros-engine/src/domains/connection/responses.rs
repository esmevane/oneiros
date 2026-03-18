use serde::{Deserialize, Serialize};

use super::model::Connection;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ConnectionResponse {
    Created(Connection),
    Found(Connection),
    Listed(Vec<Connection>),
    Removed,
}
