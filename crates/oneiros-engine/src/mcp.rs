//! MCP tool server — thin rmcp adapter delegating to per-domain dispatchers.
//!
//! Each domain owns its tool catalog in `features/mcp.rs`. This module
//! collects them into an rmcp ServerHandler, routing tool calls to the
//! appropriate domain dispatcher.

use rmcp::model::{CallToolResult, Content, Implementation, ServerCapabilities, ServerInfo, Tool};
use rmcp::{ErrorData, ServerHandler};
use serde_json::json;

use crate::contexts::ProjectContext;
use crate::domains;
use crate::mcp_support::ToolError;

/// Tool description — name + human-readable description.
struct ToolDef {
    name: &'static str,
    description: &'static str,
}

/// All tool definitions across all domains.
fn all_tools() -> Vec<ToolDef> {
    vec![
        // Level
        ToolDef {
            name: "set_level",
            description: "Define how long a kind of memory should be kept",
        },
        ToolDef {
            name: "get_level",
            description: "Look up a memory retention tier",
        },
        ToolDef {
            name: "list_levels",
            description: "See all memory retention tiers",
        },
        ToolDef {
            name: "remove_level",
            description: "Remove a memory retention tier",
        },
        // Texture
        ToolDef {
            name: "set_texture",
            description: "Define a quality of thought",
        },
        ToolDef {
            name: "get_texture",
            description: "Look up a thought category",
        },
        ToolDef {
            name: "list_textures",
            description: "See all thought categories",
        },
        ToolDef {
            name: "remove_texture",
            description: "Remove a thought category",
        },
        // Sensation
        ToolDef {
            name: "set_sensation",
            description: "Define a quality of connection between thoughts",
        },
        ToolDef {
            name: "get_sensation",
            description: "Look up an experience category",
        },
        ToolDef {
            name: "list_sensations",
            description: "See all experience categories",
        },
        ToolDef {
            name: "remove_sensation",
            description: "Remove an experience category",
        },
        // Nature
        ToolDef {
            name: "set_nature",
            description: "Define a kind of relationship between things",
        },
        ToolDef {
            name: "get_nature",
            description: "Look up a relationship category",
        },
        ToolDef {
            name: "list_natures",
            description: "See all relationship categories",
        },
        ToolDef {
            name: "remove_nature",
            description: "Remove a relationship category",
        },
        // Persona
        ToolDef {
            name: "set_persona",
            description: "Define a category of agent",
        },
        ToolDef {
            name: "get_persona",
            description: "Look up an agent category",
        },
        ToolDef {
            name: "list_personas",
            description: "See all agent categories",
        },
        ToolDef {
            name: "remove_persona",
            description: "Remove an agent category",
        },
        // Urge
        ToolDef {
            name: "set_urge",
            description: "Define a cognitive drive",
        },
        ToolDef {
            name: "get_urge",
            description: "Look up a cognitive drive",
        },
        ToolDef {
            name: "list_urges",
            description: "See all cognitive drives",
        },
        ToolDef {
            name: "remove_urge",
            description: "Remove a cognitive drive",
        },
        // Agent
        ToolDef {
            name: "create_agent",
            description: "Bring a new agent into the brain",
        },
        ToolDef {
            name: "get_agent",
            description: "Learn about a specific agent",
        },
        ToolDef {
            name: "list_agents",
            description: "See who's here",
        },
        ToolDef {
            name: "update_agent",
            description: "Reshape an agent's identity",
        },
        ToolDef {
            name: "remove_agent",
            description: "Remove an agent from the brain",
        },
        // Cognition
        ToolDef {
            name: "add_cognition",
            description: "Record a thought",
        },
        ToolDef {
            name: "get_cognition",
            description: "Revisit a specific thought",
        },
        ToolDef {
            name: "list_cognitions",
            description: "Review a stream of thoughts",
        },
        // Memory
        ToolDef {
            name: "add_memory",
            description: "Consolidate something you've learned",
        },
        ToolDef {
            name: "get_memory",
            description: "Revisit a specific memory",
        },
        ToolDef {
            name: "list_memories",
            description: "Review what you know",
        },
        // Experience
        ToolDef {
            name: "create_experience",
            description: "Mark a meaningful moment",
        },
        ToolDef {
            name: "get_experience",
            description: "Revisit a specific experience",
        },
        ToolDef {
            name: "list_experiences",
            description: "Survey threads of meaning",
        },
        ToolDef {
            name: "update_experience_description",
            description: "Refine an experience's description",
        },
        ToolDef {
            name: "update_experience_sensation",
            description: "Refine an experience's sensation",
        },
        // Connection
        ToolDef {
            name: "create_connection",
            description: "Draw a line between two related things",
        },
        ToolDef {
            name: "get_connection",
            description: "Examine a specific connection",
        },
        ToolDef {
            name: "list_connections",
            description: "See how things connect",
        },
        ToolDef {
            name: "remove_connection",
            description: "Remove a connection between two entities",
        },
        // Lifecycle
        ToolDef {
            name: "dream",
            description: "Restore an agent's full identity and cognitive context",
        },
        ToolDef {
            name: "introspect",
            description: "Look inward — consolidate what matters",
        },
        ToolDef {
            name: "reflect",
            description: "Pause on something significant",
        },
        ToolDef {
            name: "sense",
            description: "Receive and interpret something from outside",
        },
        ToolDef {
            name: "sleep",
            description: "End a session — capture continuity before resting",
        },
        // Search
        ToolDef {
            name: "search",
            description: "Search across everything in the brain",
        },
        // Storage
        ToolDef {
            name: "list_storage",
            description: "Browse your archive",
        },
        ToolDef {
            name: "get_storage",
            description: "Check on a stored artifact",
        },
        ToolDef {
            name: "remove_storage",
            description: "Remove a stored artifact",
        },
        // Pressure
        ToolDef {
            name: "get_pressure",
            description: "Check pressure for an agent",
        },
        ToolDef {
            name: "list_pressures",
            description: "See all pressure readings",
        },
    ]
}

