use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum LifecycleRequest {
    Dream { agent: String },
    Introspect { agent: String },
    Reflect { agent: String },
    Sense { agent: String, content: String },
    Sleep { agent: String },
}
