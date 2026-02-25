use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tenant {
    pub id: TenantId,
    pub name: TenantName,
}

impl Tenant {
    pub fn init(name: TenantName) -> Self {
        Self {
            id: TenantId::new(),
            name,
        }
    }
}

domain_link!(Tenant => TenantLink);
domain_id!(TenantId);
domain_name!(TenantName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tenant_same_fields_same_link() {
        let id = TenantId::new();
        let primary = Tenant {
            id,
            name: TenantName::new("default"),
        };

        let other = Tenant {
            id,
            name: TenantName::new("default"),
        };

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
