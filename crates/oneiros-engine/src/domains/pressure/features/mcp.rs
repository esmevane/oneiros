use crate::*;

pub struct PressureTools;

impl PressureTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        pressure_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        pressure_mcp::dispatch(state, config, tool_name, params).await
    }

    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![ResourceTemplateDef::new(
            "oneiros-mcp://agent/{name}/pressure",
            "agent-pressure",
            "Detailed pressure breakdown",
        )]
    }

    pub async fn read_resource(
        &self,
        context: &ProjectContext,
        path: &str,
    ) -> Option<Result<String, ToolError>> {
        if let Some(rest) = path.strip_prefix("agent/") {
            let parts: Vec<&str> = rest.splitn(2, '/').collect();
            if parts.len() == 2 && parts[1] == "pressure" {
                let agent_name = parts[0];
                return Some(pressure_mcp::read_pressure(context, agent_name).await);
            }
        }
        None
    }
}

mod pressure_mcp {
    use crate::*;

    pub async fn read_pressure(
        context: &ProjectContext,
        agent_name: &str,
    ) -> Result<String, ToolError> {
        let response = PressureService::get(
            context,
            &GetPressure::builder()
                .agent(AgentName::new(agent_name))
                .build(),
        )
        .await
        .map_err(Error::from)?;

        let mut md = format!("# Pressure — {agent_name}\n\n");
        match response {
            PressureResponse::Readings(result) => {
                if result.pressures.is_empty() {
                    md.push_str("No pressure readings.\n");
                } else {
                    md.push_str("| Urge | Urgency |\n");
                    md.push_str("|------|--------|\n");
                    for p in &result.pressures {
                        md.push_str(&format!("| {} | {:.2} |\n", p.urge, p.urgency()));
                    }
                }
            }
            _ => md.push_str("No readings.\n"),
        }
        Ok(md)
    }

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<GetPressure>::def(
                PressureRequestType::GetPressure,
                "Check an agent's cognitive pressure",
            ),
            Tool::<serde_json::Value>::def(
                PressureRequestType::ListPressures,
                "See all pressure readings",
            ),
        ]
    }

    pub async fn dispatch(
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        let context = state
            .project_context(config.clone())
            .map_err(|e| ToolError::Domain(e.to_string()))?;

        let request_type: PressureRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            PressureRequestType::GetPressure => {
                let resp = PressureService::get(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    PressureResponse::Readings(result) => {
                        let mut body = format!("# Pressure — {}\n\n", result.agent);
                        if result.pressures.is_empty() {
                            body.push_str("No pressure readings.\n");
                        } else {
                            body.push_str("| Urge | Urgency |\n");
                            body.push_str("|------|--------|\n");
                            for p in &result.pressures {
                                body.push_str(&format!("| {} | {:.2} |\n", p.urge, p.urgency()));
                            }
                        }
                        Ok(McpResponse::new(body))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            PressureRequestType::ListPressures => {
                let resp = PressureService::list(&context).await.map_err(Error::from)?;
                match resp {
                    PressureResponse::AllReadings(result) => {
                        let mut body = String::from("# All Pressure Readings\n\n");
                        if result.pressures.is_empty() {
                            body.push_str("No pressure readings.\n");
                        } else {
                            body.push_str("| Urge | Urgency |\n");
                            body.push_str("|------|--------|\n");
                            for p in &result.pressures {
                                body.push_str(&format!("| {} | {:.2} |\n", p.urge, p.urgency()));
                            }
                        }
                        Ok(McpResponse::new(body))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
        }
    }
}
