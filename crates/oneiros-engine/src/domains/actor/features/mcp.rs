use crate::*;

pub(crate) struct ActorTools;

impl ActorTools {
    pub(crate) fn defs(&self) -> Vec<ToolDef> {
        actor_mcp::tool_defs()
    }

    pub(crate) async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        actor_mcp::dispatch(context, tool_name, params).await
    }
}

mod actor_mcp {
    use crate::*;

    pub(crate) fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateActor>::new(
                ActorRequestType::CreateActor,
                "Create a new actor in the system",
            )
            .def(),
            Tool::<GetActor>::new(ActorRequestType::GetActor, "Look up a specific actor by ID")
                .def(),
            Tool::<ListActors>::new(
                ActorRequestType::ListActors,
                "List all actors in the system",
            )
            .def(),
        ]
    }

    pub(crate) async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: ActorRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let system = SystemContext::new(context.config.clone());

        let value = match request_type {
            ActorRequestType::CreateActor => {
                ActorService::create(&system, &serde_json::from_str(params)?).await
            }
            ActorRequestType::GetActor => {
                ActorService::get(&system, &serde_json::from_str(params)?).await
            }
            ActorRequestType::ListActors => {
                ActorService::list(&system, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
