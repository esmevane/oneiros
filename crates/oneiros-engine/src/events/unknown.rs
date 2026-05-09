use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub(crate) struct UnknownEvent {
    #[serde(rename = "type")]
    pub(crate) type_tag: String,
    pub(crate) data: serde_json::Value,
}
