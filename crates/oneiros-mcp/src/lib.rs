use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use oneiros_model::*;
use oneiros_service::OneirosService;
use rmcp::{
    ErrorData, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router,
};

// Disambiguate from oneiros_model::Resource
use rmcp::model::Resource as McpResource;

/// Pressure urgency threshold for resource subscription notifications.
const PRESSURE_NOTIFICATION_THRESHOLD: f64 = 0.80;

/// MCP tool server for oneiros.
///
/// Each instance is scoped to a capability level expressed by its
/// `OneirosService` variant: `Service` for system-only operations,
/// `Brain` for the full catalog.
///
/// Transports construct the toolbox with the appropriate service variant.
/// The service can be upgraded (e.g. from `Service` to `Brain`) via
/// `upgrade()` — used by the HTTP transport during MCP initialization.
#[derive(Clone)]
pub struct OneirosToolBox {
    state: OneirosService,
    tool_router: ToolRouter<Self>,
    subscriptions: Arc<RwLock<HashSet<String>>>,
}

impl OneirosToolBox {
    /// Create a toolbox starting at system-only capability.
    /// Brain context can be added later via `upgrade()`.
    pub fn system(state: OneirosService) -> Self {
        Self {
            state,
            tool_router: Self::tool_router(),
            subscriptions: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub fn upgrade(&self, token: impl Into<Token>) -> Result<Self, oneiros_service::Error> {
        Ok(Self {
            state: self.state.upgrade(token)?,
            tool_router: self.tool_router.clone(),
            subscriptions: self.subscriptions.clone(),
        })
    }

    /// Access the shared service state.
    pub fn state(&self) -> &OneirosService {
        &self.state
    }

    /// Dispatch a protocol request through the unified service layer.
    ///
    /// Domain errors (NotFound, BadRequest, Conflict) become tool-level
    /// errors visible to the calling agent. Only serialization failures
    /// produce protocol-level `ErrorData`.
    fn dispatch(&self, request: impl Into<Requests>) -> Result<CallToolResult, ErrorData> {
        match self.state.dispatch(request) {
            Ok(response) => {
                let value = serde_json::to_value(&response)
                    .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
                Ok(CallToolResult::structured(value))
            }

            Err(e) => Ok(CallToolResult::error(vec![rmcp::model::Content::text(
                e.to_string(),
            )])),
        }
    }

    /// Parse a `oneiroi://pressure/{agent}` URI and return the agent name.
    fn parse_pressure_uri(uri: &str) -> Option<AgentName> {
        let path = uri.strip_prefix("oneiroi://pressure/")?;
        let agent = path.split('/').next()?;
        if agent.is_empty() {
            return None;
        }
        Some(AgentName::new(agent))
    }

    /// List all pressure resources — one per agent.
    fn list_pressure_resources(&self) -> Result<Vec<McpResource>, ErrorData> {
        let agents = match self
            .state
            .dispatch(AgentRequests::ListAgents(ListAgentsRequest))
        {
            Ok(response) => match response.data {
                Responses::Agent(AgentResponses::AgentsListed(agents)) => agents,
                _ => return Ok(vec![]),
            },
            _ => return Ok(vec![]),
        };

        Ok(agents
            .iter()
            .map(|agent| {
                McpResource::new(
                    RawResource {
                        uri: format!("oneiroi://pressure/{}", agent.name),
                        name: format!("{} pressure", agent.name),
                        title: None,
                        description: Some(format!(
                            "What's building up for {} — urgency scores and the forces behind them",
                            agent.name
                        )),
                        mime_type: Some("application/json".to_string()),
                        size: None,
                        icons: None,
                        meta: None,
                    },
                    None,
                )
            })
            .collect())
    }

    /// Read pressure data for an agent as JSON text content.
    fn read_pressure_resource(&self, uri: &str) -> Result<ReadResourceResult, ErrorData> {
        let agent_name = Self::parse_pressure_uri(uri).ok_or_else(|| {
            ErrorData::invalid_params(format!("Invalid pressure URI: {uri}"), None)
        })?;

        let request = PressureRequests::GetPressure(GetPressureRequest { agent: agent_name });
        let pressures = match self.state.dispatch(request) {
            Ok(response) => match response.data {
                Responses::Pressure(PressureResponses::PressureFound(pressures)) => pressures,
                _ => return Err(ErrorData::internal_error("Unexpected response", None)),
            },
            Err(e) => {
                return Err(ErrorData::invalid_params(e.to_string(), None));
            }
        };

        let json = serde_json::to_string_pretty(&pressures)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;

        Ok(ReadResourceResult::new(vec![
            ResourceContents::text(json, uri).with_mime_type("application/json"),
        ]))
    }

    /// Check all subscribed pressure URIs and return those where any
    /// pressure exceeds the notification threshold.
    pub fn check_pressure_thresholds(&self) -> Vec<String> {
        let subs = match self.subscriptions.read() {
            Ok(s) => s.clone(),
            Err(_) => return vec![],
        };

        let mut triggered = Vec::new();
        for uri in &subs {
            if let Some(agent_name) = Self::parse_pressure_uri(uri) {
                let request =
                    PressureRequests::GetPressure(GetPressureRequest { agent: agent_name });
                if let Ok(response) = self.state.dispatch(request)
                    && let Responses::Pressure(PressureResponses::PressureFound(pressures)) =
                        response.data
                    && pressures
                        .iter()
                        .any(|p| p.urgency() >= PRESSURE_NOTIFICATION_THRESHOLD)
                {
                    triggered.push(uri.clone());
                }
            }
        }
        triggered
    }

    /// Add a subscription URI.
    pub fn subscribe_uri(&self, uri: &str) {
        if let Ok(mut subs) = self.subscriptions.write() {
            subs.insert(uri.to_string());
        }
    }

    /// Remove a subscription URI.
    pub fn unsubscribe_uri(&self, uri: &str) {
        if let Ok(mut subs) = self.subscriptions.write() {
            subs.remove(uri);
        }
    }

    /// Check if there are any active subscriptions.
    pub fn has_subscriptions(&self) -> bool {
        self.subscriptions
            .read()
            .map(|s| !s.is_empty())
            .unwrap_or(false)
    }
}

// ── Tool implementations ────────────────────────────────────────────

#[tool_router]
impl OneirosToolBox {
    // ── Agent ───────────────────────────────────────────────────────

    #[tool(description = "See who's here — discover all agents in this brain")]
    fn list_agents(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(AgentRequests::ListAgents(ListAgentsRequest))
    }

    #[tool(description = "Look up an agent — learn about their identity and purpose")]
    fn get_agent(
        &self,
        Parameters(request): Parameters<GetAgentRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(AgentRequests::GetAgent(request))
    }

    #[tool(
        description = "Bring a new agent into the brain — give them a name, persona, and purpose"
    )]
    fn create_agent(
        &self,
        Parameters(request): Parameters<CreateAgentRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(AgentRequests::CreateAgent(request))
    }