/// Domain dispatch table — routes tool names to domain dispatchers.
fn dispatch(
    ctx: &ProjectContext,
    tool_name: &str,
    params: &str,
) -> Result<serde_json::Value, ToolError> {
    // Check each domain's tool catalog
    if domains::level::features::mcp::tool_names().contains(&tool_name) {
        return domains::level::features::mcp::dispatch(ctx, tool_name, params);
    }
    if domains::texture::features::mcp::tool_names().contains(&tool_name) {
        return domains::texture::features::mcp::dispatch(ctx, tool_name, params);
    }
    if domains::sensation::features::mcp::tool_names().contains(&tool_name) {
        return domains::sensation::features::mcp::dispatch(ctx, tool_name, params);
    }
    if domains::nature::features::mcp::tool_names().contains(&tool_name) {
        return domains::nature::features::mcp::dispatch(ctx, tool_name, params);
    }
    if domains::persona::features::mcp::tool_names().contains(&tool_name) {
        return domains::persona::features::mcp::dispatch(ctx, tool_name, params);
    }
    if domains::urge::features::mcp::tool_names().contains(&tool_name) {
        return domains::urge::features::mcp::dispatch(ctx, tool_name, params);
    }
    if domains::agent::features::mcp::tool_names().contains(&tool_name) {
        return domains::agent::features::mcp::dispatch(ctx, tool_name, params);
    }
    if domains::cognition::features::mcp::tool_names().contains(&tool_name) {
        return domains::cognition::features::mcp::dispatch(ctx, tool_name, params);
    }
    if domains::memory::features::mcp::tool_names().contains(&tool_name) {
        return domains::memory::features::mcp::dispatch(ctx, tool_name, params);
    }
    if domains::experience::features::mcp::tool_names().contains(&tool_name) {
        return domains::experience::features::mcp::dispatch(ctx, tool_name, params);
    }
    if domains::connection::features::mcp::tool_names().contains(&tool_name) {
        return domains::connection::features::mcp::dispatch(ctx, tool_name, params);
    }
    if domains::lifecycle::features::mcp::tool_names().contains(&tool_name) {
        return domains::lifecycle::features::mcp::dispatch(ctx, tool_name, params);
    }
    if domains::search::features::mcp::tool_names().contains(&tool_name) {
        return domains::search::features::mcp::dispatch(ctx, tool_name, params);
    }
    if domains::storage::features::mcp::tool_names().contains(&tool_name) {
        return domains::storage::features::mcp::dispatch(ctx, tool_name, params);
    }
    if domains::pressure::features::mcp::tool_names().contains(&tool_name) {
        return domains::pressure::features::mcp::dispatch(ctx, tool_name, params);
    }

    Err(ToolError::UnknownTool(tool_name.to_string()))
}

/// MCP tool server wrapping the project context.
#[derive(Clone)]
pub struct EngineToolBox {
    ctx: ProjectContext,
}

impl EngineToolBox {
    pub fn new(ctx: ProjectContext) -> Self {
        Self { ctx }
    }
}

impl ServerHandler for EngineToolBox {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_server_info(
            Implementation::new("oneiros-engine", env!("CARGO_PKG_VERSION")),
        )
    }

    async fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, ErrorData> {
        let tools = all_tools()
            .into_iter()
            .map(|t| {
                let mut tool = Tool::default();
                tool.name = t.name.into();
                tool.description = Some(t.description.into());
                tool.input_schema = serde_json::from_value(json!({
                    "type": "object",
                    "properties": {},
                }))
                .unwrap();
                tool
            })
            .collect();

        Ok(rmcp::model::ListToolsResult {
            tools,
            next_cursor: None,
            meta: None,
        })
    }

    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_name = request.name.as_ref();
        let params = serde_json::to_string(&request.arguments.unwrap_or_default())
            .unwrap_or_else(|_| "{}".to_string());

        match dispatch(&self.ctx, tool_name, &params) {
            Ok(value) => Ok(CallToolResult::success(vec![
                Content::json(value).expect("content"),
            ])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }
}
