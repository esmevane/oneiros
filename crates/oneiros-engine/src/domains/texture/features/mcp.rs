use crate::*;

pub struct TextureMcp;

impl TextureMcp {
    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![ResourcePathKind::Textures.resource_def("Thought textures")]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![]
    }

    pub async fn resource(
        &self,
        context: &ProjectLog,
        request: &TextureRequest,
    ) -> Result<McpResponse, ToolError> {
        let scope = context.scope().map_err(Error::from)?;
        texture_mcp::resource(scope, request).await
    }
}

mod texture_mcp {
    use crate::*;

    pub async fn resource(
        scope: &Scope<AtBookmark>,
        request: &TextureRequest,
    ) -> Result<McpResponse, ToolError> {
        match request {
            TextureRequest::ListTextures(list) => {
                let response = TextureService::list(scope, list)
                    .await
                    .map_err(Error::from)?;
                Ok(TextureView::new(response).mcp())
            }
            TextureRequest::GetTexture(get) => {
                let response = TextureService::get(scope, get).await.map_err(Error::from)?;
                Ok(TextureView::new(response).mcp())
            }
            TextureRequest::SetTexture(_) | TextureRequest::RemoveTexture(_) => Err(
                ToolError::NotAResource("Mutations are tools, not resources".to_string()),
            ),
        }
    }
}
