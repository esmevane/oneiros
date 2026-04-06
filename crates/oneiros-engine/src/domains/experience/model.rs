use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub struct Experience {
    #[builder(default)]
    pub id: ExperienceId,
    pub agent_id: AgentId,
    #[builder(into)]
    pub sensation: SensationName,
    #[builder(into)]
    pub description: Description,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

#[derive(Hydrate, Reconcile)]
#[loro(root = "experiences")]
pub struct Experiences(HashMap<String, Experience>);

resource_id!(ExperienceId);
