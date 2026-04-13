use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub(crate) struct Tenant {
    #[builder(default, into)]
    pub(crate) id: TenantId,
    #[builder(into)]
    pub(crate) name: TenantName,
    #[builder(default = Timestamp::now(), into)]
    pub(crate) created_at: Timestamp,
}

#[derive(Clone, Default, Hydrate, Reconcile)]
#[loro(root = "tenants")]
pub(crate) struct Tenants(HashMap<String, Tenant>);

impl Tenants {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, id: TenantId) -> Option<&Tenant> {
        self.0.get(&id.to_string())
    }

    pub(crate) fn set(&mut self, tenant: &Tenant) -> Option<Tenant> {
        self.0.insert(tenant.id.to_string(), tenant.clone())
    }

    pub(crate) fn remove(&mut self, tenant_id: TenantId) -> Option<Tenant> {
        self.0.remove(&tenant_id.to_string())
    }
}

resource_id!(TenantId);
resource_name!(TenantName);
