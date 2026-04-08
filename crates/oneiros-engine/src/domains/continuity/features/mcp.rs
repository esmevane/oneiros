use crate::*;

pub struct ContinuityTools;

impl ContinuityTools {
    pub const fn defs(&self) -> &'static [ToolDef] {
        continuity_mcp::tool_defs()
    }

    pub const fn names(&self) -> &'static [&'static str] {
        continuity_mcp::tool_names()
    }

    pub async fn dispatch(
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

    pub const fn tool_defs() -> &'static [ToolDef] {
        &[
            ToolDef {
                name: "wake",
                description: "Wake an agent — restore identity and begin a session",
                input_schema: schema_for::<WakeAgent>,
            },
            ToolDef {
                name: "dream",
                description: "Restore an agent's full identity and cognitive context",
                input_schema: schema_for::<DreamAgent>,
            },
            ToolDef {
                name: "introspect",
                description: "Look inward before context compacts — consolidate what matters",
                input_schema: schema_for::<IntrospectAgent>,
            },
            ToolDef {
                name: "reflect",
                description: "Pause on something significant",
                input_schema: schema_for::<ReflectAgent>,
            },
            ToolDef {
                name: "sense",
                description: "Receive and interpret something from outside your cognitive loop",
                input_schema: schema_for::<SenseContent>,
            },
            ToolDef {
                name: "sleep",
                description: "End a session — capture continuity before resting",
                input_schema: schema_for::<SleepAgent>,
            },
            ToolDef {
                name: "guidebook",
                description: "Read the cognitive guidebook — learn how your tools work",
                input_schema: schema_for::<GuidebookAgent>,
            },
            ToolDef {
                name: "emerge",
                description: "Bring a new agent into existence with full ceremony",
                input_schema: schema_for::<EmergeAgent>,
            },
            ToolDef {
                name: "recede",
                description: "Retire an agent — honor their contributions and let them go",
                input_schema: schema_for::<RecedeAgent>,
            },
            ToolDef {
                name: "status",
                description: "See an agent's full cognitive dashboard",
                input_schema: schema_for::<StatusAgent>,
            },
        ]
    }

    pub const fn tool_names() -> &'static [&'static str] {
        &[
            "wake",
            "dream",
            "introspect",
            "reflect",
            "sense",
            "sleep",
            "guidebook",
            "emerge",
            "recede",
            "status",
        ]
    }

    /// Extract DreamOverrides from the params JSON.
    /// All fields are optional, so missing fields default to None
    /// (which means "use server default").
    fn parse_overrides(params: &str) -> DreamOverrides {
        serde_json::from_str(params).unwrap_or_default()
    }

    pub async fn dispatch(
        context: &ProjectContext,
        tool_name: &str,
        params: &str,
    ) -> Result<serde_json::Value, ToolError> {
        let overrides = parse_overrides(params);

        let value = match tool_name {
            "wake" => {
                ContinuityService::wake(context, &serde_json::from_str(params)?, &overrides).await
            }
            "dream" => {
                ContinuityService::dream(context, &serde_json::from_str(params)?, &overrides).await
            }
            "introspect" => {
                ContinuityService::introspect(context, &serde_json::from_str(params)?, &overrides)
                    .await
            }
            "reflect" => {
                ContinuityService::reflect(context, &serde_json::from_str(params)?, &overrides)
                    .await
            }
            "sense" => {
                ContinuityService::sense(context, &serde_json::from_str(params)?, &overrides).await
            }
            "sleep" => {
                ContinuityService::sleep(context, &serde_json::from_str(params)?, &overrides).await
            }
            "guidebook" => Ok(ContinuityService::guidebook(
                context,
                &serde_json::from_str(params)?,
                &overrides,
            )
            .map_err(Error::from)?),
            "emerge" => {
                ContinuityService::emerge(context, &serde_json::from_str(params)?, &overrides).await
            }
            "recede" => ContinuityService::recede(context, &serde_json::from_str(params)?).await,
            "status" => {
                let request: StatusAgent = serde_json::from_str(params).unwrap_or_default();
                Ok(ContinuityService::status(context, &request).map_err(Error::from)?)
            }
            _ => return Err(ToolError::UnknownTool(tool_name.to_string())),
        }
        .map_err(Error::from)?;

        Ok(serde_json::to_value(value)?)
    }
}
