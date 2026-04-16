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
        context: &ProjectContext,
        request: &PersonaRequest,
    ) -> Result<McpResponse, ToolError> {
        persona_mcp::resource(context, request).await
    }
}

mod persona_mcp {
    use crate::*;

    pub async fn resource(
        context: &ProjectContext,
        request: &PersonaRequest,
    ) -> Result<McpResponse, ToolError> {
        match request {
            PersonaRequest::ListPersonas(list) => {
                let response = PersonaService::list(context, list)
                    .await
                    .map_err(Error::from)?;
                Ok(PersonaView::new(response).mcp())
            }
            PersonaRequest::GetPersona(get) => {
                let response = PersonaService::get(context, get)
                    .await
                    .map_err(Error::from)?;
                Ok(PersonaView::new(response).mcp())
            }
            PersonaRequest::SetPersona(_) | PersonaRequest::RemovePersona(_) => Err(
                ToolError::NotAResource("Mutations are tools, not resources".to_string()),
            ),
        }
    }
}