    #[tool(
        description = "Reshape an agent's identity — change their description, prompt, or persona"
    )]
    fn update_agent(
        &self,
        Parameters(request): Parameters<UpdateAgentRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(AgentRequests::UpdateAgent(request))
    }

    #[tool(description = "Remove an agent from the brain — their thoughts and memories remain")]
    fn remove_agent(
        &self,
        Parameters(request): Parameters<RemoveAgentRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(AgentRequests::RemoveAgent(request))
    }

    // ── Cognition ───────────────────────────────────────────────────

    #[tool(description = "Record a thought — capture what you're thinking right now")]
    fn add_cognition(
        &self,
        Parameters(request): Parameters<AddCognitionRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(CognitionRequests::AddCognition(request))
    }

    #[tool(description = "Revisit a specific thought by its ID")]
    fn get_cognition(
        &self,
        Parameters(request): Parameters<GetCognitionRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(CognitionRequests::GetCognition(request))
    }

    #[tool(description = "Review your stream of thoughts, optionally filtered by agent or texture")]
    fn list_cognitions(
        &self,
        Parameters(request): Parameters<ListCognitionsRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(CognitionRequests::ListCognitions(request))
    }

    // ── Connection ──────────────────────────────────────────────────

    #[tool(description = "Draw a line between two things that relate to each other")]
    fn create_connection(
        &self,
        Parameters(request): Parameters<CreateConnectionRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(ConnectionRequests::CreateConnection(request))
    }

    #[tool(description = "Remove a connection between two entities")]
    fn remove_connection(
        &self,
        Parameters(request): Parameters<RemoveConnectionRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(ConnectionRequests::RemoveConnection(request))
    }

    #[tool(description = "Examine a specific connection and what it links")]
    fn get_connection(
        &self,
        Parameters(request): Parameters<GetConnectionRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(ConnectionRequests::GetConnection(request))
    }

    #[tool(
        description = "See how things connect — browse relationships, optionally filtered by nature or entity"
    )]
    fn list_connections(
        &self,
        Parameters(request): Parameters<ListConnectionsRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(ConnectionRequests::ListConnections(request))
    }

    // ── Dreaming ────────────────────────────────────────────────────

    #[tool(
        description = "Assemble an agent's full identity — memories, thoughts, threads, and pressures woven into context"
    )]
    fn dream(
        &self,
        Parameters(request): Parameters<DreamRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(DreamingRequests::Dream(request))
    }

    // ── Event ───────────────────────────────────────────────────────

    #[tool(description = "Restore a brain's history from an exported event stream")]
    fn import_events(
        &self,
        Parameters(request): Parameters<ImportEventsRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(EventRequests::ImportEvents(request))
    }

    #[tool(description = "Rebuild all projections by replaying the full event history")]
    fn replay_events(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(EventRequests::ReplayEvents(ReplayEventsRequest))
    }

    #[tool(description = "Browse the raw event stream underlying the brain")]
    fn list_events(
        &self,
        Parameters(request): Parameters<ListEventsRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(EventRequests::ListEvents(request))
    }

    #[tool(description = "Examine a specific event in the brain's history")]
    fn get_event(
        &self,
        Parameters(request): Parameters<GetEventRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(EventRequests::GetEvent(request))
    }

    #[tool(description = "Package the brain's complete history for transport")]
    fn export_events(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(EventRequests::ExportEvents(ExportEventsRequest))
    }

    // ── Experience ──────────────────────────────────────────────────

    #[tool(description = "Mark a meaningful moment — name the thread connecting your thoughts")]
    fn create_experience(
        &self,
        Parameters(request): Parameters<CreateExperienceRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(ExperienceRequests::CreateExperience(request))
    }

    #[tool(description = "Refine how an experience is described")]
    fn update_experience_description(
        &self,
        Parameters(request): Parameters<UpdateExperienceDescriptionRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(ExperienceRequests::UpdateExperienceDescription(request))
    }

    #[tool(description = "Reclassify the quality of an experience")]
    fn update_experience_sensation(
        &self,
        Parameters(request): Parameters<UpdateExperienceSensationRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(ExperienceRequests::UpdateExperienceSensation(request))
    }

    #[tool(description = "Revisit a specific experience and its connections")]
    fn get_experience(
        &self,
        Parameters(request): Parameters<GetExperienceRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(ExperienceRequests::GetExperience(request))
    }

    #[tool(
        description = "Survey your threads of meaning, optionally filtered by agent or sensation"
    )]
    fn list_experiences(
        &self,
        Parameters(request): Parameters<ListExperiencesRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(ExperienceRequests::ListExperiences(request))
    }

    // ── Introspect ──────────────────────────────────────────────────

    #[tool(
        description = "Look inward — examine what's accumulated and consolidate before context compacts"
    )]
    fn introspect(
        &self,
        Parameters(request): Parameters<IntrospectRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(IntrospectingRequests::Introspect(request))
    }

    // ── Level ───────────────────────────────────────────────────────

    #[tool(description = "Define how long a kind of memory should be kept")]
    fn set_level(&self, Parameters(level): Parameters<Level>) -> Result<CallToolResult, ErrorData> {
        self.dispatch(LevelRequests::SetLevel(level))
    }

    #[tool(description = "Remove a memory retention tier")]
    fn remove_level(
        &self,
        Parameters(request): Parameters<RemoveLevelRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(LevelRequests::RemoveLevel(request))
    }

    #[tool(description = "Look up a memory retention tier and its policy")]
    fn get_level(
        &self,
        Parameters(request): Parameters<GetLevelRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(LevelRequests::GetLevel(request))
    }

    #[tool(description = "See all the ways memories can be retained")]
    fn list_levels(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(LevelRequests::ListLevels(ListLevelsRequest))
    }

    // ── Lifecycle ───────────────────────────────────────────────────

    #[tool(description = "Wake an agent — restore their identity and begin a session")]
    fn wake(
        &self,
        Parameters(request): Parameters<WakeRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(LifecycleRequests::Wake(request))
    }

    #[tool(description = "Put an agent to rest — capture session continuity before closing")]
    fn sleep(
        &self,
        Parameters(request): Parameters<SleepRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(LifecycleRequests::Sleep(request))
    }

    #[tool(description = "Bring a new agent into existence — full ceremony with lifecycle event")]
    fn emerge(
        &self,
        Parameters(request): Parameters<CreateAgentRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(LifecycleRequests::Emerge(request))
    }

    #[tool(description = "Retire an agent — honor their contributions and let them go")]
    fn recede(
        &self,
        Parameters(request): Parameters<RecedeRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(LifecycleRequests::Recede(request))
    }

    // ── Memory ──────────────────────────────────────────────────────

    #[tool(description = "Consolidate something you've learned — store durable knowledge")]
    fn add_memory(
        &self,
        Parameters(request): Parameters<AddMemoryRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(MemoryRequests::AddMemory(request))
    }

    #[tool(description = "Revisit a specific memory")]
    fn get_memory(
        &self,
        Parameters(request): Parameters<GetMemoryRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(MemoryRequests::GetMemory(request))
    }

    #[tool(description = "Review what you know, optionally filtered by agent or retention level")]
    fn list_memories(
        &self,
        Parameters(request): Parameters<ListMemoriesRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(MemoryRequests::ListMemories(request))
    }

    // ── Nature ──────────────────────────────────────────────────────

    #[tool(description = "Define a kind of relationship that can exist between things")]
    fn set_nature(
        &self,
        Parameters(nature): Parameters<Nature>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(NatureRequests::SetNature(nature))
    }

    #[tool(description = "Remove a relationship category")]
    fn remove_nature(
        &self,
        Parameters(request): Parameters<RemoveNatureRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(NatureRequests::RemoveNature(request))
    }

    #[tool(description = "Look up a relationship category and its meaning")]
    fn get_nature(
        &self,
        Parameters(request): Parameters<GetNatureRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(NatureRequests::GetNature(request))
    }

    #[tool(description = "See all the kinds of relationships that can exist")]
    fn list_natures(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(NatureRequests::ListNatures(ListNaturesRequest))
    }

    // ── Persona ─────────────────────────────────────────────────────

    #[tool(description = "Define a category of agent — shared identity and purpose")]
    fn set_persona(
        &self,
        Parameters(persona): Parameters<Persona>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(PersonaRequests::SetPersona(persona))
    }

    #[tool(description = "Remove an agent category")]
    fn remove_persona(
        &self,
        Parameters(request): Parameters<RemovePersonaRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(PersonaRequests::RemovePersona(request))
    }

    #[tool(description = "Look up an agent category and its shared context")]
    fn get_persona(
        &self,
        Parameters(request): Parameters<GetPersonaRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(PersonaRequests::GetPersona(request))
    }

    #[tool(description = "See all the kinds of agents that can exist")]
    fn list_personas(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(PersonaRequests::ListPersonas(ListPersonasRequest))
    }

    // ── Reflect ─────────────────────────────────────────────────────

    #[tool(
        description = "Pause on something significant — capture a moment that shifted your understanding"
    )]
    fn reflect(
        &self,
        Parameters(request): Parameters<ReflectRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(ReflectingRequests::Reflect(request))
    }

    // ── Search ──────────────────────────────────────────────────────

    #[tool(description = "Search across everything — thoughts, memories, experiences, and more")]
    fn search(
        &self,
        Parameters(request): Parameters<SearchRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(SearchRequests::Search(request))
    }

    // ── Sensation ───────────────────────────────────────────────────

    #[tool(description = "Define a quality of connection — how experiences feel")]
    fn set_sensation(
        &self,
        Parameters(sensation): Parameters<Sensation>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(SensationRequests::SetSensation(sensation))
    }

    #[tool(description = "Remove an experience category")]
    fn remove_sensation(
        &self,
        Parameters(request): Parameters<RemoveSensationRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(SensationRequests::RemoveSensation(request))
    }

    #[tool(description = "Look up an experience category and its meaning")]
    fn get_sensation(
        &self,
        Parameters(request): Parameters<GetSensationRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(SensationRequests::GetSensation(request))
    }

    #[tool(description = "See all the ways experiences can feel")]
    fn list_sensations(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(SensationRequests::ListSensations(ListSensationsRequest))
    }

    // ── Sense ───────────────────────────────────────────────────────

    #[tool(description = "Receive something from outside your cognitive loop and interpret it")]
    fn sense(
        &self,
        Parameters(request): Parameters<SenseRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(SenseRequests::Sense(request))
    }

    // ── Storage ─────────────────────────────────────────────────────

    #[tool(description = "Remove a stored artifact from the archive")]
    fn remove_storage(
        &self,
        Parameters(request): Parameters<RemoveStorageRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(StorageRequests::RemoveStorage(request))
    }

    #[tool(description = "Check on a stored artifact — see its metadata")]
    fn get_storage(
        &self,
        Parameters(request): Parameters<GetStorageRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(StorageRequests::GetStorage(request))
    }

    #[tool(description = "Retrieve the contents of a stored artifact")]
    fn get_storage_content(
        &self,
        Parameters(request): Parameters<GetStorageContentRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        match self.state.get_storage_content(&request.key) {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes).into_owned();
                Ok(CallToolResult::success(vec![rmcp::model::Content::text(
                    text,
                )]))
            }
            Err(e) => Ok(CallToolResult::error(vec![rmcp::model::Content::text(
                e.to_string(),
            )])),
        }
    }

    #[tool(description = "Browse your archive of stored artifacts")]
    fn list_storage(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(StorageRequests::ListStorage(ListStorageRequest))
    }

    // ── Texture ─────────────────────────────────────────────────────

    #[tool(description = "Define a quality of thought — how a kind of thinking feels")]
    fn set_texture(
        &self,
        Parameters(texture): Parameters<Texture>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(TextureRequests::SetTexture(texture))
    }

    #[tool(description = "Remove a thought category")]
    fn remove_texture(
        &self,
        Parameters(request): Parameters<RemoveTextureRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(TextureRequests::RemoveTexture(request))
    }

    #[tool(description = "Look up a thought category and its guidance")]
    fn get_texture(
        &self,
        Parameters(request): Parameters<GetTextureRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(TextureRequests::GetTexture(request))
    }

    #[tool(description = "See all the ways thoughts can be textured")]
    fn list_textures(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(TextureRequests::ListTextures(ListTexturesRequest))
    }

    // ── Urge ────────────────────────────────────────────────────────

    #[tool(description = "Define a drive — something that builds up and asks to be addressed")]
    fn set_urge(&self, Parameters(urge): Parameters<Urge>) -> Result<CallToolResult, ErrorData> {
        self.dispatch(UrgeRequests::SetUrge(urge))
    }

    #[tool(description = "Remove a drive category")]
    fn remove_urge(
        &self,
        Parameters(request): Parameters<RemoveUrgeRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(UrgeRequests::RemoveUrge(request))
    }

    #[tool(description = "Look up a drive and understand what it means")]
    fn get_urge(
        &self,
        Parameters(request): Parameters<GetUrgeRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(UrgeRequests::GetUrge(request))
    }

    #[tool(description = "See all the drives that can build pressure")]
    fn list_urges(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(UrgeRequests::ListUrges(ListUrgesRequest))
    }

    // ── Pressure ──────────────────────────────────────────────────

    #[tool(
        description = "Feel what's building — see urgency scores and what's driving them for an agent"
    )]
    fn get_pressure(
        &self,
        Parameters(request): Parameters<GetPressureRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(PressureRequests::GetPressure(request))
    }

    #[tool(description = "Survey the pressure landscape across all agents")]
    fn list_pressures(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(PressureRequests::ListPressures(ListPressuresRequest))
    }

    // ── System: Actor ───────────────────────────────────────────────

    #[tool(description = "Look up who's using this oneiros host")]
    fn get_actor(
        &self,
        Parameters(request): Parameters<GetActorRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(ActorRequests::GetActor(request))
    }

    #[tool(description = "See everyone with access to this host")]
    fn list_actors(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(ActorRequests::ListActors(ListActorsRequest))
    }

    // ── System: Brain ───────────────────────────────────────────────

    #[tool(description = "Create a new brain — a fresh cognitive space for a project")]
    fn create_brain(
        &self,
        Parameters(request): Parameters<CreateBrainRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(BrainRequests::CreateBrain(request))
    }

    #[tool(description = "Look up a brain and its details")]
    fn get_brain(
        &self,
        Parameters(request): Parameters<GetBrainRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(BrainRequests::GetBrain(request))
    }

    #[tool(description = "See all the brains on this host")]
    fn list_brains(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(BrainRequests::ListBrains(ListBrainsRequest))
    }

    // ── System: Tenant ──────────────────────────────────────────────

    #[tool(description = "Look up a tenant on this host")]
    fn get_tenant(
        &self,
        Parameters(request): Parameters<GetTenantRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(TenantRequests::GetTenant(request))
    }

    #[tool(description = "See all tenants on this host")]
    fn list_tenants(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(TenantRequests::ListTenants(ListTenantsRequest))
    }

    // ── System: Ticket ──────────────────────────────────────────────

    #[tool(description = "Verify that an access token is valid")]
    fn validate_ticket(
        &self,
        Parameters(request): Parameters<ValidateTicketRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        self.dispatch(TicketRequests::ValidateTicket(request))
    }

    #[tool(description = "See all active access tokens")]
    fn list_tickets(&self) -> Result<CallToolResult, ErrorData> {
        self.dispatch(TicketRequests::ListTickets(ListTicketsRequest))
    }
}

