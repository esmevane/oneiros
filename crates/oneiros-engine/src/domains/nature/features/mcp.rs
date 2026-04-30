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
        nature_mcp::resource(context, request).await
    }
}

mod nature_mcp {
    use crate::*;

    pub async fn resource(
        context: &ProjectLog,
        request: &NatureRequest,
    ) -> Result<McpResponse, ToolError> {
        match request {
            NatureRequest::ListNatures(list) => {
                let response = NatureService::list(context, list)
                    .await
                    .map_err(Error::from)?;
                Ok(NatureView::new(response).mcp())
            }
            NatureRequest::GetNature(get) => {
                let response = NatureService::get(context, get)
                    .await
                    .map_err(Error::from)?;
                Ok(NatureView::new(response).mcp())
            }
            NatureRequest::SetNature(_) | NatureRequest::RemoveNature(_) => Err(
                ToolError::NotAResource("Mutations are tools, not resources".to_string()),
            ),
        }
    }
}
