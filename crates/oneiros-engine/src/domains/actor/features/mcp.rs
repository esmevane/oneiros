use crate::*;

pub struct ActorTools;

impl ActorTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        actor_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        actor_mcp::dispatch(context, tool_name, params).await
    }
}

mod actor_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateActor>::def(
                ActorRequestType::CreateActor,
                "Create a new actor in the system",
            ),
            Tool::<GetActor>::def(ActorRequestType::GetActor, "Look up a specific actor by ID"),
            Tool::<ListActors>::def(
                ActorRequestType::ListActors,
                "List all actors in the system",
            ),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        let request_type: ActorRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let system = SystemContext::new(context.config.clone());

        match request_type {
            ActorRequestType::CreateActor => {
                let resp = ActorService::create(&system, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    ActorResponse::Created(wrapped) => Ok(McpResponse::new(format!(
                        "Actor created: {}",
                        wrapped.data.id
                    ))),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            ActorRequestType::GetActor => {
                let resp = ActorService::get(&system, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    ActorResponse::Found(wrapped) => {
                        Ok(McpResponse::new(format!("**id:** {}", wrapped.data.id)))
                    }
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            ActorRequestType::ListActors => {
                let resp = ActorService::list(&system, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    ActorResponse::Listed(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for wrapped in &listed.items {
                            body.push_str(&format!("- {}\n", wrapped.data.id));
                        }
                        Ok(McpResponse::new(body))
                    }
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
        }
    }
}
