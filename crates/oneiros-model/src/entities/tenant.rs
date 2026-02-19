use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct Tenant {
    pub tenant_id: TenantId,
    pub name: TenantName,
}

impl Addressable for Tenant {
    fn address_label() -> &'static str {
        "tenant"
    }

    fn link(&self) -> Result<Link, LinkError> {
        Link::new(&(Self::address_label(), &self.name))
    }
}

domain_id!(TenantId);
domain_name!(TenantName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tenant_identity() {
        let primary = Tenant {
            tenant_id: TenantId::new(),
            name: TenantName::new("default"),
        };

        let other = Tenant {
            tenant_id: TenantId::new(),
            name: TenantName::new("default"),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }
}
