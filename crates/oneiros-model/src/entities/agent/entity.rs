use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

use super::AgentConstructionError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub name: AgentName,
    pub persona: PersonaName,
    pub description: Description,
    pub prompt: Prompt,
}

impl Agent {
    pub fn construct_from_db(
        (id, name, persona, description, prompt): (
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
        ),
    ) -> Result<Identity<AgentId, Agent>, AgentConstructionError> {
        let id: AgentId = id.as_ref().parse()?;
        let agent = Agent {
            name: AgentName::new(name),
            persona: PersonaName::new(persona),
            description: Description::new(description),
            prompt: Prompt::new(prompt),
        };
        Ok(Identity::new(id, agent))
    }
}

impl Addressable for Agent {
    fn address_label() -> &'static str {
        "agent"
    }

    fn link(&self) -> Result<Link, LinkError> {
        // description and prompt are mutable content, not identity
        Link::new(&(Self::address_label(), &self.name, &self.persona))
    }
}

domain_id!(AgentId);
domain_name!(AgentName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_identity() {
        let primary = Agent {
            name: AgentName::new("governor.process"),
            persona: PersonaName::new("process"),
            description: Description::new("first"),
            prompt: Prompt::new("first"),
        };

        let other = Agent {
            name: AgentName::new("governor.process"),
            persona: PersonaName::new("process"),
            description: Description::new("completely different"),
            prompt: Prompt::new("also different"),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }

    #[test]
    fn agent_different_persona_different_link() {
        let primary = Agent {
            name: AgentName::new("rust"),
            persona: PersonaName::new("expert"),
            description: Description::default(),
            prompt: Prompt::default(),
        };

        let other = Agent {
            name: AgentName::new("rust"),
            persona: PersonaName::new("process"),
            description: Description::default(),
            prompt: Prompt::default(),
        };

        assert_ne!(primary.link().unwrap(), other.link().unwrap());
    }
}
