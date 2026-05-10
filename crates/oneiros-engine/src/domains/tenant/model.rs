use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Tenant {
    #[builder(default, into)]
    pub(crate) id: TenantId,
    #[builder(into)]
    pub(crate) name: TenantName,
    #[builder(default = Timestamp::now(), into)]
    pub(crate) created_at: Timestamp,
}

impl Indexable<TenantId> for Tenant {
    fn id(&self) -> TenantId {
        self.id
    }
}

pub(crate) type Tenants = EntityIndex<TenantId, Tenant>;

resource_id!(TenantId);
resource_name!(TenantName);
