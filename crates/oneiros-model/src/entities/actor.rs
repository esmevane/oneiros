use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct Actor {
    pub tenant_id: TenantId,
    pub actor_id: ActorId,
    pub name: ActorName,
}

impl Addressable for Actor {
    fn address_label() -> &'static str {
        "actor"
    }

    fn link(&self) -> Result<Link, LinkError> {
        Link::new(&(Self::address_label(), &self.tenant_id, &self.name))
    }
}

domain_id!(ActorId);
domain_name!(ActorName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actor_identity() {
        let primary = Actor {
            tenant_id: TenantId::new(),
            actor_id: ActorId::new(),
            name: ActorName::new("default"),
        };

        let other = Actor {
            // Same tenant, different actor_id — same link
            tenant_id: primary.tenant_id,
            actor_id: ActorId::new(),
            name: ActorName::new("default"),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }
}
