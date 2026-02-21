use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tenant {
    pub name: TenantName,
}

domain_link!(Tenant => TenantLink);
domain_id!(TenantId);
domain_name!(TenantName);

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

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
