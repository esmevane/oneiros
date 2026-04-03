use crate::*;

pub struct TicketTools;

impl TicketTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        ticket_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        ticket_mcp::tool_names()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        ticket_mcp::dispatch(context, tool_name, params).await
    }
}

mod ticket_mcp {
    use crate::*;

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "create_ticket",
                description: "Issue a new ticket for an actor and brain",
                input_schema: schema_for::<CreateTicket>,
            },
            ToolDef {
                name: "get_ticket",
                description: "Look up a specific ticket by ID",
                input_schema: schema_for::<GetTicket>,
            },
            ToolDef {
                name: "list_tickets",
                description: "List all tickets",
                input_schema: schema_for::<ListTickets>,
            },
            ToolDef {
                name: "validate_ticket",
                description: "Validate a ticket token",
                input_schema: schema_for::<ValidateTicket>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &[
            "create_ticket",
            "get_ticket",
            "list_tickets",
            "validate_ticket",
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let system = SystemContext::new(context.config.clone());

        let value = match tool_name {
            "create_ticket" => TicketService::create(&system, &serde_json::from_str(params)?).await,
            "get_ticket" => TicketService::get(&system, &serde_json::from_str(params)?).await,
            "list_tickets" => TicketService::list(&system, &serde_json::from_str(params)?).await,
            "validate_ticket" => {
                TicketService::validate(&system, &serde_json::from_str(params)?).await
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
