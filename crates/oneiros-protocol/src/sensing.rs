#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SenseEvents {
    Sensed { agent: oneiros_model::AgentName },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SensingRequests {
    Sense { agent: oneiros_model::AgentName },
}
