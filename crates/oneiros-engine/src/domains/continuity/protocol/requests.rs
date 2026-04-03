use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct WakeAgent {
    #[builder(into)]
    pub agent: AgentName,
    /// Render the full dream with all vocabulary and memories inline.
    #[arg(long)]
    #[serde(default)]
    #[builder(default)]
    pub deep: bool,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct DreamAgent {
    #[builder(into)]
    pub agent: AgentName,
    /// Render the full dream with all vocabulary and memories inline.
    #[arg(long)]
    #[serde(default)]
    #[builder(default)]
    pub deep: bool,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct IntrospectAgent {
    #[builder(into)]
    pub agent: AgentName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct ReflectAgent {
    #[builder(into)]
    pub agent: AgentName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct SenseContent {
    #[builder(into)]
    pub agent: AgentName,
    #[builder(into)]
    pub content: Content,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct SleepAgent {
    #[builder(into)]
    pub agent: AgentName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GuidebookAgent {
    #[builder(into)]
    pub agent: AgentName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct EmergeAgent {
    #[builder(into)]
    pub name: AgentName,
    #[builder(into)]
    pub persona: PersonaName,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub description: Description,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct RecedeAgent {
    #[builder(into)]
    pub agent: AgentName,
}

#[derive(Builder, Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Args)]
pub struct StatusAgent {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub filters: SearchFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = ContinuityRequestType, display = "kebab-case")]
pub enum ContinuityRequest {
    WakeAgent(WakeAgent),
    DreamAgent(DreamAgent),
    IntrospectAgent(IntrospectAgent),
    ReflectAgent(ReflectAgent),
    SenseContent(SenseContent),
    SleepAgent(SleepAgent),
    GuidebookAgent(GuidebookAgent),
    EmergeAgent(EmergeAgent),
    RecedeAgent(RecedeAgent),
    StatusAgent(StatusAgent),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (ContinuityRequestType::WakeAgent, "wake-agent"),
            (ContinuityRequestType::DreamAgent, "dream-agent"),
            (ContinuityRequestType::IntrospectAgent, "introspect-agent"),
            (ContinuityRequestType::ReflectAgent, "reflect-agent"),
            (ContinuityRequestType::SenseContent, "sense-content"),
            (ContinuityRequestType::SleepAgent, "sleep-agent"),
            (ContinuityRequestType::GuidebookAgent, "guidebook-agent"),
            (ContinuityRequestType::EmergeAgent, "emerge-agent"),
            (ContinuityRequestType::RecedeAgent, "recede-agent"),
            (ContinuityRequestType::StatusAgent, "status-agent"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
