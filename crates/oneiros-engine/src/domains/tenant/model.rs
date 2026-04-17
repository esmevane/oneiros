use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Tenant {
    #[builder(default, into)]
    pub id: TenantId,
    #[builder(into)]
    pub name: TenantName,
    #[builder(default = Timestamp::now(), into)]
    pub created_at: Timestamp,
}

#[derive(Clone, Default)]
pub struct Tenants(HashMap<String, Tenant>);

impl Tenants {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, id: TenantId) -> Option<&Tenant> {
        self.0.get(&id.to_string())
    }

    pub fn set(&mut self, tenant: &Tenant) -> Option<Tenant> {
        self.0.insert(tenant.id.to_string(), tenant.clone())
    }

    pub fn remove(&mut self, tenant_id: TenantId) -> Option<Tenant> {
        self.0.remove(&tenant_id.to_string())
    }
}

resource_id!(TenantId);
resource_name!(TenantName);
