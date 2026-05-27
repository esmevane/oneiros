//! Protocol contract tests — the protocol is what we say it is.
//!
//! The canonical domain lists declare the protocol surface. These tests
//! verify that declaration holds: the API exposes the right domains,
//! MCP offers the right tools, serialization round-trips cleanly through
//! the super-enums, and replay preserves structural identity.

use std::collections::BTreeSet;

use crate::tests::harness::TestApp;
use crate::*;

// ── Canonical domain lists ─────────────────────────────────────

/// All domains — event-producing + facility. Used by the domain count check.
const ALL_DOMAINS: &[&str] = &[
    "actor",
    "agent",
    "bookmark",
    "cognition",
    "connection",
    "continuity",
    "doctor",
    "experience",
    "follow",
    "host",
    "level",
    "mcp",
    "memory",
    "nature",
    "peer",
    "persona",
    "pressure",
    "project",
    "search",
    "seed",
    "sensation",
    "setup",
    "storage",
    "tenant",
    "texture",
    "ticket",
    "trail",
    "urge",
];

/// Domains that produce events — they appear in Events, Requests, and Responses.
const EVENT_DOMAINS: &[&str] = &[
    "actor",
    "agent",
    "bookmark",
    "cognition",
    "connection",
    "continuity",
    "experience",
    "follow",
    "level",
    "memory",
    "nature",
    "peer",
    "persona",
    "project",
    "sensation",
    "storage",
    "tenant",
    "texture",
    "ticket",
    "trail",
    "urge",
];

/// Facility domains — appear in Responses (and sometimes Requests) but not Events.
const FACILITY_DOMAINS: &[&str] = &[
    "doctor", "host", "mcp", "pressure", "search", "seed", "setup",
];

/// Tag names as they appear in the OpenAPI spec.
const TAGGED_DOMAINS: &[&str] = &[
    "actors",
    "agents",
    "bookmarks",
    "cognition",
    "connections",
    "continuity",
    "experiences",
    "follows",
    "host",
    "levels",
    "memory",
    "natures",
    "peers",
    "personas",
    "pressure",
    "projects",
    "search",
    "seed",
    "sensations",
    "storage",
    "tenants",
    "textures",
    "tickets",
    "trail",
    "urges",
];

/// MCP tool names expected from the tool surface.
const MCP_TOOLS: &[&str] = &[
    "add-cognition",
    "add-memory",
    "create-agent",
    "create-connection",
    "create-experience",
    "remove-agent",
    "search-query",
    "update-agent",
];

// ── Canonical list consistency ──────────────────────────────────

#[test]
fn canonical_lists_are_consistent() {
    let events: BTreeSet<&str> = EVENT_DOMAINS.iter().copied().collect();
    let facilities: BTreeSet<&str> = FACILITY_DOMAINS.iter().copied().collect();
    let all: BTreeSet<&str> = ALL_DOMAINS.iter().copied().collect();

    assert_eq!(
        events.len(),
        EVENT_DOMAINS.len(),
        "EVENT_DOMAINS has duplicates"
    );
    assert_eq!(
        facilities.len(),
        FACILITY_DOMAINS.len(),
        "FACILITY_DOMAINS has duplicates"
    );
    assert_eq!(all.len(), ALL_DOMAINS.len(), "ALL_DOMAINS has duplicates");

    let union: BTreeSet<&str> = events.union(&facilities).copied().collect();
    assert_eq!(
        union, all,
        "EVENT_DOMAINS + FACILITY_DOMAINS should equal ALL_DOMAINS"
    );

    let overlap: Vec<&&str> = events.intersection(&facilities).collect();
    assert!(
        overlap.is_empty(),
        "EVENT_DOMAINS and FACILITY_DOMAINS should not overlap: {overlap:?}"
    );
}

// ── API tag coverage ───────────────────────────────────────────

