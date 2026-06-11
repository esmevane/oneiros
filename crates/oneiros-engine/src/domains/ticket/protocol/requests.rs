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
            /// Operations this ticket grants. Repeatable. Empty = full access.
            #[arg(long = "permission", value_enum)]
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            #[builder(default)]
            pub(crate) permissions: Vec<PermissionOp>,
        }
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

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ValidateTicket {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) token: Token,
        }
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

resource_requests! {
    CreateTicket => |this, client| { client.post("/tickets", this).await },
    GetTicket => |this, client| {
        let GetTicket::V1(lookup) = this;
        client.get(&format!("/tickets/{}", lookup.key)).await
    },
    ListTickets => |this, client| {
        let ListTickets::V1(listing) = this;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        client.get(&format!("/tickets?{query}")).await
    },
    ValidateTicket => |this, client| { client.post("/tickets/validate", this).await },
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

// Internal service request for issuing a ticket. Not an HTTP/CLI endpoint —
// used by services that need to mint distribution tickets (bookmark share,
// project share).
versioned! {
    pub(crate) enum IssueTicket {
        V1 => {
            #[builder(into)] pub(crate) project_name: ProjectName,
            pub(crate) project: Project,
            pub(crate) actor_id: ActorId,
            pub(crate) target: Ref,
            #[builder(default)]
            pub(crate) permissions: Vec<Permission>,
        }
    }
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
