use std::path::PathBuf;

use rmcp::model::{RawResource, RawResourceTemplate};

use crate::*;

/// Server-side MCP surface assembly — toolsets, resources, resource templates,
/// and prompts. All methods are stateless associated functions; no `&self`.
pub struct McpServerService;

impl McpServerService {
    /// Tool definitions for the root layer — always available regardless of
    /// which toolset is active.
    pub fn root_tool_defs() -> Vec<ToolDef> {
        vec![
            Tool::<ActivateToolsetRequest>::def(
                ToolsetRequestType::ActivateToolset,
                "Load a toolset by name",
            ),
            Tool::<DeactivateToolsetRequest>::def(
                ToolsetRequestType::DeactivateToolset,
                "Unload the active toolset",
            ),
        ]
    }

    /// Collect tool definitions from all domains, filtered to the cataloged
    /// surface. Get/list tools are excluded — reads are served as resources.
    pub fn all_tools() -> Vec<ToolDef> {
        let domain_tools: Vec<ToolDef> = [
            ActorTools.defs(),
            TenantTools.defs(),
            BrainTools.defs(),
            TicketTools.defs(),
            BookmarkTools.defs(),
            AgentTools.defs(),
            CognitionTools.defs(),
            MemoryTools.defs(),
            ExperienceTools.defs(),
            ConnectionTools.defs(),
            ContinuityTools.defs(),
            SearchTools.defs(),
            StorageTools.defs(),
            PressureTools.defs(),
            LevelTools.defs(),
            TextureTools.defs(),
            SensationTools.defs(),
            NatureTools.defs(),
            PersonaTools.defs(),
            UrgeTools.defs(),
        ]
        .into_iter()
        .flatten()
        .collect();

        domain_tools
            .into_iter()
            .filter(|t| is_cataloged(t.name.as_str()))
            .collect()
    }

    /// Filter the full tool catalog to root + active toolset tools.
    pub fn scoped_tools(active_toolset: Option<Toolset>) -> Vec<ToolDef> {
        let all = Self::all_tools();
        let mut result = Self::root_tool_defs();

        // Always include pressure tools (root layer)
        for tool in &all {
            if is_root_tool(tool.name.as_str()) {
                result.push(ToolDef {
                    name: tool.name.clone(),
                    description: tool.description.clone(),
                    input_schema: tool.input_schema.clone(),
                });
            }
        }

        // Add tools from the active toolset
        if let Some(toolset) = active_toolset {
            for tool in &all {
                if toolset.contains(tool.name.as_str()) {
                    result.push(ToolDef {
                        name: tool.name.clone(),
                        description: tool.description.clone(),
                        input_schema: tool.input_schema.clone(),
                    });
                }
            }
        }

        result
    }

    /// Collect static resources from all domains plus cross-domain vocabulary.
    pub fn all_resources() -> Vec<rmcp::model::Resource> {
        let domain_resources: Vec<ResourceDef> = [
            AgentTools.resources(),
            BookmarkTools.resources(),
            StorageTools.resources(),
        ]
        .into_iter()
        .flatten()
        .collect();

        let mut resources: Vec<rmcp::model::Resource> = domain_resources
            .into_iter()
            .map(Self::resource_def_to_rmcp)
            .collect();

        // Cross-domain: vocabulary is a view across multiple domains
        use rmcp::model::AnnotateAble;
        resources.push(
            RawResource::new("oneiros-mcp://vocabulary", "vocabulary")
                .with_description(
                    "All vocabulary: levels, textures, sensations, natures, personas, urges",
                )
                .with_mime_type("text/markdown")
                .no_annotation(),
        );

        resources
    }

    /// Collect resource templates from all domains.
    pub fn all_resource_templates() -> Vec<rmcp::model::ResourceTemplate> {
        let domain_templates: Vec<ResourceTemplateDef> = [
            AgentTools.resource_templates(),
            CognitionTools.resource_templates(),
            MemoryTools.resource_templates(),
            ExperienceTools.resource_templates(),
            ConnectionTools.resource_templates(),
            ContinuityTools.resource_templates(),
            PressureTools.resource_templates(),
            SearchTools.resource_templates(),
            StorageTools.resource_templates(),
            BookmarkTools.resource_templates(),
        ]
        .into_iter()
        .flatten()
        .collect();

        let mut templates: Vec<rmcp::model::ResourceTemplate> = domain_templates
            .into_iter()
            .map(Self::resource_template_def_to_rmcp)
            .collect();

        // Cross-domain: vocabulary/{kind}
        use rmcp::model::AnnotateAble;
        templates.push(
            RawResourceTemplate::new("oneiros-mcp://vocabulary/{kind}", "vocabulary-kind")
                .with_description("A specific vocabulary kind")
                .with_mime_type("text/markdown")
                .no_annotation(),
        );

        templates
    }

