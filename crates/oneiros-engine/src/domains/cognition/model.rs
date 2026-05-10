use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Cognition {
    #[builder(default)]
    pub(crate) id: CognitionId,
    pub(crate) agent_id: AgentId,
    #[builder(into)]
    pub(crate) texture: TextureName,
    #[builder(into)]
    pub(crate) content: Content,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

impl Cognition {
    pub(crate) fn ref_token(&self) -> RefToken {
        RefToken::from(Ref::cognition(self.id))
    }
}

impl Indexable<CognitionId> for Cognition {
    fn id(&self) -> CognitionId {
        self.id
    }
}

pub(crate) type Cognitions = EntityIndex<CognitionId, Cognition>;

resource_id!(CognitionId);
