pub mod storage_mcp {
    use schemars::JsonSchema;
    use serde::Deserialize;

    use crate::*;

    #[derive(Deserialize, JsonSchema)]
    struct KeyParam {
        key: StorageKey,
    }

    pub fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "list_storage",
                description: "Browse your archive",
                input_schema: schema_for::<serde_json::Value>,
            },
            ToolDef {
                name: "get_storage",
                description: "Check on a stored artifact",
                input_schema: schema_for::<KeyParam>,
            },
            ToolDef {
                name: "remove_storage",
                description: "Remove a stored artifact",
                input_schema: schema_for::<KeyParam>,
            },
        ]
    }

    pub fn tool_names() -> &'static [&'static str] {
        &["list_storage", "get_storage", "remove_storage"]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "list_storage" => {
                let response =
                    StorageService::list(context).map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "get_storage" => {
                let p: KeyParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = StorageService::show(context, &p.key)
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            "remove_storage" => {
                let p: KeyParam = serde_json::from_str(params)
                    .map_err(|e| ToolError::Parameter(e.to_string()))?;
                let response = StorageService::remove(context, &p.key)
                    .await
                    .map_err(|e| ToolError::Domain(e.to_string()))?;
                serde_json::to_value(response)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        };
        value.map_err(|e| ToolError::Parameter(e.to_string()))
    }
}
