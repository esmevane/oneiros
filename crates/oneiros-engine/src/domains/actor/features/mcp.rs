use crate::*;

pub struct ActorMcp;

impl ActorMcp {
    pub fn defs(&self) -> Vec<ToolDef> {
        actor_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectLog,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        actor_mcp::dispatch(context, tool_name, params).await
    }
}

mod actor_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
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

    pub async fn dispatch(
        context: &ProjectLog,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: ActorRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let scope = ComposeScope::new(context.config.clone())
            .host()
            .map_err(Error::from)?;
        let system = scope.host_log();

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
