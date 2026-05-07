use crate::*;

pub(crate) struct CognitionMcp;

impl CognitionMcp {
    pub(crate) fn defs(&self) -> Vec<ToolDef> {
        cognition_mcp::tool_defs()
    }

    pub(crate) async fn dispatch(
        &self,
        context: &ProjectLog,
        mailbox: &Mailbox,
        tool_name: &ToolName,
        params: &serde_json::Value,
    ) -> Result<McpResponse, ToolError> {
        cognition_mcp::dispatch(context, mailbox, tool_name, params).await
    }

    pub(crate) fn resources(&self) -> Vec<ResourceDef> {
        vec![]
    }

    pub(crate) fn resource_templates(&self) -> Vec<ResourceTemplateDef> {
        vec![ResourcePathKind::Cognition.into_template("A specific cognition")]
    }

    pub(crate) async fn resource(
        &self,
        context: &ProjectLog,
        request: &CognitionRequest,
    ) -> Result<McpResponse, ToolError> {
        cognition_mcp::resource(context, request).await
    }
}

mod cognition_mcp {
    use crate::*;

    pub(crate) fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<AddCognition>::new(CognitionRequestType::AddCognition, "Record a thought").def(),
        ]
    }

    pub(crate) async fn dispatch(
        context: &ProjectLog,
        mailbox: &Mailbox,
        tool_name: &ToolName,
        params: &serde_json::Value,
    ) -> Result<McpResponse, ToolError> {
        let request_type: CognitionRequestType = tool_name
            .as_str()
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        match request_type {
            CognitionRequestType::AddCognition => {
                let addition: AddCognition = serde_json::from_value(params.clone())?;
                let request = CognitionRequest::AddCognition(addition.clone());
                let scope = context.scope().map_err(Error::from)?;
                let response = CognitionService::add(scope, mailbox, &addition)
                    .await
                    .map_err(Error::from)?;
                Ok(CognitionView::new(response, &request).mcp())
            }
            CognitionRequestType::GetCognition | CognitionRequestType::ListCognitions => {
                Err(ToolError::UnknownTool(tool_name.to_string()))
            }
        }
    }

    pub(crate) async fn resource(
        context: &ProjectLog,
        request: &CognitionRequest,
    ) -> Result<McpResponse, ToolError> {
        let scope = context.scope().map_err(Error::from)?;
        let response = match request {
            CognitionRequest::GetCognition(get) => CognitionService::get(scope, get)
                .await
                .map_err(Error::from)?,
            CognitionRequest::ListCognitions(listing) => CognitionService::list(scope, listing)
                .await
                .map_err(Error::from)?,
            CognitionRequest::AddCognition(_) => {
                return Err(ToolError::NotAResource(
                    "Add is a tool, not a resource".to_string(),
                ));
            }
        };

        match &response {
            CognitionResponse::NoCognitions => {
                Err(ToolError::NotFound("Cognition not found".to_string()))
            }
            _ => Ok(CognitionView::new(response, request).mcp()),
        }
    }
}
