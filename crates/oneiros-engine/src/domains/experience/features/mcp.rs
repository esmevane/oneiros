use crate::*;

pub(crate) struct ExperienceMcp;

impl ExperienceMcp {
    pub(crate) fn defs(&self) -> Vec<ToolDef> {
        experience_mcp::tool_defs()
    }

    #[expect(deprecated)]
    pub(crate) async fn dispatch(
        &self,
        context: &ProjectLog,
        mailbox: &Mailbox,
        tool_name: &ToolName,
        params: &serde_json::Value,
    ) -> Result<McpResponse, ToolError> {
        experience_mcp::dispatch(context, mailbox, tool_name, params).await
    }

    pub(crate) fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![ResourcePathKind::Experience.template_def("A specific experience")]
    }

    #[expect(deprecated)]
    pub(crate) async fn resource(
        &self,
        context: &ProjectLog,
        request: &ExperienceRequest,
    ) -> Result<McpResponse, ToolError> {
        experience_mcp::resource(context, request).await
    }
}

mod experience_mcp {
    use crate::*;

    pub(crate) fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<CreateExperience>::new(
                ExperienceRequestType::CreateExperience,
                "Mark a meaningful moment",
            )
            .def(),
        ]
    }

    #[expect(deprecated)]
    pub(crate) async fn dispatch(
        context: &ProjectLog,
        mailbox: &Mailbox,
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
                let scope = context.scope().map_err(Error::from)?;
                let response = ExperienceService::create(scope, mailbox, &creation)
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

    #[expect(deprecated)]
    pub(crate) async fn resource(
        context: &ProjectLog,
        request: &ExperienceRequest,
    ) -> Result<McpResponse, ToolError> {
        let scope = context.scope().map_err(Error::from)?;
        let response = match request {
            ExperienceRequest::GetExperience(get) => ExperienceService::get(scope, get)
                .await
                .map_err(Error::from)?,
            ExperienceRequest::ListExperiences(listing) => ExperienceService::list(scope, listing)
                .await
                .map_err(Error::from)?,
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
