use crate::*;

pub struct SensationMcp;

impl SensationMcp {
    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![ResourcePathKind::Sensations.resource_def("Experience sensations")]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![]
    }

    pub async fn resource(
        &self,
        context: &ProjectContext,
        request: &SensationRequest,
    ) -> Result<McpResponse, ToolError> {
        sensation_mcp::resource(context, request).await
    }
}

mod sensation_mcp {
    use crate::*;

    pub async fn resource(
        context: &ProjectContext,
        request: &SensationRequest,
    ) -> Result<McpResponse, ToolError> {
        match request {
            SensationRequest::ListSensations(list) => {
                let response = SensationService::list(context, list)
                    .await
                    .map_err(Error::from)?;
                Ok(SensationView::new(response).mcp())
            }
            SensationRequest::GetSensation(get) => {
                let response = SensationService::get(context, get)
                    .await
                    .map_err(Error::from)?;
                Ok(SensationView::new(response).mcp())
            }
            SensationRequest::SetSensation(_) | SensationRequest::RemoveSensation(_) => Err(
                ToolError::NotAResource("Mutations are tools, not resources".to_string()),
            ),
        }
    }
}
