use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = TenantEventsType, display = "kebab-case")]
pub enum TenantEvents {
    TenantCreated(TenantCreated),
}

impl TenantEvents {
    pub fn maybe_tenant(&self) -> Option<Tenant> {
        match self {
            TenantEvents::TenantCreated(event) => event.clone().current().ok().map(|v| v.tenant),
        }
    }
}

versioned! {
    pub enum TenantCreated {
        V1 => {
            #[serde(flatten)] pub tenant: Tenant,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        assert_eq!(
            &TenantEventsType::TenantCreated.to_string(),
            "tenant-created"
        );
    }

    #[test]
    fn tenant_created_wire_format_is_flat() {
        let tenant = Tenant::builder().name(TenantName::new("acme")).build();

        let event = TenantEvents::TenantCreated(TenantCreated::V1(TenantCreatedV1 {
            tenant: tenant.clone(),
        }));
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "tenant-created");
        assert!(
            json["data"].get("tenant").is_none(),
            "flatten must elide the tenant envelope on the wire"
        );
        assert_eq!(json["data"]["id"], tenant.id.to_string());
        assert_eq!(json["data"]["name"], "acme");
        assert!(json["data"].get("created_at").is_some());
    }
}
