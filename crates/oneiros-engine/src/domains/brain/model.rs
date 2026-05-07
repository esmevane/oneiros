use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Brain {
    #[builder(default = BrainId::new())]
    pub(crate) id: BrainId,
    #[builder(into)]
    pub(crate) name: BrainName,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

impl Indexable<BrainId> for Brain {
    fn id(&self) -> BrainId {
        self.id
    }
}

pub(crate) type Brains = EntityIndex<BrainId, Brain>;

resource_name!(BrainName);
resource_id!(BrainId);
