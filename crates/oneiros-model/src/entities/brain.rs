use std::path::PathBuf;

use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BrainStatus {
    Active,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Brain {
    pub tenant_id: TenantId,
    pub name: BrainName,
    pub path: PathBuf,
    pub status: BrainStatus,
}

impl Addressable for Brain {
    fn address_label() -> &'static str {
        "brain"
    }

    fn link(&self) -> Result<Link, LinkError> {
        // path and status are operational, not identity
        Link::new(&(Self::address_label(), &self.tenant_id, &self.name))
    }
}

domain_id!(BrainId);
domain_name!(BrainName);
oneiros_link::domain_link!(BrainLink, "brain");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brain_identity() {
        let tenant = TenantId::new();

        let primary = Brain {
            tenant_id: tenant.clone(),
            name: BrainName::new("oneiros"),
            path: PathBuf::from("/tmp/a"),
            status: BrainStatus::Active,
        };

        let other = Brain {
            tenant_id: tenant,
            name: BrainName::new("oneiros"),
            path: PathBuf::from("/tmp/b"),
            status: BrainStatus::Active,
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }
}
