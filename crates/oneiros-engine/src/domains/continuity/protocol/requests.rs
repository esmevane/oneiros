use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum WakeAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum DreamAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum IntrospectAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ReflectAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum SenseContent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub agent: AgentName,
            #[builder(into)] pub content: Content,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum SleepAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum GuidebookAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum EmergeAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: AgentName,
            #[builder(into)] pub persona: PersonaName,
            #[arg(long, default_value = "")]
            #[builder(default, into)] pub description: Description,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum RecedeAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum StatusAgent {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub filters: SearchFilters,
        }
    }
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
