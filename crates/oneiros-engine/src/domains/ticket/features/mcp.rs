use crate::*;

pub struct TicketTools;

impl TicketTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        ticket_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        ticket_mcp::dispatch(context, tool_name, params).await
    }
}

mod ticket_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateTicket>::def(
                TicketRequestType::CreateTicket,
                "Issue a new ticket for an actor and brain",
            ),
            Tool::<GetTicket>::def(
                TicketRequestType::GetTicket,
                "Look up a specific ticket by ID",
            ),
            Tool::<ListTickets>::def(TicketRequestType::ListTickets, "List all tickets"),
            Tool::<ValidateTicket>::def(
                TicketRequestType::ValidateTicket,
                "Validate a ticket token",
            ),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        let request_type: TicketRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let system = SystemContext::new(context.config.clone());

        match request_type {
            TicketRequestType::CreateTicket => {
                let resp = TicketService::create(&system, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    TicketResponse::Created(ticket) => Ok(McpResponse::new(format!(
                        "Ticket issued: {} for brain {}",
                        ticket.id, ticket.brain_name
                    ))),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            TicketRequestType::GetTicket => {
                let resp = TicketService::get(&system, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    TicketResponse::Found(ticket) => Ok(McpResponse::new(format!(
                        "**id:** {}\n**brain:** {}\n",
                        ticket.id, ticket.brain_name
                    ))),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            TicketRequestType::ListTickets => {
                let resp = TicketService::list(&system, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    TicketResponse::Listed(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for ticket in &listed.items {
                            body.push_str(&format!(
                                "- {} (brain: {})\n",
                                ticket.id, ticket.brain_name
                            ));
                        }
                        Ok(McpResponse::new(body))
                    }
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            TicketRequestType::ValidateTicket => {
                let resp = TicketService::validate(&system, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    TicketResponse::Validated(ticket) => Ok(McpResponse::new(format!(
                        "Ticket valid: {} for brain {}",
                        ticket.id, ticket.brain_name
                    ))),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
        }
    }
}
