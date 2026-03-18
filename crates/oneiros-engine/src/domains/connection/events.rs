use serde::{Deserialize, Serialize};

use super::model::Connection;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ConnectionEvents {
    ConnectionCreated(Connection),
    ConnectionRemoved(ConnectionRemoved),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionRemoved {
    pub id: String,
}
