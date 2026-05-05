use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct UnknownEvent {
    #[serde(rename = "type")]
    pub type_tag: String,
    pub data: serde_json::Value,
}
