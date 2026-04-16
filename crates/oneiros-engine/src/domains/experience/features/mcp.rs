use crate::*;

pub struct ExperienceTools;

impl ExperienceTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        experience_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        state: &ServerState,
        config: &Config,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        experience_mcp::dispatch(state, config, tool_name, params).await
    }

    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![ResourceTemplateDef::new(
            "oneiros-mcp://agent/{name}/experiences",
            "agent-experiences",
            "Threads of meaning for an agent",
        )]
    }

    pub async fn read_resource(
        &self,
        context: &ProjectContext,
        path: &str,
    ) -> Option<Result<String, ToolError>> {
        if let Some(rest) = path.strip_prefix("agent/") {
            let parts: Vec<&str> = rest.splitn(2, '/').collect();
            if parts.len() == 2 && parts[1] == "experiences" {
                let agent_name = parts[0];
                return Some(experience_mcp::read_experiences(context, agent_name).await);
            }
        }
        None
    }
}

mod experience_mcp {
    use crate::*;

    pub async fn read_experiences(
        context: &ProjectContext,
        agent_name: &str,
    ) -> Result<String, ToolError> {
        let response = ExperienceService::list(
            context,
            &ListExperiences::builder()
                .agent(AgentName::new(agent_name))
                .build(),
        )
        .await
        .map_err(Error::from)?;

        let mut md = format!("# Experiences — {agent_name}\n\n");
        match response {
            ExperienceResponse::Experiences(listed) => {
                md.push_str(&format!("{} of {} total\n\n", listed.len(), listed.total));
                for wrapped in &listed.items {
                    let e = &wrapped.data;
                    md.push_str(&format!("- **{}** {}\n", e.sensation, e.description));
                }
            }
            ExperienceResponse::NoExperiences => md.push_str("No experiences.\n"),
            _ => {}
        }
        Ok(md)
    }

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateExperience>::def(
                ExperienceRequestType::CreateExperience,
                "Mark a meaningful moment",
            ),
            Tool::<GetExperience>::def(
                ExperienceRequestType::GetExperience,
                "Revisit a specific experience",
            ),
            Tool::<ListExperiences>::def(
                ExperienceRequestType::ListExperiences,
                "Survey threads of meaning",
            ),
            Tool::<UpdateExperienceDescription>::def(
                ExperienceRequestType::UpdateExperienceDescription,
                "Refine an experience's description",
            ),
            Tool::<UpdateExperienceSensation>::def(
                ExperienceRequestType::UpdateExperienceSensation,
                "Change an experience's sensation",
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

        let request_type: ExperienceRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            ExperienceRequestType::CreateExperience => {
                let resp = ExperienceService::create(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    ExperienceResponse::ExperienceCreated(wrapped) => {
                        let e = &wrapped.data;
                        let ref_token = wrapped.meta().ref_token();
                        let mut response = McpResponse::new(format!(
                            "Experience marked: **{}** — {}",
                            e.sensation, e.description
                        ));
                        if let Some(rt) = ref_token {
                            response = response.hint(Hint::suggest(
                                format!("create-connection {rt} <target>"),
                                "Link to something related",
                            ));
                        }
                        Ok(response)
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            ExperienceRequestType::GetExperience => {
                let resp = ExperienceService::get(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    ExperienceResponse::ExperienceDetails(wrapped) => {
                        let e = &wrapped.data;
                        let body = format!(
                            "**sensation:** {}\n**description:** {}\n",
                            e.sensation, e.description
                        );
                        Ok(McpResponse::new(body))
                    }
                    ExperienceResponse::NoExperiences => {
                        Ok(McpResponse::new("Experience not found."))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            ExperienceRequestType::ListExperiences => {
                let resp = ExperienceService::list(&context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    ExperienceResponse::Experiences(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for wrapped in &listed.items {
                            let e = &wrapped.data;
                            body.push_str(&format!("- **{}** {}\n", e.sensation, e.description));
                        }
                        Ok(McpResponse::new(body))
                    }
                    ExperienceResponse::NoExperiences => Ok(McpResponse::new("No experiences.")),
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            ExperienceRequestType::UpdateExperienceDescription => {
                let resp =
                    ExperienceService::update_description(&context, &serde_json::from_str(params)?)
                        .await
                        .map_err(Error::from)?;
                match resp {
                    ExperienceResponse::ExperienceUpdated(wrapped) => {
                        let e = &wrapped.data;
                        Ok(McpResponse::new(format!(
                            "Experience updated: **{}** — {}",
                            e.sensation, e.description
                        )))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
            ExperienceRequestType::UpdateExperienceSensation => {
                let resp =
                    ExperienceService::update_sensation(&context, &serde_json::from_str(params)?)
                        .await
                        .map_err(Error::from)?;
                match resp {
                    ExperienceResponse::ExperienceUpdated(wrapped) => {
                        let e = &wrapped.data;
                        Ok(McpResponse::new(format!(
                            "Experience sensation updated: **{}** — {}",
                            e.sensation, e.description
                        )))
                    }
                    _ => Ok(McpResponse::new("Operation completed.")),
                }
            }
        }
    }
}
