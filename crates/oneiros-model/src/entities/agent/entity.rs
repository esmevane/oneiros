use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

use super::AgentConstructionError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub id: AgentId,
    pub name: AgentName,
    pub persona: PersonaName,
    pub description: Description,
    pub prompt: Prompt,
}

impl<A, B, C, D, E> TryFrom<(A, B, C, D, E)> for Agent
where
    A: AsRef<str>,
    B: AsRef<str>,
    C: AsRef<str>,
    D: AsRef<str>,
    E: AsRef<str>,
{
    type Error = AgentConstructionError;

    fn try_from(
        (id, name, persona, description, prompt): (A, B, C, D, E),
    ) -> Result<Self, Self::Error> {
        Ok(Agent {
            id: id.as_ref().parse()?,
            name: AgentName::new(name),
            persona: PersonaName::new(persona),
            description: Description::new(description),
            prompt: Prompt::new(prompt),
        })
    }
}

impl Agent {
    pub fn construct_from_db(
        row: impl TryInto<Self, Error = AgentConstructionError>,
    ) -> Result<Self, AgentConstructionError> {
        row.try_into()
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
            id: AgentId::new(),
            name: AgentName::new("governor.process"),
            persona: PersonaName::new("process"),
            description: Description::new("first"),
            prompt: Prompt::new("first"),
        };

        let other = Agent {
            id: AgentId::new(),
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
            id: AgentId::new(),
            name: AgentName::new("rust"),
            persona: PersonaName::new("expert"),
            description: Description::default(),
            prompt: Prompt::default(),
        };

        let other = Agent {
            id: AgentId::new(),
            name: AgentName::new("rust"),
            persona: PersonaName::new("process"),
            description: Description::default(),
            prompt: Prompt::default(),
        };

        assert_ne!(primary.link().unwrap(), other.link().unwrap());
    }
}
