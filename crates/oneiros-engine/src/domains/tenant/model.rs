use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub struct Tenant {
    #[builder(default, into)]
    pub id: TenantId,
    #[builder(into)]
    pub name: TenantName,
    #[builder(default = Timestamp::now(), into)]
    pub created_at: Timestamp,
}

#[derive(Hydrate, Reconcile)]
#[loro(root = "tenants")]
pub struct Tenants(HashMap<String, Tenant>);

resource_id!(TenantId);
resource_name!(TenantName);
