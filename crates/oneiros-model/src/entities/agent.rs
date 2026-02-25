use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum AgentConstructionError {
    #[error("invalid agent id: {0}")]
    InvalidId(#[from] IdParseError),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Agent {
    pub id: AgentId,
    pub name: AgentName,
    pub persona: PersonaName,
    pub description: Description,
    pub prompt: Prompt,
}

impl Agent {
    pub fn init(
        description: impl Into<Description>,
        prompt: impl Into<Prompt>,
        name: AgentName,
        persona: PersonaName,
    ) -> Self {
        Self {
            id: AgentId::new(),
            name,
            persona,
            description: description.into(),
            prompt: prompt.into(),
        }
    }

    pub fn construct(
        id: impl Into<AgentId>,
        description: impl Into<Description>,
        prompt: impl Into<Prompt>,
        name: AgentName,
        persona: PersonaName,
    ) -> Self {
        Self {
            id: id.into(),
            name,
            persona,
            description: description.into(),
            prompt: prompt.into(),
        }
    }

    pub fn construct_from_db(
        (id, name, persona, description, prompt): (
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
        ),
    ) -> Result<Agent, AgentConstructionError> {
        Ok(Agent {
            id: id.as_ref().parse()?,
            name: AgentName::new(name),
            persona: PersonaName::new(persona),
            description: Description::new(description),
            prompt: Prompt::new(prompt),
        })
    }
}

domain_link!(Agent => AgentLink);
domain_id!(AgentId);
domain_name!(AgentName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_same_fields_same_link() {
        let primary = Agent::construct(
            AgentId::new(),
            "desc",
            "prompt",
            AgentName::new("governor.process"),
            PersonaName::new("process"),
        );
        let other = Agent::construct(
            primary.id,
            "desc",
            "prompt",
            AgentName::new("governor.process"),
            PersonaName::new("process"),
        );

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }

    #[test]
    fn agent_different_persona_different_link() {
        let id = AgentId::new();
        let primary = Agent::construct(
            id,
            "desc",
            "prompt",
            AgentName::new("rust"),
            PersonaName::new("expert"),
        );
        let other = Agent::construct(
            id,
            "desc",
            "prompt",
            AgentName::new("rust"),
            PersonaName::new("process"),
        );

        assert_ne!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
