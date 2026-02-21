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
oneiros_link::domain_link!(AgentLink, "agent");

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

    #[test]
    fn agent_link_narrows_from_link() {
        let agent = Agent {
            name: AgentName::new("governor.process"),
            persona: PersonaName::new("process"),
            description: Description::default(),
            prompt: Prompt::default(),
        };
        let link = agent.link().unwrap();
        let typed = AgentLink::try_from(link).unwrap();
        assert_eq!(typed.to_string(), agent.link().unwrap().to_string());
    }

    #[test]
    fn agent_link_rejects_wrong_label() {
        let link = Link::new(&("cognition", "working", "some thought")).unwrap();
        let result = AgentLink::try_from(link);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.expected, "agent");
    }

    #[test]
    fn agent_link_broadens_to_link() {
        let agent = Agent {
            name: AgentName::new("governor.process"),
            persona: PersonaName::new("process"),
            description: Description::default(),
            prompt: Prompt::default(),
        };
        let link = agent.link().unwrap();
        let typed = AgentLink::try_from(link.clone()).unwrap();
        let broadened: Link = typed.into();
        assert_eq!(broadened, link);
    }

    #[test]
    fn agent_link_serde_roundtrip() {
        let agent = Agent {
            name: AgentName::new("governor.process"),
            persona: PersonaName::new("process"),
            description: Description::default(),
            prompt: Prompt::default(),
        };
        let typed = AgentLink::try_from(agent.link().unwrap()).unwrap();
        let json = serde_json::to_string(&typed).unwrap();
        let parsed: AgentLink = serde_json::from_str(&json).unwrap();
        assert_eq!(typed, parsed);
    }

    #[test]
    fn agent_link_from_str_roundtrip() {
        let agent = Agent {
            name: AgentName::new("governor.process"),
            persona: PersonaName::new("process"),
            description: Description::default(),
            prompt: Prompt::default(),
        };
        let typed = AgentLink::try_from(agent.link().unwrap()).unwrap();
        let s = typed.to_string();
        let parsed: AgentLink = s.parse().unwrap();
        assert_eq!(typed, parsed);
    }

    #[test]
    fn agent_link_from_str_rejects_wrong_label() {
        let link = Link::new(&("cognition", "working", "some thought")).unwrap();
        let s = link.to_string();
        let result = s.parse::<AgentLink>();
        assert!(result.is_err());
    }
}
