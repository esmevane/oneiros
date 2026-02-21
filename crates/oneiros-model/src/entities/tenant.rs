use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct Tenant {
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
oneiros_link::domain_link!(TenantLink, "tenant");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tenant_identity() {
        let primary = Tenant {
            name: TenantName::new("default"),
        };

        let other = Tenant {
            name: TenantName::new("default"),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }
}
