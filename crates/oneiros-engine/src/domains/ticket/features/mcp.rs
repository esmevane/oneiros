pub mod ticket_mcp {
    use schemars::JsonSchema;
    use serde::Deserialize;

    use crate::*;

    #[derive(Deserialize, JsonSchema)]
    struct CreateParams {
        actor_id: ActorId,
        brain_name: BrainName,
    }

    #[derive(Deserialize, JsonSchema)]
    struct GetParams {
        id: TicketId,
    }

    #[derive(Deserialize, JsonSchema)]
    struct ValidateParams {
        token: String,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "create_ticket",
                description: "Issue a new ticket for an actor and brain",
                input_schema: schema_for::<CreateParams>,
            },
            ToolDef {
                name: "get_ticket",
                description: "Look up a specific ticket by ID",
                input_schema: schema_for::<GetParams>,
            },
            ToolDef {
                name: "list_tickets",
                description: "List all tickets in the system",
                input_schema: schema_for::<serde_json::Value>,
            },
            ToolDef {
                name: "validate_ticket",
                description: "Validate a ticket token",
                input_schema: schema_for::<ValidateParams>,
            },
        ]
    }

    pub fn tool_names() -> &'static [&'static str] {
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
            "create_ticket" => {
                let p: CreateParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = TicketService::create(&system, p.actor_id, p.brain_name)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_ticket" => {
                let p: GetParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = TicketService::get(&system, &p.id)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "list_tickets" => {
                let response = TicketService::list(&system)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "validate_ticket" => {
                let p: ValidateParams = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = TicketService::validate(&system, &p.token)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
