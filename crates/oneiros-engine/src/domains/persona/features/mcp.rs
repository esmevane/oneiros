use crate::*;

pub struct PersonaMcp;

impl PersonaMcp {
    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![ResourcePathKind::Personas.resource_def("Agent personas")]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![]
    }

    pub async fn resource(
        &self,
        context: &ProjectLog,
        request: &PersonaRequest,
    ) -> Result<McpResponse, ToolError> {
        let scope = context.scope().map_err(Error::from)?;
        persona_mcp::resource(scope, request).await
    }
}

mod persona_mcp {
    use crate::*;

    pub async fn resource(
        scope: &Scope<AtBookmark>,
        request: &PersonaRequest,
    ) -> Result<McpResponse, ToolError> {
        match request {
            PersonaRequest::ListPersonas(list) => {
                let response = PersonaService::list(scope, list)
                    .await
                    .map_err(Error::from)?;
                Ok(PersonaView::new(response).mcp())
            }
            PersonaRequest::GetPersona(get) => {
                let response = PersonaService::get(scope, get).await.map_err(Error::from)?;
                Ok(PersonaView::new(response).mcp())
            }
            PersonaRequest::SetPersona(_) | PersonaRequest::RemovePersona(_) => Err(
                ToolError::NotAResource("Mutations are tools, not resources".to_string()),
            ),
        }
    }
}
