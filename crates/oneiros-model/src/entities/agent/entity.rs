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

domain_id!(AgentId);
domain_name!(AgentName);
