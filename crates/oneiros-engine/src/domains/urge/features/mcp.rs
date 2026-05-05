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
        let scope = context.scope().map_err(Error::from)?;
        urge_mcp::resource(scope, request).await
    }
}

mod urge_mcp {
    use crate::*;

    pub async fn resource(
        scope: &Scope<AtBookmark>,
        request: &UrgeRequest,
    ) -> Result<McpResponse, ToolError> {
        match request {
            UrgeRequest::ListUrges(list) => {
                let response = UrgeService::list(scope, list).await.map_err(Error::from)?;
                Ok(UrgeView::new(response).mcp())
            }
            UrgeRequest::GetUrge(get) => {
                let response = UrgeService::get(scope, get).await.map_err(Error::from)?;
                Ok(UrgeView::new(response).mcp())
            }
            UrgeRequest::SetUrge(_) | UrgeRequest::RemoveUrge(_) => Err(ToolError::NotAResource(
                "Mutations are tools, not resources".to_string(),
            )),
        }
    }
}
