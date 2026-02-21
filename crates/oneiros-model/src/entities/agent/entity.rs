use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

use super::AgentConstructionError;

pub type AgentRecord = Identity<AgentId, HasDescription<HasPrompt<Agent>>>;

impl AgentRecord {
    pub fn init(
        description: impl Into<Description>,
        prompt: impl Into<Prompt>,
        agent: impl Into<Agent>,
    ) -> Self {
        Self::construct(AgentId::new(), description, prompt, agent)
    }

    pub fn construct(
        id: impl Into<AgentId>,
        description: impl Into<Description>,
        prompt: impl Into<Prompt>,
        agent: impl Into<Agent>,
    ) -> Self {
        Identity::new(
            id.into(),
            HasDescription::new(
                description.into(),
                HasPrompt::new(prompt.into(), agent.into()),
            ),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Agent {
    pub name: AgentName,
    pub persona: PersonaName,
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
    ) -> Result<AgentRecord, AgentConstructionError> {
        let agent = Agent {
            name: AgentName::new(name),
            persona: PersonaName::new(persona),
        };

        Ok(AgentRecord::construct(
            id.as_ref().parse::<AgentId>()?,
            description,
            prompt,
            agent,
        ))
    }
}

domain_link!(Agent => AgentLink);
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
        };

        let other = Agent {
            name: AgentName::new("governor.process"),
            persona: PersonaName::new("process"),
        };

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }

    #[test]
    fn agent_different_persona_different_link() {
        let primary = Agent {
            name: AgentName::new("rust"),
            persona: PersonaName::new("expert"),
        };

        let other = Agent {
            name: AgentName::new("rust"),
            persona: PersonaName::new("process"),
        };

        assert_ne!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
