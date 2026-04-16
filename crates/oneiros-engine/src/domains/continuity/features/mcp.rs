use crate::*;

pub struct ContinuityTools;

impl ContinuityTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        continuity_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        continuity_mcp::dispatch(state, config, tool_name, params).await
    }

    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![
            ResourceTemplateDef::new(
                "oneiros-mcp://agent/{name}/status",
                "agent-status",
                "Dashboard for a specific agent",
            ),
            ResourceTemplateDef::new(
                "oneiros-mcp://agent/{name}/dream",
                "agent-dream",
                "Full assembled identity and cognitive context",
            ),
            ResourceTemplateDef::new(
                "oneiros-mcp://agent/{name}/guidebook",
                "agent-guidebook",
                "Cognitive reference for an agent",
            ),
        ]
    }

    pub async fn read_resource(
        &self,
        context: &ProjectContext,
        path: &str,
    ) -> Option<Result<String, ToolError>> {
        if let Some(rest) = path.strip_prefix("agent/") {
            let parts: Vec<&str> = rest.splitn(2, '/').collect();
            if parts.len() == 2 {
                let agent_name = parts[0];
                return match parts[1] {
                    "status" => Some(continuity_mcp::read_status(context, agent_name).await),
                    "dream" => Some(continuity_mcp::read_dream(context, agent_name).await),
                    "guidebook" => Some(continuity_mcp::read_guidebook(context, agent_name).await),
                    _ => None,
                };
            }
        }
        None
    }
}

mod continuity_mcp {
    use crate::*;

    pub async fn read_status(
        context: &ProjectContext,
        agent_name: &str,
    ) -> Result<String, ToolError> {
        let response =
            ContinuityService::status(context, &StatusAgent::default()).map_err(Error::from)?;

        let md = match response {
            ContinuityResponse::Status(table) => {
                format!("# Status — {agent_name}\n\n```\n{table}\n```\n")
            }
            _ => format!("# Status — {agent_name}\n\nNo data.\n"),
        };
        Ok(md)
    }

    pub async fn read_dream(
        context: &ProjectContext,
        agent_name: &str,
    ) -> Result<String, ToolError> {
        let response = ContinuityService::dream(
            context,
            &DreamAgent::builder()
                .agent(AgentName::new(agent_name))
                .build(),
            &DreamOverrides::default(),
        )
        .await
        .map_err(Error::from)?;

        let md = match response {
            ContinuityResponse::Dreaming(ctx) => DreamTemplate::new(&ctx).to_string(),
            _ => format!("# Dream — {agent_name}\n\nNo context.\n"),
        };
        Ok(md)
    }

