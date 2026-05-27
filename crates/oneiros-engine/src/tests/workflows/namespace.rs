// Compile-time guard: values::* and domains::* share the crate-root
// namespace via glob re-exports. If a new type in either module collides
// with a name in the other, this file will fail to compile with an
// "ambiguous import" error, surfacing the collision immediately.
//
// To keep this effective, reference at least one type per value module
// and one response/service type per domain. Add new entries when adding
// new modules.

use crate::*;

// ── Values ─────────────────────────────────────────────────────────
#[expect(clippy::too_many_arguments)]
fn _values(
    _chronicle: Chronicle,
    _content: Content,
    _description: Description,
    _dream_config: DreamConfig,
    _entity_index: EntityIndex<(), ()>,
    _gauge: Gauge,
    _host_canon: HostCanon,
    _label: Label,
    _limit: Limit,
    _offset: Offset,
    _output_mode: OutputMode,
    _palette: Palette,
    _project_canon: ProjectCanon,
    _ref_token: RefToken,
    _rendered: Rendered<()>,
    _resource_key: ResourceKey<()>,
    _search_filters: SearchFilters,
    _source: Source,
    _timestamp: Timestamp,
    _tool_name: ToolName,
) {
}

// ── Domains ────────────────────────────────────────────────────────
#[expect(clippy::too_many_arguments)]
fn _domains(
    _agent: Agent,
    _agent_response: AgentResponse,
    _bookmark: Bookmark,
    _bookmark_response: BookmarkResponse,
    _cognition: Cognition,
    _cognition_response: CognitionResponse,
    _connection: Connection,
    _connection_response: ConnectionResponse,
    _experience: Experience,
    _experience_response: ExperienceResponse,
    _follow: Follow,
    _follow_response: FollowResponse,
    _level: Level,
    _level_response: LevelResponse,
    _memory: Memory,
    _memory_response: MemoryResponse,
    _nature: Nature,
    _nature_response: NatureResponse,
    _peer: Peer,
    _peer_response: PeerResponse,
    _persona: Persona,
    _persona_response: PersonaResponse,
    _pressure: Pressure,
    _pressure_response: PressureResponse,
    _search_response: SearchResponse,
    _sensation: Sensation,
    _sensation_response: SensationResponse,
    _storage_response: StorageResponse,
    _tenant: Tenant,
    _tenant_response: TenantResponse,
    _texture: Texture,
    _texture_response: TextureResponse,
    _trail_response: TrailResponse,
    _urge: Urge,
    _urge_response: UrgeResponse,
) {
}
