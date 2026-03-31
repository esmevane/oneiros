use crate::*;

pub struct StorageTools;

impl StorageTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        storage_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        storage_mcp::tool_names()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        storage_mcp::dispatch(context, tool_name, params).await
    }
}

mod storage_mcp {
    use crate::*;

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "list_storage",
                description: "Browse your archive",
                input_schema: schema_for::<serde_json::Value>,
            },
            ToolDef {
                name: "get_storage",
                description: "Retrieve a stored artifact",
                input_schema: schema_for::<GetStorage>,
            },
            ToolDef {
                name: "remove_storage",
                description: "Remove a stored artifact",
                input_schema: schema_for::<RemoveStorage>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &["list_storage", "get_storage", "remove_storage"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "list_storage" => StorageService::list(context).await,
            "get_storage" => StorageService::show(context, &serde_json::from_str(params)?).await,
            "remove_storage" => {
                StorageService::remove(context, &serde_json::from_str(params)?).await
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
