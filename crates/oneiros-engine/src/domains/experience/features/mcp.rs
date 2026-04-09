use crate::*;

pub struct ExperienceTools;

impl ExperienceTools {
    pub fn defs(&self) -> Vec<ToolDef> {
        experience_mcp::tool_defs()
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

    pub fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateExperience>::new(
                ExperienceRequestType::CreateExperience,
                "Mark a meaningful moment",
            )
            .def(),
            Tool::<GetExperience>::new(
                ExperienceRequestType::GetExperience,
                "Revisit a specific experience",
            )
            .def(),
            Tool::<ListExperiences>::new(
                ExperienceRequestType::ListExperiences,
                "Survey threads of meaning",
            )
            .def(),
            Tool::<UpdateExperienceDescription>::new(
                ExperienceRequestType::UpdateExperienceDescription,
                "Refine an experience's description",
            )
            .def(),
            Tool::<UpdateExperienceSensation>::new(
                ExperienceRequestType::UpdateExperienceSensation,
                "Change an experience's sensation",
            )
            .def(),
        ]
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: ExperienceRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let value = match request_type {
            ExperienceRequestType::CreateExperience => {
                ExperienceService::create(context, &serde_json::from_str(params)?).await
            }
            ExperienceRequestType::GetExperience => {
                ExperienceService::get(context, &serde_json::from_str(params)?).await
            }
            ExperienceRequestType::ListExperiences => {
                ExperienceService::list(context, &serde_json::from_str(params)?).await
            }
            ExperienceRequestType::UpdateExperienceDescription => {
                ExperienceService::update_description(context, &serde_json::from_str(params)?).await
            }
            ExperienceRequestType::UpdateExperienceSensation => {
                ExperienceService::update_sensation(context, &serde_json::from_str(params)?).await
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
