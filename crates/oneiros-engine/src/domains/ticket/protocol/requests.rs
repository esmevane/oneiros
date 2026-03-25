use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct CreateTicket {
    #[arg(long)]
    pub actor_id: ActorId,
    #[arg(long)]
    pub brain_name: BrainName,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetTicket {
    pub id: TicketId,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct ValidateTicket {
    pub token: Token,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TicketRequest {
    Create(CreateTicket),
    Get(GetTicket),
    List,
    Validate(ValidateTicket),
}
