use crate::*;

pub struct UrgeMcp;

impl UrgeMcp {
    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![ResourcePathKind::Urges.resource_def("Cognitive drives")]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![]
    }

    pub async fn resource(
        &self,
        context: &ProjectLog,
        request: &UrgeRequest,
    ) -> Result<McpResponse, ToolError> {
        urge_mcp::resource(context, request).await
    }
}

mod urge_mcp {
    use crate::*;

    pub async fn resource(
        context: &ProjectLog,
        request: &UrgeRequest,
    ) -> Result<McpResponse, ToolError> {
        match request {
            UrgeRequest::ListUrges(list) => {
                let response = UrgeService::list(context, list)
                    .await
                    .map_err(Error::from)?;
                Ok(UrgeView::new(response).mcp())
            }
            UrgeRequest::GetUrge(get) => {
                let response = UrgeService::get(context, get).await.map_err(Error::from)?;
                Ok(UrgeView::new(response).mcp())
            }
            UrgeRequest::SetUrge(_) | UrgeRequest::RemoveUrge(_) => Err(ToolError::NotAResource(
                "Mutations are tools, not resources".to_string(),
            )),
        }
    }
}
