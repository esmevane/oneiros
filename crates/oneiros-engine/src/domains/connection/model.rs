use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema, PartialEq)]
pub(crate) struct Connection {
    #[builder(default)]
    pub(crate) id: ConnectionId,
    pub(crate) from_ref: Ref,
    pub(crate) to_ref: Ref,
    #[builder(into)]
    pub(crate) nature: NatureName,
    #[builder(default = Timestamp::now())]
    pub(crate) created_at: Timestamp,
}

impl Connection {
    pub(crate) fn get_from_token(&self) -> RefToken {
        RefToken::from(self.from_ref.clone())
    }

    pub(crate) fn get_to_token(&self) -> RefToken {
        RefToken::from(self.to_ref.clone())
    }
}

impl Indexable<ConnectionId> for Connection {
    fn id(&self) -> ConnectionId {
        self.id
    }
}

pub(crate) type Connections = EntityIndex<ConnectionId, Connection>;

resource_id!(ConnectionId);
