use serde::{Deserialize, Serialize};

use super::agent::CreateAgentRequest;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LifecycleEvents {
    Woke { name: AgentName },
    Slept { name: AgentName },
    Emerged { name: AgentName },
    Receded { name: AgentName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum DreamingEvents {
    DreamBegun { agent: AgentName },
    DreamComplete { agent: Agent },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum IntrospectingEvents {
    IntrospectionBegun { agent: AgentName },
    IntrospectionComplete { agent: AgentName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ReflectingEvents {
    ReflectionBegun { agent: AgentName },
    ReflectionComplete { agent: AgentName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SenseEvents {
    Sensed { agent: AgentName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LifecycleRequests {
    Wake { agent: AgentName },
    Sleep { agent: AgentName },
    Emerge(CreateAgentRequest),
    Recede { agent: AgentName },
    Dream { agent: AgentName },
    Introspect { agent: AgentName },
    Reflect { agent: AgentName },
    Sense { agent: AgentName },
}
