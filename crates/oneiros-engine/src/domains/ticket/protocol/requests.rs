use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub enum CreateTicket {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            #[builder(into)] pub actor_id: ActorId,
            #[arg(long)]
            #[builder(into)] pub brain_name: BrainName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum GetTicket {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub key: ResourceKey<TicketId>,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ValidateTicket {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub token: Token,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ListTickets {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub filters: SearchFilters,
        }
    }
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
