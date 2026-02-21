use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum BrainStatus {
    Active,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Brain {
    pub tenant_id: TenantId,
    pub name: BrainName,
    pub status: BrainStatus,
}

domain_link!(Brain => BrainLink);
domain_id!(BrainId);
domain_name!(BrainName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brain_identity() {
        let tenant = TenantId::new();

        let primary = Brain {
            tenant_id: tenant,
            name: BrainName::new("oneiros"),
            status: BrainStatus::Active,
        };

        let other = Brain {
            tenant_id: tenant,
            name: BrainName::new("oneiros"),
            status: BrainStatus::Active,
        };

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
