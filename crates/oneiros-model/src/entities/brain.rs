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
    pub id: BrainId,
    pub tenant_id: TenantId,
    pub name: BrainName,
    pub status: BrainStatus,
    pub path: std::path::PathBuf,
}

impl Brain {
    pub fn init(tenant_id: TenantId, name: BrainName, path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            id: BrainId::new(),
            tenant_id,
            name,
            status: BrainStatus::Active,
            path: path.into(),
        }
    }
}

domain_link!(Brain => BrainLink);
domain_id!(BrainId);
domain_name!(BrainName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brain_same_fields_same_link() {
        let id = BrainId::new();
        let tenant = TenantId::new();

        let primary = Brain {
            id,
            tenant_id: tenant,
            name: BrainName::new("oneiros"),
            status: BrainStatus::Active,
            path: std::path::PathBuf::from("/tmp/test.db"),
        };

        let other = Brain {
            id,
            tenant_id: tenant,
            name: BrainName::new("oneiros"),
            status: BrainStatus::Active,
            path: std::path::PathBuf::from("/tmp/test.db"),
        };

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
