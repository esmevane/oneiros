use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Experience {
    #[builder(default)]
    pub(crate) id: ExperienceId,
    pub(crate) agent_id: AgentId,
    #[builder(into)]
    pub(crate) sensation: SensationName,
    #[builder(into)]
    pub(crate) description: Description,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

impl Experience {
    pub(crate) fn ref_token(&self) -> RefToken {
        RefToken::from(Ref::experience(self.id))
    }
}

impl Indexable<ExperienceId> for Experience {
    fn id(&self) -> ExperienceId {
        self.id
    }
}

pub(crate) type Experiences = EntityIndex<ExperienceId, Experience>;

resource_id!(ExperienceId);
