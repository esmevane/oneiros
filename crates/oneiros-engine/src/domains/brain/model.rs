use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub struct Brain {
    #[builder(default = BrainId::new())]
    pub id: BrainId,
    #[builder(into)]
    pub name: BrainName,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

#[derive(Hydrate, Reconcile)]
#[loro(root = "brains")]
pub struct Brains(HashMap<String, Brain>);

resource_name!(BrainName);
resource_id!(BrainId);
