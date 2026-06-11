use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum WakeAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum DreamAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum IntrospectAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ReflectAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SenseContent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
            #[builder(into)] pub(crate) content: Content,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum SleepAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GuidebookAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum EmergeAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) name: AgentName,
            #[builder(into)] pub(crate) persona: PersonaName,
            #[arg(long, default_value = "")]
            #[builder(default, into)] pub(crate) description: Description,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum RecedeAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) agent: AgentName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum StatusAgent {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

fn encode_dream_overrides(overrides: &DreamOverrides) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(value) = overrides.recent_window {
        parts.push(format!("recent_window={value}"));
    }
    if let Some(value) = overrides.dream_depth {
        parts.push(format!("dream_depth={value}"));
    }
    if let Some(value) = overrides.cognition_size {
        parts.push(format!("cognition_size={value}"));
    }
    if let Some(value) = &overrides.recollection_level {
        parts.push(format!("recollection_level={value}"));
    }
    if let Some(value) = overrides.recollection_size {
        parts.push(format!("recollection_size={value}"));
    }
    if let Some(value) = overrides.experience_size {
        parts.push(format!("experience_size={value}"));
    }
    parts.join("&")
}

resource_requests! {
    WakeAgent => |this, client| {
        let WakeAgent::V1(wake) = this;
        client
            .post(
                &format!("/continuity/{agent}/wake", agent = wake.agent),
                &serde_json::Value::Null,
            )
            .await
    },
    DreamAgent => |this, client| {
        let DreamAgent::V1(dream) = this;
        let query = encode_dream_overrides(&DreamOverrides::default());
        let path = if query.is_empty() {
            format!("/continuity/{agent}/dream", agent = dream.agent)
        } else {
            format!("/continuity/{agent}/dream?{query}", agent = dream.agent)
        };
        client.post(&path, &serde_json::Value::Null).await
    },
    IntrospectAgent => |this, client| {
        let IntrospectAgent::V1(introspecting) = this;
        client
            .post(
                &format!(
                    "/continuity/{agent}/introspect",
                    agent = introspecting.agent
                ),
                &serde_json::Value::Null,
            )
            .await
    },
    ReflectAgent => |this, client| {
        let ReflectAgent::V1(reflecting) = this;
        client
            .post(
                &format!("/continuity/{agent}/reflect", agent = reflecting.agent),
                &serde_json::Value::Null,
            )
            .await
    },
    SenseContent => |this, client| {
        let SenseContent::V1(sense) = this;
        client
            .post(
                &format!("/continuity/{agent}/sense", agent = sense.agent),
                this,
            )
            .await
    },
    SleepAgent => |this, client| {
        let SleepAgent::V1(sleeping) = this;
        client
            .post(
                &format!("/continuity/{agent}/sleep", agent = sleeping.agent),
                &serde_json::Value::Null,
            )
            .await
    },
    GuidebookAgent => |this, client| {
        let GuidebookAgent::V1(lookup) = this;
        client
            .get(&format!(
                "/continuity/{agent}/guidebook",
                agent = lookup.agent
            ))
            .await
    },
    EmergeAgent => |this, client| {
        client.post("/continuity", this).await
    },
    RecedeAgent => |this, client| {
        let RecedeAgent::V1(receding) = this;
        client
            .delete(&format!("/continuity/{agent}", agent = receding.agent))
            .await
    },
}

resource_requests! {
    StatusAgent => |client| { client.get("/continuity").await },
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