#[tokio::test]
async fn api_tags_cover_all_routed_domains() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?;

    let http = reqwest::Client::new();
    let token = app.token().expect("project should have a token");
    let url = format!("{}/api.json", app.base_url());

    let resp = http
        .get(&url)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await?;

    assert!(resp.status().is_success(), "GET /api.json should succeed");

    let spec: serde_json::Value = resp.json().await?;
    let tags = spec["tags"]
        .as_array()
        .expect("/api.json should have a tags array");

    let tag_names: BTreeSet<&str> = tags.iter().filter_map(|t| t["name"].as_str()).collect();
    let expected: BTreeSet<&str> = TAGGED_DOMAINS.iter().copied().collect();

    let missing: Vec<&&str> = expected.difference(&tag_names).collect();
    let extra: Vec<&&str> = tag_names.difference(&expected).collect();

    assert!(
        missing.is_empty(),
        "Domains missing from OpenAPI tags: {missing:?}"
    );
    assert!(
        extra.is_empty(),
        "Unexpected OpenAPI tags not in canonical list: {extra:?}"
    );

    Ok(())
}

// ── Skill inventory coverage ───────────────────────────────────

#[tokio::test]
async fn mcp_tools_cover_expected_surface() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let token = app.token().expect("project should have a token");
    let http = reqwest::Client::new();
    let url = app.mcp_url();

    let init_resp = mcp_post(&http, &url)
        .header("Authorization", format!("Bearer {token}"))
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": { "name": "coverage-test", "version": "0.1.0" }
            }
        }))
        .send()
        .await?;

    assert!(init_resp.status().is_success());

    let session_id = init_resp
        .headers()
        .get("mcp-session-id")
        .map(|v| v.to_str().unwrap().to_string());

    let mut notif = mcp_post(&http, &url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    }));
    if let Some(sid) = &session_id {
        notif = notif.header("mcp-session-id", sid);
    }
    notif.send().await?;

    let mut req = mcp_post(&http, &url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    }));
    if let Some(sid) = &session_id {
        req = req.header("mcp-session-id", sid);
    }

    let json = extract_sse_json(&req.send().await?.text().await?);

    let tools = json["result"]["tools"]
        .as_array()
        .expect("tools should be an array");

    let actual: BTreeSet<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();
    let expected: BTreeSet<&str> = MCP_TOOLS.iter().copied().collect();

    let missing: Vec<&&str> = expected.difference(&actual).collect();
    let extra: Vec<&&str> = actual.difference(&expected).collect();

    assert!(
        missing.is_empty(),
        "Expected MCP tools missing: {missing:?}"
    );
    assert!(
        extra.is_empty(),
        "Unexpected MCP tools not in canonical list: {extra:?}"
    );

    Ok(())
}

// ── Super-enum round-trip ──────────────────────────────────────
//
// For each domain, construct a minimal response value, serialize it,
// deserialize through the Responses super-enum, re-serialize, and
// verify the JSON is unchanged. This catches missing collects_enum!
// arms and serde(untagged) ordering issues.

