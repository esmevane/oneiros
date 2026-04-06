use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub struct Cognition {
    #[builder(default)]
    pub id: CognitionId,
    pub agent_id: AgentId,
    #[builder(into)]
    pub texture: TextureName,
    #[builder(into)]
    pub content: Content,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

#[derive(Hydrate, Reconcile)]
#[loro(root = "cognitions")]
pub struct Cognitions(HashMap<String, Cognition>);

resource_id!(CognitionId);
