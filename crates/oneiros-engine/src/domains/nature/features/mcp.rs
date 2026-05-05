use crate::*;

pub struct NatureMcp;

impl NatureMcp {
    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![ResourcePathKind::Natures.resource_def("Connection natures")]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![]
    }

    pub async fn resource(
        &self,
        context: &ProjectLog,
        request: &NatureRequest,
    ) -> Result<McpResponse, ToolError> {
        let scope = context.scope().map_err(Error::from)?;
        nature_mcp::resource(scope, request).await
    }
}

mod nature_mcp {
    use crate::*;

    pub async fn resource(
        scope: &Scope<AtBookmark>,
        request: &NatureRequest,
    ) -> Result<McpResponse, ToolError> {
        match request {
            NatureRequest::ListNatures(list) => {
                let response = NatureService::list(scope, list)
                    .await
                    .map_err(Error::from)?;
                Ok(NatureView::new(response).mcp())
            }
            NatureRequest::GetNature(get) => {
                let response = NatureService::get(scope, get).await.map_err(Error::from)?;
                Ok(NatureView::new(response).mcp())
            }
            NatureRequest::SetNature(_) | NatureRequest::RemoveNature(_) => Err(
                ToolError::NotAResource("Mutations are tools, not resources".to_string()),
            ),
        }
    }
}
