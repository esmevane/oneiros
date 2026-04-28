use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum CreateAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: AgentName,
            #[builder(into)] pub persona: PersonaName,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub description: Description,
            #[arg(long, default_value = "")]
            #[builder(default, into)]
            pub prompt: Prompt,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum GetAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: ResourceKey<AgentName>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListAgents {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub filters: SearchFilters,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum UpdateAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: AgentName,
            #[builder(into)] pub persona: PersonaName,
            #[arg(long, default_value = "")]
            #[builder(into)]
            pub description: Description,
            #[arg(long, default_value = "")]
            #[builder(into)]
            pub prompt: Prompt,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum RemoveAgent {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub name: AgentName,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = AgentRequestType, display = "kebab-case")]
pub enum AgentRequest {
    CreateAgent(CreateAgent),
    GetAgent(GetAgent),
    ListAgents(ListAgents),
    UpdateAgent(UpdateAgent),
    RemoveAgent(RemoveAgent),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (AgentRequestType::CreateAgent, "create-agent"),
            (AgentRequestType::GetAgent, "get-agent"),
            (AgentRequestType::ListAgents, "list-agents"),
            (AgentRequestType::UpdateAgent, "update-agent"),
            (AgentRequestType::RemoveAgent, "remove-agent"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }

    #[test]
    fn create_agent_wire_format_is_unwrapped() {
        let request = CreateAgent::V1(CreateAgentV1 {
            name: AgentName::new("test.process"),
            persona: PersonaName::new("process"),
            description: Description::new("desc"),
            prompt: Prompt::new("prompt"),
        });

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["name"], "test.process");
        assert_eq!(json["persona"], "process");
        assert!(
            json.get("V1").is_none(),
            "V1 layer must not appear on the wire"
        );
    }
}
