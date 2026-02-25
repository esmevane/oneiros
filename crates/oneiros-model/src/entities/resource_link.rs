use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemResourceLink {
    Actor(ActorLink),
    Brain(BrainLink),
    Tenant(TenantLink),
}

impl SystemResourceLink {
    pub fn to_string(&self) -> Result<String, LinkError> {
        Ok(data_encoding::BASE64URL_NOPAD.encode(&postcard::to_allocvec(self)?))
    }

    pub fn from_string(given_string: impl AsRef<str>) -> Result<Self, LinkError> {
        Ok(postcard::from_bytes(
            &data_encoding::BASE64URL_NOPAD.decode(given_string.as_ref().as_bytes())?,
        )?)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectResourceLink {
    Agent(AgentLink),
    Connection(ConnectionLink),
    Experience(ExperienceLink),
    StorageEntry(StorageEntryLink),
}

impl ProjectResourceLink {
    pub fn to_string(&self) -> Result<String, LinkError> {
        Ok(data_encoding::BASE64URL_NOPAD.encode(&postcard::to_allocvec(self)?))
    }

    pub fn from_string(given_string: impl AsRef<str>) -> Result<Self, LinkError> {
        Ok(postcard::from_bytes(
            &data_encoding::BASE64URL_NOPAD.decode(given_string.as_ref().as_bytes())?,
        )?)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResourceLink {
    ProjectResourceLink(ProjectResourceLink),
    SystemResourceLink(SystemResourceLink),
}

impl ResourceLink {
    pub fn to_string(&self) -> Result<String, LinkError> {
        Ok(data_encoding::BASE64URL_NOPAD.encode(&postcard::to_allocvec(self)?))
    }

    pub fn from_string(given_string: impl AsRef<str>) -> Result<Self, LinkError> {
        Ok(postcard::from_bytes(
            &data_encoding::BASE64URL_NOPAD.decode(given_string.as_ref().as_bytes())?,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_deserializing() {
        let tenant_id = TenantId::new();
        let actor_id = ActorId::new();
        let actor_link = ActorLink::Actor(Actor {
            id: actor_id,
            tenant_id,
            name: ActorName::new("actor"),
        });

        let resource_link =
            ResourceLink::SystemResourceLink(SystemResourceLink::Actor(ActorLink::Actor(Actor {
                id: actor_id,
                tenant_id,
                name: ActorName::new("actor"),
            })));

        assert_eq!(
            serde_json::to_string(&resource_link).unwrap(),
            serde_json::to_string(&actor_link).unwrap()
        );

        assert_eq!(
            actor_link.to_link_string().unwrap(),
            resource_link.to_string().unwrap()
        );

        assert_eq!(
            actor_link.to_link_string().unwrap(),
            ActorLink::from_string(resource_link.to_string().unwrap())
                .unwrap()
                .to_link_string()
                .unwrap()
        )
    }
}
