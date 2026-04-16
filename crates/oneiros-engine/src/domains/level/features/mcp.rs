use crate::*;

pub struct LevelMcp;

impl LevelMcp {
    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![ResourcePathKind::Levels.resource_def("Memory retention tiers")]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![]
    }

    pub async fn resource(
        &self,
        context: &ProjectContext,
        request: &LevelRequest,
    ) -> Result<McpResponse, ToolError> {
        level_mcp::resource(context, request).await
    }
}

mod level_mcp {
    use crate::*;

    pub async fn resource(
        context: &ProjectContext,
        request: &LevelRequest,
    ) -> Result<McpResponse, ToolError> {
        match request {
            LevelRequest::ListLevels(list) => {
                let response = LevelService::list(context, list)
                    .await
                    .map_err(Error::from)?;
                Ok(LevelView::new(response).mcp())
            }
            LevelRequest::GetLevel(get) => {
                let response = LevelService::get(context, get).await.map_err(Error::from)?;
                Ok(LevelView::new(response).mcp())
            }
            LevelRequest::SetLevel(_) | LevelRequest::RemoveLevel(_) => Err(
                ToolError::NotAResource("Mutations are tools, not resources".to_string()),
            ),
        }
    }
}
