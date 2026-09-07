use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use axum::{
    Json,
    extract::{Path, Query},
};

use crate::*;

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "Issue a ticket",
        description: "Generate a new access ticket granting entry to a project.",
        content: include_str!("../features/skills/issue.md"),
        status: 201,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                CreateTicket::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.response::<201, Json<TicketCreatedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum CreateTicket {
        #[derive(clap::Args)]
        V1 => {
            #[arg(long)]
            #[builder(into)] pub(crate) actor_id: ActorId,
            #[arg(long)]
            #[builder(into)] pub(crate) project_name: ProjectName,
            #[arg(long = "permission", value_enum)]
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            #[builder(default)]
            pub(crate) permissions: Vec<PermissionOp>,
        }
    }
}

impl CreateTicket {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        mailbox: Mailbox,
        Json(body): Json<CreateTicket>,
    ) -> Result<Json<TicketResponse>, TicketError> {
        let response = TicketService::create(&scope, &mailbox, &body).await?;
        Ok(Json(response))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/{id}",
        summary: "Show a ticket",
        description: "Retrieve the details of a specific project access ticket.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                GetTicket::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.input::<IdPathParam<TicketId>>().response::<200, Json<TicketFoundResponse>>()
                },
            )
        },
    })]
    pub(crate) enum GetTicket {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) key: ResourceKey<TicketId>,
        }
    }
}

impl GetTicket {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        Path(key): Path<ResourceKey<TicketId>>,
    ) -> Result<Json<TicketResponse>, TicketError> {
        Ok(Json(
            TicketService::get(&scope, &GetTicket::builder_v1().key(key).build().into()).await?,
        ))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/validate",
        summary: "Validate a ticket",
        description: "Verify that a ticket is still valid and not revoked.",
        content: include_str!("../features/skills/validate.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Post,
        build: |docs| {
            ResourceMethod::Post.router(
                ValidateTicket::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.response::<200, Json<TicketValidatedResponse>>()
                },
            )
        },
    })]
    pub(crate) enum ValidateTicket {
        #[derive(clap::Args)]
        V1 => {
            #[builder(into)] pub(crate) token: Token,
        }
    }
}

impl ValidateTicket {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        Json(body): Json<ValidateTicket>,
    ) -> Result<Json<TicketResponse>, TicketError> {
        Ok(Json(TicketService::validate(&scope, &body).await?))
    }
}

versioned! {
    #[derive(JsonSchema)]
    #[annotation(ResourceMeta {
        path: "/",
        summary: "List tickets",
        description: "See all access tickets issued for this project.",
        content: include_str!("../features/skills/list.md"),
        status: 200,
    })]
    #[annotation(ResourceHandler {
        method: ResourceMethod::Get,
        build: |docs| {
            ResourceMethod::Get.router(
                ListTickets::handler,
                move |op| {
                    let op = docs.transform(op);
                    op.response::<200, Json<TicketsResponse>>()
                },
            )
        },
    })]
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

impl ListTickets {
    pub(crate) async fn handler(
        scope: Scope<AtHost>,
        Query(params): Query<ListTickets>,
    ) -> Result<Json<TicketResponse>, TicketError> {
        Ok(Json(TicketService::list(&scope, &params).await?))
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
        let query = format!("limit={}&offset={}", listing.filters.limit, listing.filters.offset,);
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

resource_root! {
    TicketRequest => {
        label: "tickets",
        purpose: "Issue and manage project access tickets",
        operations: [CreateTicket, GetTicket, ListTickets, ValidateTicket],
    }
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
