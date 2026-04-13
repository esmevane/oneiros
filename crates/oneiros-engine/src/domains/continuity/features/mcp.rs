use crate::*;

pub(crate) struct ContinuityTools;

impl ContinuityTools {
    pub(crate) fn defs(&self) -> Vec<ToolDef> {
        continuity_mcp::tool_defs()
    }

    pub(crate) async fn dispatch(
        &self,
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        continuity_mcp::dispatch(context, tool_name, params).await
    }
}

mod continuity_mcp {
    use crate::*;

    pub(crate) fn tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<WakeAgent>::new(
                ContinuityRequestType::WakeAgent,
                "Wake an agent — restore identity and begin a session",
            )
            .def(),
            Tool::<DreamAgent>::new(
                ContinuityRequestType::DreamAgent,
                "Restore an agent's full identity and cognitive context",
            )
            .def(),
            Tool::<IntrospectAgent>::new(
                ContinuityRequestType::IntrospectAgent,
                "Look inward before context compacts — consolidate what matters",
            )
            .def(),
            Tool::<ReflectAgent>::new(
                ContinuityRequestType::ReflectAgent,
                "Pause on something significant",
            )
            .def(),
            Tool::<SenseContent>::new(
                ContinuityRequestType::SenseContent,
                "Receive and interpret something from outside your cognitive loop",
            )
            .def(),
            Tool::<SleepAgent>::new(
                ContinuityRequestType::SleepAgent,
                "End a session — capture continuity before resting",
            )
            .def(),
            Tool::<GuidebookAgent>::new(
                ContinuityRequestType::GuidebookAgent,
                "Read the cognitive guidebook — learn how your tools work",
            )
            .def(),
            Tool::<EmergeAgent>::new(
                ContinuityRequestType::EmergeAgent,
                "Bring a new agent into existence with full ceremony",
            )
            .def(),
            Tool::<RecedeAgent>::new(
                ContinuityRequestType::RecedeAgent,
                "Retire an agent — honor their contributions and let them go",
            )
            .def(),
            Tool::<StatusAgent>::new(
                ContinuityRequestType::StatusAgent,
                "See an agent's full cognitive dashboard",
            )
            .def(),
        ]
    }

    /// Extract DreamOverrides from the params JSON.
    /// All fields are optional, so missing fields default to None
    /// (which means "use server default").
    fn parse_overrides(params: &str) -> DreamOverrides {
        serde_json::from_str(params).unwrap_or_default()
    }

    pub(crate) async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let request_type: ContinuityRequestType = tool_name
            .parse()
            .map_err(|_| ToolError::UnknownTool(tool_name.to_string()))?;

        let overrides = parse_overrides(params);

        let value = match request_type {
            ContinuityRequestType::WakeAgent => {
                ContinuityService::wake(context, &serde_json::from_str(params)?, &overrides).await
            }
            ContinuityRequestType::DreamAgent => {
                ContinuityService::dream(context, &serde_json::from_str(params)?, &overrides).await
            }
            ContinuityRequestType::IntrospectAgent => {
                ContinuityService::introspect(context, &serde_json::from_str(params)?, &overrides)
                    .await
            }
            ContinuityRequestType::ReflectAgent => {
                ContinuityService::reflect(context, &serde_json::from_str(params)?, &overrides)
                    .await
            }
            ContinuityRequestType::SenseContent => {
                ContinuityService::sense(context, &serde_json::from_str(params)?, &overrides).await
            }
            ContinuityRequestType::SleepAgent => {
                ContinuityService::sleep(context, &serde_json::from_str(params)?, &overrides).await
            }
            ContinuityRequestType::GuidebookAgent => Ok(ContinuityService::guidebook(
                context,
                &serde_json::from_str(params)?,
                &overrides,
            )
            .map_err(Error::from)?),
            ContinuityRequestType::EmergeAgent => {
                ContinuityService::emerge(context, &serde_json::from_str(params)?, &overrides).await
            }
            ContinuityRequestType::RecedeAgent => {
                ContinuityService::recede(context, &serde_json::from_str(params)?).await
            }
            ContinuityRequestType::StatusAgent => {
                let request: StatusAgent = serde_json::from_str(params).unwrap_or_default();
                Ok(ContinuityService::status(context, &request).map_err(Error::from)?)
            }
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
