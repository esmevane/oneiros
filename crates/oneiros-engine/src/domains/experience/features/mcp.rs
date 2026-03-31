use crate::*;

pub struct ExperienceTools;

impl ExperienceTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        experience_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        experience_mcp::tool_names()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        experience_mcp::dispatch(context, tool_name, params).await
    }
}

mod experience_mcp {
    use crate::*;

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "create_experience",
                description: "Mark a meaningful moment",
                input_schema: schema_for::<CreateExperience>,
            },
            ToolDef {
                name: "get_experience",
                description: "Revisit a specific experience",
                input_schema: schema_for::<GetExperience>,
            },
            ToolDef {
                name: "list_experiences",
                description: "Survey threads of meaning",
                input_schema: schema_for::<ListExperiences>,
            },
            ToolDef {
                name: "update_experience_description",
                description: "Refine an experience's description",
                input_schema: schema_for::<UpdateExperienceDescription>,
            },
            ToolDef {
                name: "update_experience_sensation",
                description: "Change an experience's sensation",
                input_schema: schema_for::<UpdateExperienceSensation>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &[
            "create_experience",
            "get_experience",
            "list_experiences",
            "update_experience_description",
            "update_experience_sensation",
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let value = match tool_name {
            "create_experience" => {
                ExperienceService::create(context, &serde_json::from_str(params)?).await
            }
            "get_experience" => {
                ExperienceService::get(context, &serde_json::from_str(params)?).await
            }
            "list_experiences" => {
                ExperienceService::list(context, &serde_json::from_str(params)?).await
            }
            "update_experience_description" => {
                ExperienceService::update_description(context, &serde_json::from_str(params)?).await
            }
            "update_experience_sensation" => {
                ExperienceService::update_sensation(context, &serde_json::from_str(params)?).await
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