    /// Returns the MCP prompt catalog.
    pub fn prompt_catalog() -> Vec<rmcp::model::Prompt> {
        use rmcp::model::PromptArgument;
        vec![
            rmcp::model::Prompt::new(
                "orient",
                Some("Read pressure, recent activity, suggest next actions"),
                Some(vec![
                    PromptArgument::new("agent")
                        .with_description("The agent to orient")
                        .with_required(true),
                ]),
            ),
            rmcp::model::Prompt::new(
                "toolsets",
                Some("List all toolsets with descriptions and tool counts"),
                None::<Vec<PromptArgument>>,
            ),
        ]
    }

    /// Render the toolsets prompt text as authored markdown.
    pub fn render_toolsets_prompt() -> String {
        let mut md = String::from("# Available Toolsets\n\n");
        md.push_str("Activate a toolset to load its tools into your session.\n\n");

        for toolset in Toolset::ALL {
            let tools = toolset.tool_names();
            md.push_str(&format!(
                "## {} — {}\n{} tools: {}\n\n",
                toolset,
                toolset.description(),
                tools.len(),
                tools.join(", "),
            ));
        }

        md.push_str(
            "Use `activate-toolset` to load one. Use `deactivate-toolset` to return to root.\n",
        );
        md
    }

    /// Convert a [`ResourceDef`] into an rmcp `Resource`.
    pub fn resource_def_to_rmcp(def: ResourceDef) -> rmcp::model::Resource {
        use rmcp::model::AnnotateAble;
        RawResource::new(def.uri, def.name)
            .with_description(def.description.0)
            .with_mime_type(def.mime_type)
            .no_annotation()
    }

    /// Convert a [`ResourceTemplateDef`] into an rmcp `ResourceTemplate`.
    pub fn resource_template_def_to_rmcp(
        def: ResourceTemplateDef,
    ) -> rmcp::model::ResourceTemplate {
        use rmcp::model::AnnotateAble;
        RawResourceTemplate::new(def.uri_template, def.name)
            .with_description(def.description.0)
            .with_mime_type(def.mime_type)
            .no_annotation()
    }
}

pub struct McpConfigService;

impl McpConfigService {
    pub fn init(config: &Config, request: &InitMcp) -> Result<McpConfigResponse, McpConfigError> {
        let token = request
            .token
            .clone()
            .or_else(|| config.token())
            .ok_or(McpConfigError::NoToken)?;

        let address = request.address.unwrap_or(config.service_addr());

        let mcp_json = serde_json::json!({
            "mcpServers": {
                "oneiros-local": {
                    "type": "http",
                    "url": format!("http://{address}/mcp"),
                    "headers": {
                        "Authorization": format!("Bearer {token}")
                    }
                }
            }
        });

        let path = Self::mcp_json_path();

        if path.exists() && !request.yes {
            // In non-interactive contexts (like setup), the caller handles
            // the prompt. Here we just report the file exists.
            return Ok(McpConfigResponse::McpConfigExists(path));
        }

        let content = serde_json::to_string_pretty(&mcp_json)?;
        std::fs::write(&path, content)?;

        Ok(McpConfigResponse::McpConfigWritten(path))
    }

    /// Write the .mcp.json regardless of whether it exists.
    /// Used by setup after the user confirms.
    pub fn write(config: &Config, request: &InitMcp) -> Result<McpConfigResponse, McpConfigError> {
        let mut forced = request.clone();
        forced.yes = true;
        Self::init(config, &forced)
    }

    /// The path to .mcp.json in the current working directory.
    pub fn mcp_json_path() -> PathBuf {
        std::env::current_dir()
            .unwrap_or_default()
            .join(".mcp.json")
    }

    /// Check whether .mcp.json exists.
    pub fn is_configured() -> bool {
        Self::mcp_json_path().exists()
    }
}
