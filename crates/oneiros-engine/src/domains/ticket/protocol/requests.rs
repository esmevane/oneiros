use bon::Builder;
use clap::Args;
use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct CreateTicket {
    #[arg(long)]
    #[builder(into)]
    pub actor_id: ActorId,
    #[arg(long)]
    #[builder(into)]
    pub brain_name: BrainName,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetTicket {
    #[builder(into)]
    pub id: TicketId,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct ValidateTicket {
    #[builder(into)]
    pub token: Token,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct ListTickets {
    #[command(flatten)]
    #[serde(flatten)]
    #[builder(default)]
    pub filters: SearchFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = TicketRequestType, display = "kebab-case")]
pub enum TicketRequest {
    CreateTicket(CreateTicket),
    GetTicket(GetTicket),
    ListTickets(ListTickets),
    ValidateTicket(ValidateTicket),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_are_kebab_cased() {
        let cases = [
            (TicketRequestType::CreateTicket, "create-ticket"),
            (TicketRequestType::GetTicket, "get-ticket"),
            (TicketRequestType::ListTickets, "list-tickets"),
            (TicketRequestType::ValidateTicket, "validate-ticket"),
        ];

        for (request_type, expectation) in cases {
            assert_eq!(&request_type.to_string(), expectation)
        }
    }
}
