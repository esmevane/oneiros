use bon::Builder;
use lorosurgeon::{Hydrate, Reconcile};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::*;

#[derive(
    Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Hydrate, Reconcile,
)]
pub struct Ticket {
    #[builder(default, into)]
    pub id: TicketId,
    pub actor_id: ActorId,
    pub brain_name: BrainName,
    pub brain_id: BrainId,
    pub token: Token,
    #[builder(default = Timestamp::now())]
    pub created_at: Timestamp,
}

#[derive(Hydrate, Reconcile)]
#[loro(root = "tickets")]
pub struct Tickets(HashMap<String, Ticket>);

resource_id!(TicketId);
