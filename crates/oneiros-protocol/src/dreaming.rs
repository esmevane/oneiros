use oneiros_model::{AgentName, DreamContext};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum DreamingEvents {
    DreamBegun { agent: AgentName },
    DreamComplete(Box<DreamContext>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum DreamingRequests {
    BeginDream { agent: AgentName },
}
