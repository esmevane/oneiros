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
        context: &ProjectContext,
        request: &TextureRequest,
    ) -> Result<McpResponse, ToolError> {
        texture_mcp::resource(context, request).await
    }
}

mod texture_mcp {
    use crate::*;

    pub async fn resource(
        context: &ProjectContext,
        request: &TextureRequest,
    ) -> Result<McpResponse, ToolError> {
        match request {
            TextureRequest::ListTextures(list) => {
                let response = TextureService::list(context, list)
                    .await
                    .map_err(Error::from)?;
                Ok(TextureView::new(response).mcp())
            }
            TextureRequest::GetTexture(get) => {
                let response = TextureService::get(context, get)
                    .await
                    .map_err(Error::from)?;
                Ok(TextureView::new(response).mcp())
            }
            TextureRequest::SetTexture(_) | TextureRequest::RemoveTexture(_) => Err(
                ToolError::NotAResource("Mutations are tools, not resources".to_string()),
            ),
        }
    }
}