// ── ServerHandler ───────────────────────────────────────────────────

#[tool_handler]
impl ServerHandler for OneirosToolBox {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_resources_subscribe()
                .build(),
        )
        .with_server_info(Implementation::new("oneiros", env!("CARGO_PKG_VERSION")))
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        let resources = self.list_pressure_resources()?;
        Ok(ListResourcesResult {
            resources,
            next_cursor: None,
            meta: None,
        })
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, ErrorData> {
        Ok(ListResourceTemplatesResult {
            resource_templates: vec![ResourceTemplate::new(
                RawResourceTemplate {
                    uri_template: "oneiroi://pressure/{agent}".to_string(),
                    name: "Agent pressure".to_string(),
                    title: None,
                    description: Some("What's building up for an agent — urgency scores and the forces behind them".to_string()),
                    mime_type: Some("application/json".to_string()),
                    icons: None,
                },
                None,
            )],
            next_cursor: None,
            meta: None,
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, ErrorData> {
        self.read_pressure_resource(&request.uri)
    }

    async fn subscribe(
        &self,
        request: SubscribeRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<(), ErrorData> {
        self.subscribe_uri(&request.uri);
        Ok(())
    }

    async fn unsubscribe(
        &self,
        request: UnsubscribeRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<(), ErrorData> {
        self.unsubscribe_uri(&request.uri);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oneiros_db::Database;
    use oneiros_service::ServiceState;
    use pretty_assertions::assert_eq;

    fn test_toolbox(data_dir: &std::path::Path) -> OneirosToolBox {
        let db_path = data_dir.join("system.db");
        let db = Database::create(&db_path).expect("create system db");

        let source = Source::default();
        let state = Arc::new(ServiceState::new(db, data_dir.to_path_buf(), source));
        let brain_path = data_dir.join("test.db");
        let brain_db = Database::create_brain_db(&brain_path).expect("create brain db");

        let service = OneirosService::brain(state, brain_db);
        OneirosToolBox::system(service)
    }

    #[test]
    fn tool_router_has_all_tools() {
        let dir = tempfile::tempdir().unwrap();
        let toolbox = test_toolbox(dir.path());

        let router = &toolbox.tool_router;
        let tools = router.list_all();

        // Full catalog: 73 tools across 24 domains
        assert_eq!(tools.len(), 73, "expected 73 tools, got {}", tools.len());

        // Spot-check key tools from each domain
        assert!(router.has_route("list_agents"), "missing list_agents");
        assert!(router.has_route("create_agent"), "missing create_agent");
        assert!(router.has_route("get_cognition"), "missing get_cognition");
        assert!(
            router.has_route("create_connection"),
            "missing create_connection"
        );
        assert!(router.has_route("dream"), "missing dream");
        assert!(router.has_route("import_events"), "missing import_events");
        assert!(
            router.has_route("create_experience"),
            "missing create_experience"
        );
        assert!(router.has_route("introspect"), "missing introspect");
        assert!(router.has_route("set_level"), "missing set_level");
        assert!(router.has_route("wake"), "missing wake");
        assert!(router.has_route("emerge"), "missing emerge");
        assert!(router.has_route("add_memory"), "missing add_memory");
        assert!(router.has_route("set_nature"), "missing set_nature");
        assert!(router.has_route("set_persona"), "missing set_persona");
        assert!(router.has_route("reflect"), "missing reflect");
        assert!(router.has_route("search"), "missing search");
        assert!(router.has_route("set_sensation"), "missing set_sensation");
        assert!(router.has_route("sense"), "missing sense");
        assert!(router.has_route("remove_storage"), "missing remove_storage");
        assert!(router.has_route("set_texture"), "missing set_texture");
        assert!(router.has_route("set_urge"), "missing set_urge");
        assert!(router.has_route("get_actor"), "missing get_actor");
        assert!(router.has_route("create_brain"), "missing create_brain");
        assert!(router.has_route("get_tenant"), "missing get_tenant");
        assert!(
            router.has_route("validate_ticket"),
            "missing validate_ticket"
        );
    }

    #[test]
    fn system_toolbox_starts_without_brain() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("system.db");
        let db = Database::create(&db_path).expect("create system db");
        let source = Source::default();
        let state = Arc::new(ServiceState::new(db, dir.path().to_path_buf(), source));

        let service = OneirosService::system(state);
        let toolbox = OneirosToolBox::system(service);
        assert!(
            matches!(toolbox.state(), OneirosService::System { .. }),
            "system toolbox should start in System mode"
        );
    }

    #[test]
    fn parse_pressure_uri_extracts_agent() {
        assert_eq!(
            OneirosToolBox::parse_pressure_uri("oneiroi://pressure/governor.process"),
            Some(AgentName::new("governor.process"))
        );
    }

    #[test]
    fn parse_pressure_uri_rejects_invalid() {
        assert_eq!(
            OneirosToolBox::parse_pressure_uri("https://example.com"),
            None
        );
        assert_eq!(
            OneirosToolBox::parse_pressure_uri("oneiroi://pressure/"),
            None
        );
        assert_eq!(
            OneirosToolBox::parse_pressure_uri("oneiroi://other/foo"),
            None
        );
    }

    #[test]
    fn list_pressure_resources_returns_per_agent() {
        let dir = tempfile::tempdir().unwrap();
        let toolbox = test_toolbox(dir.path());

        // Seed persona + agent
        toolbox
            .dispatch(PersonaRequests::SetPersona(Persona::init(
                PersonaName::new("test-persona"),
                "Test persona",
                "Test",
            )))
            .unwrap();
        toolbox
            .dispatch(AgentRequests::CreateAgent(CreateAgentRequest {
                name: AgentName::new("test-agent"),
                persona: PersonaName::new("test-persona"),
                description: Description::new("Test"),
                prompt: oneiros_model::Prompt::new("Test"),
            }))
            .unwrap();

        let resources = toolbox.list_pressure_resources().unwrap();
        assert_eq!(resources.len(), 1);
        assert_eq!(resources[0].uri, "oneiroi://pressure/test-agent");
        assert_eq!(resources[0].mime_type, Some("application/json".to_string()));
    }

    #[test]
    fn read_pressure_resource_returns_json() {
        let dir = tempfile::tempdir().unwrap();
        let toolbox = test_toolbox(dir.path());

        // Seed agent + persona (needed for pressure queries)
        toolbox
            .dispatch(PersonaRequests::SetPersona(Persona::init(
                PersonaName::new("test-persona"),
                "Test persona",
                "Test",
            )))
            .unwrap();
        toolbox
            .dispatch(AgentRequests::CreateAgent(CreateAgentRequest {
                name: AgentName::new("test-agent"),
                persona: PersonaName::new("test-persona"),
                description: Description::new("Test"),
                prompt: oneiros_model::Prompt::new("Test"),
            }))
            .unwrap();

        let result = toolbox
            .read_pressure_resource("oneiroi://pressure/test-agent")
            .unwrap();
        assert_eq!(result.contents.len(), 1);
    }

    #[test]
    fn read_pressure_resource_rejects_invalid_uri() {
        let dir = tempfile::tempdir().unwrap();
        let toolbox = test_toolbox(dir.path());

        let result = toolbox.read_pressure_resource("invalid://uri");
        assert!(result.is_err());
    }

    #[test]
    fn subscribe_and_unsubscribe_track_uris() {
        let dir = tempfile::tempdir().unwrap();
        let toolbox = test_toolbox(dir.path());

        assert!(!toolbox.has_subscriptions());

        toolbox.subscribe_uri("oneiroi://pressure/test-agent");
        assert!(toolbox.has_subscriptions());

        toolbox.unsubscribe_uri("oneiroi://pressure/test-agent");
        assert!(!toolbox.has_subscriptions());
    }

    #[test]
    fn server_capabilities_include_resources() {
        let dir = tempfile::tempdir().unwrap();
        let toolbox = test_toolbox(dir.path());

        let info = toolbox.get_info();
        assert!(
            info.capabilities.resources.is_some(),
            "server should advertise resource capability"
        );
        let resources = info.capabilities.resources.unwrap();
        assert_eq!(resources.subscribe, Some(true));
    }
}
