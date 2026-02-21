use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct Actor {
    pub tenant_id: TenantId,
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
oneiros_link::domain_link!(ActorLink, "actor");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actor_identity() {
        let tenant_id = TenantId::new();

        let primary = Actor {
            tenant_id: tenant_id.clone(),
            name: ActorName::new("default"),
        };

        let other = Actor {
            tenant_id,
            name: ActorName::new("default"),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }
}
