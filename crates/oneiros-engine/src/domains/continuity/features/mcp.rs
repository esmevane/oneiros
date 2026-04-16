use crate::*;

pub struct ContinuityMcp;

impl ContinuityMcp {
    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![ResourcePathKind::Status.resource_def("Cross-agent activity table")]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![]
    }

    pub async fn resource(
        &self,
        context: &ProjectContext,
        request: &ContinuityRequest,
    ) -> Result<McpResponse, ToolError> {
        continuity_mcp::resource(context, request).await
    }
}

mod continuity_mcp {
    use crate::*;

    pub async fn resource(
        context: &ProjectContext,
        request: &ContinuityRequest,
    ) -> Result<McpResponse, ToolError> {
        match request {
            ContinuityRequest::StatusAgent(status) => {
                let response = ContinuityService::status(context, status).map_err(Error::from)?;
                Ok(ContinuityView::new(response).mcp())
            }
            _ => Err(ToolError::NotAResource(
                "Only status is a resource; other continuity operations are tools".to_string(),
            )),
        }
    }
}
