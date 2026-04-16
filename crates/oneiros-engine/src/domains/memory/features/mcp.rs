use crate::*;

pub struct MemoryMcp;

impl MemoryMcp {
    pub fn defs(&self) -> Vec<ToolDef> {
        memory_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &ToolName,
        params: &serde_json::Value,
    ) -> Result<McpResponse, ToolError> {
        memory_mcp::dispatch(context, tool_name, params).await
    }

    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![ResourcePathKind::Memory.into_template("A specific memory")]
    }

    pub async fn resource(
        &self,
        context: &ProjectContext,
        request: &MemoryRequest,
    ) -> Result<McpResponse, ToolError> {
        memory_mcp::resource(context, request).await
    }
}

mod memory_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<AddMemory>::new(
                MemoryRequestType::AddMemory,
                "Consolidate something you've learned",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &ToolName,
        params: &serde_json::Value,
    ) -> Result<McpResponse, ToolError> {
        let request_type: MemoryRequestType = tool_name
            .as_str()
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            MemoryRequestType::AddMemory => {
                let addition: AddMemory = serde_json::from_value(params.clone())?;
                let request = MemoryRequest::AddMemory(addition.clone());
                let response = MemoryService::add(context, &addition)
                    .await
                    .map_err(Error::from)?;
                Ok(MemoryView::new(response, &request).mcp())
            }
            MemoryRequestType::GetMemory | MemoryRequestType::ListMemories => {
                Err(ToolError::UnknownTool(tool_name.to_string()))
            }
        }
    }

    pub async fn resource(
        context: &ProjectContext,
        request: &MemoryRequest,
    ) -> Result<McpResponse, ToolError> {
        let response = match request {
            MemoryRequest::GetMemory(get) => MemoryService::get(context, get)
                .await
                .map_err(Error::from)?,
            MemoryRequest::ListMemories(listing) => MemoryService::list(context, listing)
                .await
                .map_err(Error::from)?,
            MemoryRequest::AddMemory(_) => {
                return Err(ToolError::NotAResource(
                    "Add is a tool, not a resource".to_string(),
                ));
            }
        };

        match &response {
            MemoryResponse::NoMemories => Err(ToolError::NotFound("Memory not found".to_string())),
            _ => Ok(MemoryView::new(response, request).mcp()),
        }
    }
}
