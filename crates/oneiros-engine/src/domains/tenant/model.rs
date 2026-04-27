use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum Tenant {
    Current(TenantV1),
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct TenantV1 {
    #[builder(default, into)]
    pub id: TenantId,
    #[builder(into)]
    pub name: TenantName,
    #[builder(default = Timestamp::now(), into)]
    pub created_at: Timestamp,
}

impl Tenant {
    pub fn build_v1() -> TenantV1Builder {
        TenantV1::builder()
    }

    pub fn id(&self) -> TenantId {
        match self {
            Self::Current(v) => v.id,
        }
    }

    pub fn name(&self) -> &TenantName {
        match self {
            Self::Current(v) => &v.name,
        }
    }

    pub fn created_at(&self) -> Timestamp {
        match self {
            Self::Current(v) => v.created_at,
        }
    }
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
        self.0.insert(tenant.id().to_string(), tenant.clone())
    }

    pub fn remove(&mut self, tenant_id: TenantId) -> Option<Tenant> {
        self.0.remove(&tenant_id.to_string())
    }
}

resource_id!(TenantId);
resource_name!(TenantName);
