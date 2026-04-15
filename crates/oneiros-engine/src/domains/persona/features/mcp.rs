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
    ) -> Result<McpResponse, ToolError> {
        persona_mcp::dispatch(context, tool_name, params).await
    }
}

mod persona_mcp {
    use crate::*;

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<SetPersona>::def(PersonaRequestType::SetPersona, "Define a category of agent"),
            Tool::<GetPersona>::def(PersonaRequestType::GetPersona, "Look up an agent category"),
            Tool::<ListPersonas>::def(PersonaRequestType::ListPersonas, "See all agent categories"),
            Tool::<RemovePersona>::def(
                PersonaRequestType::RemovePersona,
                "Remove an agent category",
            ),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<McpResponse, ToolError> {
        let request_type: PersonaRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            PersonaRequestType::SetPersona => {
                let resp = PersonaService::set(context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    PersonaResponse::PersonaSet(name) => {
                        Ok(McpResponse::new(format!("Persona set: {name}")))
                    }
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            PersonaRequestType::GetPersona => {
                let resp = PersonaService::get(context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    PersonaResponse::PersonaDetails(wrapped) => {
                        let p = &wrapped.data;
                        Ok(McpResponse::new(format!(
                            "**name:** {}\n**description:** {}\n",
                            p.name, p.description
                        )))
                    }
                    PersonaResponse::NoPersonas => Ok(McpResponse::new("Persona not found.")),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            PersonaRequestType::ListPersonas => {
                let resp = PersonaService::list(context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    PersonaResponse::Personas(listed) => {
                        let mut body = format!("{} of {} total\n\n", listed.len(), listed.total);
                        for wrapped in &listed.items {
                            body.push_str(&format!("- {}\n", wrapped.data.name));
                        }
                        Ok(McpResponse::new(body))
                    }
                    PersonaResponse::NoPersonas => Ok(McpResponse::new("No personas.")),
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
            PersonaRequestType::RemovePersona => {
                let resp = PersonaService::remove(context, &serde_json::from_str(params)?)
                    .await
                    .map_err(Error::from)?;
                match resp {
                    PersonaResponse::PersonaRemoved(name) => {
                        Ok(McpResponse::new(format!("Persona removed: {name}")))
                    }
                    other => Ok(McpResponse::new(format!("{other:?}"))),
                }
            }
        }
    }
}
