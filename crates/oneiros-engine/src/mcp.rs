//! MCP tool server — exposes all domain services as MCP tools.
//!
//! Uses rmcp's `#[tool_router]` and `#[tool_handler]` macros to register
//! tools and handle the MCP protocol. Each tool delegates to a domain
//! service via the project or system context.

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, Implementation, ServerCapabilities, ServerInfo};
use rmcp::{ErrorData, ServerHandler, tool, tool_handler, tool_router};
use serde::Deserialize;
use serde_json::json;

use crate::contexts::ProjectContext;

/// MCP tool server wrapping the project context.
#[derive(Clone)]
pub struct EngineToolBox {
    ctx: ProjectContext,
    tool_router: ToolRouter<Self>,
}

impl EngineToolBox {
    pub fn new(ctx: ProjectContext) -> Self {
        Self {
            ctx,
            tool_router: Self::tool_router(),
        }
    }

    fn ok(&self, value: impl serde::Serialize) -> Result<CallToolResult, ErrorData> {
        Ok(CallToolResult::success(vec![
            rmcp::model::Content::json(serde_json::to_value(value).unwrap_or(json!(null)))
                .expect("content"),
        ]))
    }

    fn err(&self, e: impl std::fmt::Display) -> Result<CallToolResult, ErrorData> {
        Ok(CallToolResult::error(vec![rmcp::model::Content::text(
            e.to_string(),
        )]))
    }
}

