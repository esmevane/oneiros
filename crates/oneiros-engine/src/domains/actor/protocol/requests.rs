use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ActorRequest {
    Create { tenant_id: String, name: String },
    Get { id: String },
    List,
}