    pub async fn read_guidebook(
        context: &ProjectContext,
        agent_name: &str,
    ) -> Result<String, ToolError> {
        let response = ContinuityService::guidebook(
            context,
            &GuidebookAgent::builder()
                .agent(AgentName::new(agent_name))
                .build(),
            &DreamOverrides::default(),
        )
        .map_err(Error::from)?;

        let md = match response {
            ContinuityResponse::Guidebook(ctx) => GuidebookTemplate::new(&ctx).to_string(),
            _ => format!("# Guidebook — {agent_name}\n\nNo context.\n"),
        };
        Ok(md)
    }

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<WakeAgent>::def(
                ContinuityRequestType::WakeAgent,
                "Wake an agent — restore identity and begin a session",
            ),
            Tool::<DreamAgent>::def(
                ContinuityRequestType::DreamAgent,
                "Restore an agent's full identity and cognitive context",
            ),
            Tool::<IntrospectAgent>::def(
                ContinuityRequestType::IntrospectAgent,
                "Look inward before context compacts — consolidate what matters",
            ),
            Tool::<ReflectAgent>::def(
                ContinuityRequestType::ReflectAgent,
                "Pause on something significant",
            ),
            Tool::<SenseContent>::def(
                ContinuityRequestType::SenseContent,
                "Receive and interpret something from outside your cognitive loop",
            ),
            Tool::<SleepAgent>::def(
                ContinuityRequestType::SleepAgent,
                "End a session — capture continuity before resting",
            ),
            Tool::<GuidebookAgent>::def(
                ContinuityRequestType::GuidebookAgent,
                "Read the cognitive guidebook — learn how your tools work",
            ),
            Tool::<EmergeAgent>::def(
                ContinuityRequestType::EmergeAgent,
                "Bring a new agent into existence with full ceremony",
            ),
            Tool::<RecedeAgent>::def(
                ContinuityRequestType::RecedeAgent,
                "Retire an agent — honor their contributions and let them go",
            ),
            Tool::<StatusAgent>::def(
                ContinuityRequestType::StatusAgent,
                "See an agent's full cognitive dashboard",
            ),
        ]
    }

    /// Extract DreamOverrides from the params JSON.
    /// All fields are optional, so missing fields default to None
    /// (which means "use server default").
    fn parse_overrides(params: &str) -> DreamOverrides {
        serde_json::from_str(params).unwrap_or_default()
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

        let request_type: ContinuityRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let overrides = parse_overrides(params);

        match request_type {
            ContinuityRequestType::WakeAgent => {
                let resp =
                    ContinuityService::wake(&context, &serde_json::from_str(params)?, &overrides)
                        .await
                        .map_err(Error::from)?;
                match resp {
                    ContinuityResponse::Waking(ctx) => {
                        let body = DreamTemplate::new(&ctx).to_string();
                        let response = McpResponse::new(body)
                            .hint(Hint::follow_up(
                                "add-cognition",
                                "Record your first impression",
                            ))
                            .hint(Hint::inspect(
                                "oneiros-mcp://agent/{name}/pressure",
                                "Check cognitive pressure levels",
                            ));
                        Ok(response)
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            ContinuityRequestType::DreamAgent => {
                let resp =
                    ContinuityService::dream(&context, &serde_json::from_str(params)?, &overrides)
                        .await
                        .map_err(Error::from)?;
                match resp {
                    ContinuityResponse::Dreaming(ctx) => {
                        let body = DreamTemplate::new(&ctx).to_string();
                        let response = McpResponse::new(body)
                            .hint(Hint::follow_up(
                                "add-cognition",
                                "Record your first impression",
                            ))
                            .hint(Hint::inspect(
                                "oneiros-mcp://agent/{name}/pressure",
                                "Check cognitive pressure levels",
                            ));
                        Ok(response)
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            ContinuityRequestType::IntrospectAgent => {
                let resp = ContinuityService::introspect(
                    &context,
                    &serde_json::from_str(params)?,
                    &overrides,
                )
                .await
                .map_err(Error::from)?;
                match resp {
                    ContinuityResponse::Introspecting(ctx) => {
                        let pressures = RelevantPressures::from_pressures(
                            ctx.pressures.iter().map(|r| r.pressure.clone()).collect(),
                        );
                        let body = IntrospectTemplate::new(&ctx.agent, pressures).to_string();
                        let response = McpResponse::new(body)
                            .hint(Hint::suggest("add-memory", "Consolidate what matters"));
                        Ok(response)
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            ContinuityRequestType::ReflectAgent => {
                let resp =
                    ContinuityService::reflect(&context, &serde_json::from_str(params)?, &overrides)
                        .await
                        .map_err(Error::from)?;
                match resp {
                    ContinuityResponse::Reflecting(ctx) => {
                        let pressures = RelevantPressures::from_pressures(
                            ctx.pressures.iter().map(|r| r.pressure.clone()).collect(),
                        );
                        let body = ReflectTemplate::new(&ctx.agent, pressures).to_string();
                        let response = McpResponse::new(body)
                            .hint(Hint::suggest("add-cognition", "Capture what surfaced"));
                        Ok(response)
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            ContinuityRequestType::SenseContent => {
                let resp =
                    ContinuityService::sense(&context, &serde_json::from_str(params)?, &overrides)
                        .await
                        .map_err(Error::from)?;
                match resp {
                    ContinuityResponse::Sleeping(ctx) => {
                        let pressures = RelevantPressures::from_pressures(
                            ctx.pressures.iter().map(|r| r.pressure.clone()).collect(),
                        );
                        let body = SenseTemplate::new(&ctx.agent, "", pressures).to_string();
                        Ok(McpResponse::new(body))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            ContinuityRequestType::SleepAgent => {
                let resp =
                    ContinuityService::sleep(&context, &serde_json::from_str(params)?, &overrides)
                        .await
                        .map_err(Error::from)?;
                match resp {
                    ContinuityResponse::Sleeping(ctx) => Ok(McpResponse::new(format!(
                        "Session ended for **{}**. Rest well.",
                        ctx.agent.name
                    ))),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            ContinuityRequestType::GuidebookAgent => {
                let resp = ContinuityService::guidebook(
                    &context,
                    &serde_json::from_str(params)?,
                    &overrides,
                )
                .map_err(Error::from)?;
                match resp {
                    ContinuityResponse::Guidebook(ctx) => {
                        Ok(McpResponse::new(GuidebookTemplate::new(&ctx).to_string()))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            ContinuityRequestType::EmergeAgent => {
                let resp =
                    ContinuityService::emerge(&context, &serde_json::from_str(params)?, &overrides)
                        .await
                        .map_err(Error::from)?;
                match resp {
                    ContinuityResponse::Emerged(ctx) => {
                        let body = DreamTemplate::new(&ctx).to_string();
                        let response = McpResponse::new(body)
                            .hint(Hint::follow_up(
                                "add-cognition",
                                "Record your first impression",
                            ))
                            .hint(Hint::suggest(
                                "wake-agent",
                                "Begin a session for this agent",
                            ));
                        Ok(response)
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            ContinuityRequestType::RecedeAgent => {
                let resp = ContinuityService::recede(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    ContinuityResponse::Receded(name) => {
                        Ok(McpResponse::new(format!("Agent retired: {name}")))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            ContinuityRequestType::StatusAgent => {
                let request: StatusAgent = serde_json::from_str(params).unwrap_or_default();
                let resp = ContinuityService::status(&context, &request).map_err(Error::from)?;
                match resp {
                    ContinuityResponse::Status(table) => {
                        Ok(McpResponse::new(format!("```\n{table}\n```")))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
        }
    }
}