// ── Request types for multi-parameter tools ─────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct CreateAgentParams {
    name: String,
    persona: String,
    description: String,
    prompt: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct UpdateAgentParams {
    name: String,
    persona: String,
    description: String,
    prompt: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct NameParam {
    name: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct IdParam {
    id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct AgentParam {
    agent: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct AddCognitionParams {
    agent: String,
    texture: String,
    content: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct ListCognitionsParams {
    agent: Option<String>,
    texture: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct AddMemoryParams {
    agent: String,
    level: String,
    content: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct ListFilterParams {
    agent: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct CreateExperienceParams {
    agent: String,
    sensation: String,
    description: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct UpdateDescriptionParams {
    id: String,
    description: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct UpdateSensationParams {
    id: String,
    sensation: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct CreateConnectionParams {
    from_entity: String,
    to_entity: String,
    nature: String,
    description: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct ListConnectionsParams {
    entity: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct SearchParams {
    query: String,
    agent: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct SenseParams {
    agent: String,
    content: String,
}

// ── Tool registration ────────────────────────────────────────────

#[tool_router]
impl EngineToolBox {
    // ── Level ────────────────────────────────────────────────────

    #[tool(description = "Define how long a kind of memory should be kept")]
    fn set_level(
        &self,
        Parameters(level): Parameters<crate::domains::level::model::Level>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::level::service::LevelService;
        match LevelService::set(&self.ctx, level) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Look up a memory retention tier")]
    fn get_level(&self, Parameters(p): Parameters<NameParam>) -> Result<CallToolResult, ErrorData> {
        use crate::domains::level::service::LevelService;
        match LevelService::get(&self.ctx, &p.name) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "See all memory retention tiers")]
    fn list_levels(&self) -> Result<CallToolResult, ErrorData> {
        use crate::domains::level::service::LevelService;
        match LevelService::list(&self.ctx) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Remove a memory retention tier")]
    fn remove_level(
        &self,
        Parameters(p): Parameters<NameParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::level::service::LevelService;
        match LevelService::remove(&self.ctx, &p.name) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    // ── Texture ──────────────────────────────────────────────────

    #[tool(description = "Define a quality of thought")]
    fn set_texture(
        &self,
        Parameters(texture): Parameters<crate::domains::texture::model::Texture>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::texture::service::TextureService;
        match TextureService::set(&self.ctx, texture) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Look up a thought category")]
    fn get_texture(
        &self,
        Parameters(p): Parameters<NameParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::texture::service::TextureService;
        match TextureService::get(&self.ctx, &p.name) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "See all thought categories")]
    fn list_textures(&self) -> Result<CallToolResult, ErrorData> {
        use crate::domains::texture::service::TextureService;
        match TextureService::list(&self.ctx) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Remove a thought category")]
    fn remove_texture(
        &self,
        Parameters(p): Parameters<NameParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::texture::service::TextureService;
        match TextureService::remove(&self.ctx, &p.name) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    // ── Sensation ────────────────────────────────────────────────

    #[tool(description = "Define a quality of connection between thoughts")]
    fn set_sensation(
        &self,
        Parameters(s): Parameters<crate::domains::sensation::model::Sensation>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::sensation::service::SensationService;
        match SensationService::set(&self.ctx, s) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Look up an experience category")]
    fn get_sensation(
        &self,
        Parameters(p): Parameters<NameParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::sensation::service::SensationService;
        match SensationService::get(&self.ctx, &p.name) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "See all experience categories")]
    fn list_sensations(&self) -> Result<CallToolResult, ErrorData> {
        use crate::domains::sensation::service::SensationService;
        match SensationService::list(&self.ctx) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Remove an experience category")]
    fn remove_sensation(
        &self,
        Parameters(p): Parameters<NameParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::sensation::service::SensationService;
        match SensationService::remove(&self.ctx, &p.name) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    // ── Nature ───────────────────────────────────────────────────

    #[tool(description = "Define a kind of relationship between things")]
    fn set_nature(
        &self,
        Parameters(n): Parameters<crate::domains::nature::model::Nature>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::nature::service::NatureService;
        match NatureService::set(&self.ctx, n) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Look up a relationship category")]
    fn get_nature(
        &self,
        Parameters(p): Parameters<NameParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::nature::service::NatureService;
        match NatureService::get(&self.ctx, &p.name) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "See all relationship categories")]
    fn list_natures(&self) -> Result<CallToolResult, ErrorData> {
        use crate::domains::nature::service::NatureService;
        match NatureService::list(&self.ctx) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Remove a relationship category")]
    fn remove_nature(
        &self,
        Parameters(p): Parameters<NameParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::nature::service::NatureService;
        match NatureService::remove(&self.ctx, &p.name) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    // ── Persona ──────────────────────────────────────────────────

    #[tool(description = "Define a category of agent")]
    fn set_persona(
        &self,
        Parameters(p): Parameters<crate::domains::persona::model::Persona>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::persona::service::PersonaService;
        match PersonaService::set(&self.ctx, p) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Look up an agent category")]
    fn get_persona(
        &self,
        Parameters(p): Parameters<NameParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::persona::service::PersonaService;
        match PersonaService::get(&self.ctx, &p.name) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "See all agent categories")]
    fn list_personas(&self) -> Result<CallToolResult, ErrorData> {
        use crate::domains::persona::service::PersonaService;
        match PersonaService::list(&self.ctx) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Remove an agent category")]
    fn remove_persona(
        &self,
        Parameters(p): Parameters<NameParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::persona::service::PersonaService;
        match PersonaService::remove(&self.ctx, &p.name) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    // ── Urge ─────────────────────────────────────────────────────

    #[tool(description = "Define a cognitive drive")]
    fn set_urge(
        &self,
        Parameters(u): Parameters<crate::domains::urge::model::Urge>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::urge::service::UrgeService;
        match UrgeService::set(&self.ctx, u) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Look up a cognitive drive")]
    fn get_urge(&self, Parameters(p): Parameters<NameParam>) -> Result<CallToolResult, ErrorData> {
        use crate::domains::urge::service::UrgeService;
        match UrgeService::get(&self.ctx, &p.name) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "See all cognitive drives")]
    fn list_urges(&self) -> Result<CallToolResult, ErrorData> {
        use crate::domains::urge::service::UrgeService;
        match UrgeService::list(&self.ctx) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Remove a cognitive drive")]
    fn remove_urge(
        &self,
        Parameters(p): Parameters<NameParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::urge::service::UrgeService;
        match UrgeService::remove(&self.ctx, &p.name) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    // ── Agent ────────────────────────────────────────────────────

    #[tool(description = "Bring a new agent into the brain")]
    fn create_agent(
        &self,
        Parameters(p): Parameters<CreateAgentParams>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::agent::service::AgentService;
        match AgentService::create(&self.ctx, p.name, p.persona, p.description, p.prompt) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Learn about a specific agent")]
    fn get_agent(&self, Parameters(p): Parameters<NameParam>) -> Result<CallToolResult, ErrorData> {
        use crate::domains::agent::service::AgentService;
        match AgentService::get(&self.ctx, &p.name) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "See who's here")]
    fn list_agents(&self) -> Result<CallToolResult, ErrorData> {
        use crate::domains::agent::service::AgentService;
        match AgentService::list(&self.ctx) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Reshape an agent's identity")]
    fn update_agent(
        &self,
        Parameters(p): Parameters<UpdateAgentParams>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::agent::service::AgentService;
        match AgentService::update(&self.ctx, p.name, p.persona, p.description, p.prompt) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Remove an agent from the brain")]
    fn remove_agent(
        &self,
        Parameters(p): Parameters<NameParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::agent::service::AgentService;
        match AgentService::remove(&self.ctx, &p.name) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    // ── Cognition ────────────────────────────────────────────────

    #[tool(description = "Record a thought")]
    fn add_cognition(
        &self,
        Parameters(p): Parameters<AddCognitionParams>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::cognition::service::CognitionService;
        match CognitionService::add(&self.ctx, p.agent, p.texture, p.content) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Revisit a specific thought")]
    fn get_cognition(
        &self,
        Parameters(p): Parameters<IdParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::cognition::service::CognitionService;
        match CognitionService::get(&self.ctx, &p.id) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Review a stream of thoughts")]
    fn list_cognitions(
        &self,
        Parameters(p): Parameters<ListCognitionsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::cognition::service::CognitionService;
        match CognitionService::list(&self.ctx, p.agent.as_deref(), p.texture.as_deref()) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    // ── Memory ───────────────────────────────────────────────────

    #[tool(description = "Consolidate something you've learned")]
    fn add_memory(
        &self,
        Parameters(p): Parameters<AddMemoryParams>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::memory::service::MemoryService;
        match MemoryService::add(&self.ctx, p.agent, p.level, p.content) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Revisit a specific memory")]
    fn get_memory(&self, Parameters(p): Parameters<IdParam>) -> Result<CallToolResult, ErrorData> {
        use crate::domains::memory::service::MemoryService;
        match MemoryService::get(&self.ctx, &p.id) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Review what you know")]
    fn list_memories(
        &self,
        Parameters(p): Parameters<ListFilterParams>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::memory::service::MemoryService;
        match MemoryService::list(&self.ctx, p.agent.as_deref()) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    // ── Experience ───────────────────────────────────────────────

    #[tool(description = "Mark a meaningful moment")]
    fn create_experience(
        &self,
        Parameters(p): Parameters<CreateExperienceParams>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::experience::service::ExperienceService;
        match ExperienceService::create(&self.ctx, p.agent, p.sensation, p.description) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Revisit a specific experience")]
    fn get_experience(
        &self,
        Parameters(p): Parameters<IdParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::experience::service::ExperienceService;
        match ExperienceService::get(&self.ctx, &p.id) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Survey threads of meaning")]
    fn list_experiences(
        &self,
        Parameters(p): Parameters<ListFilterParams>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::experience::service::ExperienceService;
        match ExperienceService::list(&self.ctx, p.agent.as_deref()) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Refine an experience's description")]
    fn update_experience_description(
        &self,
        Parameters(p): Parameters<UpdateDescriptionParams>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::experience::service::ExperienceService;
        match ExperienceService::update_description(&self.ctx, &p.id, p.description) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Refine an experience's sensation")]
    fn update_experience_sensation(
        &self,
        Parameters(p): Parameters<UpdateSensationParams>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::experience::service::ExperienceService;
        match ExperienceService::update_sensation(&self.ctx, &p.id, p.sensation) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    // ── Connection ───────────────────────────────────────────────

    #[tool(description = "Draw a line between two related things")]
    fn create_connection(
        &self,
        Parameters(p): Parameters<CreateConnectionParams>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::connection::service::ConnectionService;
        match ConnectionService::create(
            &self.ctx,
            p.from_entity,
            p.to_entity,
            p.nature,
            p.description,
        ) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Examine a specific connection")]
    fn get_connection(
        &self,
        Parameters(p): Parameters<IdParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::connection::service::ConnectionService;
        match ConnectionService::get(&self.ctx, &p.id) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "See how things connect")]
    fn list_connections(
        &self,
        Parameters(p): Parameters<ListConnectionsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::connection::service::ConnectionService;
        match ConnectionService::list(&self.ctx, p.entity.as_deref()) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Remove a connection between two entities")]
    fn remove_connection(
        &self,
        Parameters(p): Parameters<IdParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::connection::service::ConnectionService;
        match ConnectionService::remove(&self.ctx, &p.id) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    // ── Lifecycle ────────────────────────────────────────────────

    #[tool(description = "Restore an agent's full identity and cognitive context")]
    fn dream(&self, Parameters(p): Parameters<AgentParam>) -> Result<CallToolResult, ErrorData> {
        use crate::domains::lifecycle::service::LifecycleService;
        match LifecycleService::dream(&self.ctx, &p.agent) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Look inward — consolidate what matters")]
    fn introspect(
        &self,
        Parameters(p): Parameters<AgentParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::lifecycle::service::LifecycleService;
        match LifecycleService::introspect(&self.ctx, &p.agent) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Pause on something significant")]
    fn reflect(&self, Parameters(p): Parameters<AgentParam>) -> Result<CallToolResult, ErrorData> {
        use crate::domains::lifecycle::service::LifecycleService;
        match LifecycleService::reflect(&self.ctx, &p.agent) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Receive and interpret something from outside")]
    fn sense(&self, Parameters(p): Parameters<SenseParams>) -> Result<CallToolResult, ErrorData> {
        use crate::domains::lifecycle::service::LifecycleService;
        match LifecycleService::sense(&self.ctx, &p.agent, &p.content) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "End a session — capture continuity before resting")]
    fn sleep(&self, Parameters(p): Parameters<AgentParam>) -> Result<CallToolResult, ErrorData> {
        use crate::domains::lifecycle::service::LifecycleService;
        match LifecycleService::sleep(&self.ctx, &p.agent) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    // ── Search ───────────────────────────────────────────────────

    #[tool(description = "Search across everything in the brain")]
    fn search(&self, Parameters(p): Parameters<SearchParams>) -> Result<CallToolResult, ErrorData> {
        use crate::domains::search::service::SearchService;
        match SearchService::search(&self.ctx, &p.query, p.agent.as_deref()) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    // ── Storage ──────────────────────────────────────────────────

    #[tool(description = "Browse your archive")]
    fn list_storage(&self) -> Result<CallToolResult, ErrorData> {
        use crate::domains::storage::service::StorageService;
        match StorageService::list(&self.ctx) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Check on a stored artifact")]
    fn get_storage(&self, Parameters(p): Parameters<IdParam>) -> Result<CallToolResult, ErrorData> {
        use crate::domains::storage::service::StorageService;
        match StorageService::get(&self.ctx, &p.id) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "Remove a stored artifact")]
    fn remove_storage(
        &self,
        Parameters(p): Parameters<IdParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::storage::service::StorageService;
        match StorageService::remove(&self.ctx, &p.id) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    // ── Pressure ─────────────────────────────────────────────────

    #[tool(description = "Check pressure for an agent")]
    fn get_pressure(
        &self,
        Parameters(p): Parameters<AgentParam>,
    ) -> Result<CallToolResult, ErrorData> {
        use crate::domains::pressure::service::PressureService;
        match PressureService::get(&self.ctx, &p.agent) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }

    #[tool(description = "See all pressure readings")]
    fn list_pressures(&self) -> Result<CallToolResult, ErrorData> {
        use crate::domains::pressure::service::PressureService;
        match PressureService::list(&self.ctx) {
            Ok(r) => self.ok(r),
            Err(e) => self.err(e),
        }
    }
}

// ── Server handler ───────────────────────────────────────────────

#[tool_handler]
impl ServerHandler for EngineToolBox {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_server_info(
            Implementation::new("oneiros-engine", env!("CARGO_PKG_VERSION")),
        )
    }
}