#[tokio::test]
async fn responses_round_trip_through_super_enum() {
    round_trip::<ActorResponse>(
        "actor",
        ActorResponse::Listed(ActorsResponse::V1(ActorsResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<AgentResponse>(
        "agent",
        AgentResponse::Agents(AgentsResponse::V1(AgentsResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<BookmarkResponse>(
        "bookmark",
        BookmarkResponse::Bookmarks(Listed::new(vec![], 0)),
    );
    round_trip::<CognitionResponse>(
        "cognition",
        CognitionResponse::Cognitions(CognitionsResponse::V1(CognitionsResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<ConnectionResponse>(
        "connection",
        ConnectionResponse::Connections(ConnectionsResponse::V1(ConnectionsResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<ContinuityResponse>(
        "continuity",
        ContinuityResponse::Status(StatusResponse::V1(StatusResponseV1 {
            table: AgentActivityTable { agents: vec![] },
        })),
    );
    round_trip::<DoctorResponse>(
        "doctor",
        DoctorResponse::CheckupStatus(CheckupStatusResponse::V1(CheckupStatusResponseV1 {
            checks: vec![],
        })),
    );
    round_trip::<ExperienceResponse>(
        "experience",
        ExperienceResponse::Experiences(ExperiencesResponse::V1(ExperiencesResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<FollowResponse>(
        "follow",
        FollowResponse::Listed(FollowsResponse::V1(FollowsResponseV1 {
            follows: Listed::new(vec![], 0),
        })),
    );
    round_trip::<HostResponse>("host", HostResponse::HostAlreadyInitialized);
    round_trip::<LevelResponse>(
        "level",
        LevelResponse::Levels(LevelsResponse::V1(LevelsResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<McpResponses>(
        "mcp",
        McpResponses::McpConfigExists(McpConfigExistsResponse::V1(McpConfigExistsResponseV1 {
            path: std::path::PathBuf::from("/dev/null"),
        })),
    );
    round_trip::<MemoryResponse>(
        "memory",
        MemoryResponse::Memories(MemoriesResponse::V1(MemoriesResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<NatureResponse>(
        "nature",
        NatureResponse::Natures(NaturesResponse::V1(NaturesResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<PeerResponse>(
        "peer",
        PeerResponse::Listed(PeersResponse::V1(PeersResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<PersonaResponse>(
        "persona",
        PersonaResponse::Personas(PersonasResponse::V1(PersonasResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<PressureResponse>(
        "pressure",
        PressureResponse::AllReadings(AllReadingsResponse::V1(AllReadingsResponseV1 {
            pressures: vec![],
        })),
    );
    round_trip::<ProjectResponse>(
        "project",
        ProjectResponse::Listed(ProjectsResponse::V1(ProjectsResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<SearchResponse>(
        "search",
        SearchResponse::Results(ResultsResponse::V1(SearchResults {
            query: QueryText::new("test"),
            total: 0,
            hits: vec![],
            facets: Facets::default(),
        })),
    );
    round_trip::<SeedResponse>("seed", SeedResponse::SeedComplete);
    round_trip::<SensationResponse>(
        "sensation",
        SensationResponse::Sensations(SensationsResponse::V1(SensationsResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<SetupResponse>(
        "setup",
        SetupResponse::SetupComplete(SetupCompleteResponse::V1(SetupCompleteResponseV1 {
            steps: vec![],
        })),
    );
    round_trip::<StorageResponse>("storage", StorageResponse::NoEntries);
    round_trip::<TenantResponse>(
        "tenant",
        TenantResponse::Listed(TenantsResponse::V1(TenantsResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<TextureResponse>(
        "texture",
        TextureResponse::Textures(TexturesResponse::V1(TexturesResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<TicketResponse>(
        "ticket",
        TicketResponse::Listed(TicketsResponse::V1(TicketsResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
    round_trip::<TrailResponse>("trail", TrailResponse::NoTrail);
    round_trip::<UrgeResponse>(
        "urge",
        UrgeResponse::Urges(UrgesResponse::V1(UrgesResponseV1 {
            items: vec![],
            total: 0,
        })),
    );
}

fn round_trip<R>(domain: &str, value: R)
where
    R: serde::Serialize + serde::de::DeserializeOwned + Into<Responses> + Clone,
{
    let json =
        serde_json::to_value(&value).unwrap_or_else(|e| panic!("{domain}: serialize failed: {e}"));

    let _: Responses = serde_json::from_value(json.clone())
        .unwrap_or_else(|e| panic!("{domain}: deserialize through Responses failed: {e}"));

    let via_from: Responses = value.into();
    let from_json = serde_json::to_value(&via_from)
        .unwrap_or_else(|e| panic!("{domain}: re-serialize via From failed: {e}"));

    assert_eq!(
        json, from_json,
        "{domain}: round-trip through From<> changed shape"
    );
}

// ── Replay determinism ─────────────────────────────────────────

#[tokio::test]
async fn replay_is_structurally_deterministic() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();

    seed_agent(&client).await;
    emit_representative_events(&client).await;

    let before = capture_projection_state(&app).await;

    app.command("project replay")
        .await
        .expect("replay should succeed");

    let after = capture_projection_state(&app).await;

    assert_eq!(
        before.levels, after.levels,
        "levels should be identical after replay"
    );
    assert_eq!(
        before.agents, after.agents,
        "agents should be identical after replay"
    );
    assert_eq!(
        before.cognitions, after.cognitions,
        "cognitions should be identical after replay"
    );
    assert_eq!(
        before.memories, after.memories,
        "memories should be identical after replay"
    );
    assert_eq!(
        before.experiences, after.experiences,
        "experiences should be identical after replay"
    );
    assert_eq!(
        before.connections, after.connections,
        "connections should be identical after replay"
    );

    Ok(())
}

// ── Helpers ────────────────────────────────────────────────────

fn mcp_post(http: &reqwest::Client, url: &str) -> reqwest::RequestBuilder {
    http.post(url)
        .header("Accept", "application/json, text/event-stream")
        .header("Content-Type", "application/json")
}

fn extract_sse_json(body: &str) -> serde_json::Value {
    let json_str = body
        .lines()
        .filter(|line| line.starts_with("data:"))
        .map(|line| line.strip_prefix("data:").unwrap().trim())
        .find(|s| !s.is_empty())
        .expect("response should contain a non-empty data: line");

    serde_json::from_str(json_str).expect("data: line should be valid JSON")
}

async fn seed_agent(client: &crate::tests::harness::TestClient) {
    client
        .persona()
        .set(
            &SetPersona::builder_v1()
                .name("test-persona")
                .description("A test persona")
                .prompt("You are a test.")
                .build()
                .into(),
        )
        .await
        .expect("seed persona");

    client
        .agent()
        .create(&CreateAgent::V1(
            CreateAgentV1::builder()
                .name("smoketest")
                .persona("test-persona")
                .description("Smoke test agent")
                .prompt("You test")
                .build(),
        ))
        .await
        .expect("seed agent");
}

async fn emit_representative_events(client: &crate::tests::harness::TestClient) {
    let agent = AgentName::new("smoketest.test-persona");

    let cognition_id = match client
        .cognition()
        .add(
            &AddCognition::builder_v1()
                .agent(agent.clone())
                .texture("observation")
                .content("A smoke test thought")
                .build()
                .into(),
        )
        .await
        .expect("add cognition")
    {
        CognitionResponse::CognitionAdded(CognitionAddedResponse::V1(added)) => added.cognition.id,
        other => panic!("expected CognitionAdded, got {other:?}"),
    };

    let memory_id = match client
        .memory()
        .add(
            &AddMemory::builder_v1()
                .agent(agent.clone())
                .level("working")
                .content("A smoke test memory")
                .build()
                .into(),
        )
        .await
        .expect("add memory")
    {
        MemoryResponse::MemoryAdded(MemoryAddedResponse::V1(added)) => added.memory.id,
        other => panic!("expected MemoryAdded, got {other:?}"),
    };

    client
        .experience()
        .create(
            &CreateExperience::builder_v1()
                .agent(agent.clone())
                .sensation("echoes")
                .description("A smoke test experience")
                .build()
                .into(),
        )
        .await
        .expect("create experience");

    client
        .connection()
        .create(
            &CreateConnection::builder_v1()
                .from_ref(RefToken::new(Ref::cognition(cognition_id)))
                .to_ref(RefToken::new(Ref::memory(memory_id)))
                .nature("context")
                .build()
                .into(),
        )
        .await
        .expect("create connection");
}

#[derive(Debug)]
struct ProjectionSnapshot {
    levels: String,
    agents: String,
    cognitions: String,
    memories: String,
    experiences: String,
    connections: String,
}

async fn capture_projection_state(app: &TestApp) -> ProjectionSnapshot {
    let client = app.client();
    let agent = AgentName::new("smoketest.test-persona");

    let levels = client
        .level()
        .list(&ListLevels::builder_v1().build().into())
        .await
        .expect("list levels");

    let agents = client
        .agent()
        .list(&ListAgents::builder_v1().build().into())
        .await
        .expect("list agents");

    let cognitions = client
        .cognition()
        .list(
            &ListCognitions::builder_v1()
                .agent(agent.clone())
                .build()
                .into(),
        )
        .await
        .expect("list cognitions");

    let memories = client
        .memory()
        .list(
            &ListMemories::builder_v1()
                .agent(agent.clone())
                .build()
                .into(),
        )
        .await
        .expect("list memories");

    let experiences = client
        .experience()
        .list(
            &ListExperiences::builder_v1()
                .agent(agent.clone())
                .build()
                .into(),
        )
        .await
        .expect("list experiences");

    let connections = client
        .connection()
        .list(&ListConnections::builder_v1().build().into())
        .await
        .expect("list connections");

    ProjectionSnapshot {
        levels: serde_json::to_string(&levels).unwrap(),
        agents: serde_json::to_string(&agents).unwrap(),
        cognitions: serde_json::to_string(&cognitions).unwrap(),
        memories: serde_json::to_string(&memories).unwrap(),
        experiences: serde_json::to_string(&experiences).unwrap(),
        connections: serde_json::to_string(&connections).unwrap(),
    }
}
