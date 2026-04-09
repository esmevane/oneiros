use crate::*;

pub struct PersonaTools;

impl PersonaTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        persona_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        persona_mcp::dispatch(context, tool_name, params).await
    }
}

mod persona_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<SetPersona>::new(PersonaRequestType::SetPersona, "Define a category of agent")
                .def(),
            Tool::<GetPersona>::new(PersonaRequestType::GetPersona, "Look up an agent category")
                .def(),
            Tool::<ListPersonas>::new(PersonaRequestType::ListPersonas, "See all agent categories")
                .def(),
            Tool::<RemovePersona>::new(
                PersonaRequestType::RemovePersona,
                "Remove an agent category",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: PersonaRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let value = match request_type {
            PersonaRequestType::SetPersona => {
                PersonaService::set(context, &serde_json::from_str(params)?).await
            }
            PersonaRequestType::GetPersona => {
                PersonaService::get(context, &serde_json::from_str(params)?).await
            }
            PersonaRequestType::ListPersonas => {
                PersonaService::list(context, &serde_json::from_str(params)?).await
            }
            PersonaRequestType::RemovePersona => {
                PersonaService::remove(context, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
