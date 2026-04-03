use crate::*;

pub struct ActorTools;

impl ActorTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        actor_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        actor_mcp::tool_names()
    }

    pub async fn dispatch(
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

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "create_actor",
                description: "Create a new actor in the system",
                input_schema: schema_for::<CreateActor>,
            },
            ToolDef {
                name: "get_actor",
                description: "Look up a specific actor by ID",
                input_schema: schema_for::<GetActor>,
            },
            ToolDef {
                name: "list_actors",
                description: "List all actors in the system",
                input_schema: schema_for::<ListActors>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &["create_actor", "get_actor", "list_actors"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let system = SystemContext::new(context.config.clone());

        let value = match tool_name {
            "create_actor" => ActorService::create(&system, &serde_json::from_str(params)?).await,
            "get_actor" => ActorService::get(&system, &serde_json::from_str(params)?).await,
            "list_actors" => ActorService::list(&system, &serde_json::from_str(params)?).await,
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
