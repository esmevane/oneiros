use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct WakeAgent {
    #[builder(into)]
    pub(crate) agent: AgentName,
    /// Render the full dream with all vocabulary and memories inline.
    #[arg(long)]
    #[serde(default)]
    #[builder(default)]
    pub(crate) deep: bool,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct DreamAgent {
    #[builder(into)]
    pub(crate) agent: AgentName,
    /// Render the full dream with all vocabulary and memories inline.
    #[arg(long)]
    #[serde(default)]
    #[builder(default)]
    pub(crate) deep: bool,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct IntrospectAgent {
    #[builder(into)]
    pub(crate) agent: AgentName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct ReflectAgent {
    #[builder(into)]
    pub(crate) agent: AgentName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct SenseContent {
    #[builder(into)]
    pub(crate) agent: AgentName,
    #[builder(into)]
    pub(crate) content: Content,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct SleepAgent {
    #[builder(into)]
    pub(crate) agent: AgentName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct GuidebookAgent {
    #[builder(into)]
    pub(crate) agent: AgentName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct EmergeAgent {
    #[builder(into)]
    pub(crate) name: AgentName,
    #[builder(into)]
    pub(crate) persona: PersonaName,
    #[arg(long, default_value = "")]
    #[builder(default, into)]
    pub(crate) description: Description,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct RecedeAgent {
    #[builder(into)]
    pub(crate) agent: AgentName,
}

#[derive(Builder, Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Args)]
pub(crate) struct StatusAgent {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub(crate) filters: SearchFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = ContinuityRequestType, display = "kebab-case")]
pub(crate) enum ContinuityRequest {
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
