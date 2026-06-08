use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum CreateTicket {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            #[builder(into)] pub(crate) actor_id: ActorId,
            #[arg(long)]
            #[builder(into)] pub(crate) project_name: ProjectName,
        }
    }
}

impl ClientRequest for CreateTicket {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/tickets", self).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum GetTicket {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<TicketId>,
        }
    }
}

impl ClientRequest for GetTicket {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let GetTicket::V1(lookup) = self;
        client.get(&format!("/tickets/{}", lookup.key)).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ValidateTicket {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) token: Token,
        }
    }
}

impl ClientRequest for ValidateTicket {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        client.post("/tickets/validate", self).await
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ListTickets {
        #[derive(clap::Args)]
        V1 => {
            #[command(flatten)]
            #[serde(flatten)]
            #[builder(default)]
            pub(crate) filters: SearchFilters,
        }
    }
}

impl ClientRequest for ListTickets {
    type Error = ClientError;

    async fn execute_request(&self, client: &Client) -> Result<Vec<u8>, Self::Error> {
        let ListTickets::V1(listing) = self;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/tickets?{query}")).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Kinded)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
#[kinded(kind = TicketRequestType, display = "kebab-case")]
pub(crate) enum TicketRequest {
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
