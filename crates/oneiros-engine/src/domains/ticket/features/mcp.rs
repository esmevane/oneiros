use crate::*;

pub struct TicketMcp;

impl TicketMcp {
    pub fn defs(&self) -> Vec<ToolDef> {
        ticket_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectLog,
        mailbox: &Mailbox,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        ticket_mcp::dispatch(context, mailbox, tool_name, params).await
    }
}

mod ticket_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateTicket>::new(
                TicketRequestType::CreateTicket,
                "Issue a new ticket for an actor and brain",
            )
            .def(),
            Tool::<GetTicket>::new(
                TicketRequestType::GetTicket,
                "Look up a specific ticket by ID",
            )
            .def(),
            Tool::<ListTickets>::new(TicketRequestType::ListTickets, "List all tickets").def(),
            Tool::<ValidateTicket>::new(
                TicketRequestType::ValidateTicket,
                "Validate a ticket token",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectLog,
        mailbox: &Mailbox,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: TicketRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let scope = ComposeScope::new(context.config.clone())
            .host()
            .map_err(Error::from)?;

        let value = match request_type {
            TicketRequestType::CreateTicket => {
                TicketService::create(&scope, mailbox, &serde_json::from_str(params)?).await
            }
            TicketRequestType::GetTicket => {
                TicketService::get(&scope, &serde_json::from_str(params)?).await
            }
            TicketRequestType::ListTickets => {
                TicketService::list(&scope, &serde_json::from_str(params)?).await
            }
            TicketRequestType::ValidateTicket => {
                TicketService::validate(&scope, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
