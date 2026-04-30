use crate::*;

pub struct ExperienceMcp;

impl ExperienceMcp {
    pub fn defs(&self) -> Vec<ToolDef> {
        experience_mcp::tool_defs()
    }

    pub async fn dispatch(
        &self,
        context: &ProjectLog,
        tool_name: &ToolName,
        params: &serde_json::Value,
    ) -> Result<McpResponse, ToolError> {
        experience_mcp::dispatch(context, tool_name, params).await
    }

    pub fn resources(&self) -> Vec<ResourceDef> {
        vec![]
    }

    pub fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![ResourcePathKind::Experience.into_template("A specific experience")]
    }

    pub async fn resource(
        &self,
        context: &ProjectLog,
        request: &ExperienceRequest,
    ) -> Result<McpResponse, ToolError> {
        experience_mcp::resource(context, request).await
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
        ]
    }

    pub async fn dispatch(
        context: &ProjectLog,
        tool_name: &ToolName,
        params: &serde_json::Value,
    ) -> Result<McpResponse, ToolError> {
        let request_type: ExperienceRequestType = tool_name
            .as_str()
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            ExperienceRequestType::CreateExperience => {
                let creation: CreateExperience = serde_json::from_value(params.clone())?;
                let request = ExperienceRequest::CreateExperience(creation.clone());
                let response = ExperienceService::create(context, &creation)
                    .await
                    .map_err(Error::from)?;
                Ok(ExperienceView::new(response, &request).mcp())
            }
            ExperienceRequestType::GetExperience
            | ExperienceRequestType::ListExperiences
            | ExperienceRequestType::UpdateExperienceDescription
            | ExperienceRequestType::UpdateExperienceSensation => {
                Err(ToolError::UnknownTool(tool_name.to_string()))
            }
        }
    }

    pub async fn resource(
        context: &ProjectLog,
        request: &ExperienceRequest,
    ) -> Result<McpResponse, ToolError> {
        let response = match request {
            ExperienceRequest::GetExperience(get) => ExperienceService::get(context, get)
                .await
                .map_err(Error::from)?,
            ExperienceRequest::ListExperiences(listing) => {
                ExperienceService::list(context, listing)
                    .await
                    .map_err(Error::from)?
            }
            ExperienceRequest::CreateExperience(_)
            | ExperienceRequest::UpdateExperienceDescription(_)
            | ExperienceRequest::UpdateExperienceSensation(_) => {
                return Err(ToolError::NotAResource(
                    "Mutations are tools, not resources".to_string(),
                ));
            }
        };

        match &response {
            ExperienceResponse::NoExperiences => {
                Err(ToolError::NotFound("Experience not found".to_string()))
            }
            _ => Ok(ExperienceView::new(response, request).mcp()),
        }
    }
}
