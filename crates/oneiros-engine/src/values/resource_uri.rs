use core::fmt;
use std::str::FromStr;

use kinded::Kinded;

use crate::*;

const SCHEME: &str = "oneiros-mcp://";

/// A parsed MCP resource URI.
///
/// Converts `oneiros-mcp://agent/governor.process/cognitions` into a
/// structured `ResourcePath` that carries typed parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceUri {
    raw: String,
    path: ResourcePath,
}

impl ResourceUri {
    pub fn path(&self) -> &ResourcePath {
        &self.path
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }
}

impl fmt::Display for ResourceUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

/// The structured content of a resource URI after parsing.
#[derive(Debug, Clone, PartialEq, Eq, Kinded)]
#[kinded(kind = ResourcePathKind, display = "kebab-case")]
pub enum ResourcePath {
    Agents,
    Levels,
    Textures,
    Sensations,
    Natures,
    Personas,
    Urges,
    Status,
    Pressure,
    Agent(AgentName),
    AgentCognitions(AgentName),
    AgentMemories(AgentName),
    AgentExperiences(AgentName),
    AgentConnections(AgentName),
    AgentPressure(AgentName),
    Cognition(CognitionId),
    Memory(MemoryId),
    Experience(ExperienceId),
    Connection(ConnectionId),
}

impl ResourcePathKind {
    pub fn uri(&self) -> String {
        format!("{SCHEME}{self}")
    }

    pub fn uri_template(&self) -> String {
        let path = match self {
            kind @ Self::Agents
            | kind @ Self::Levels
            | kind @ Self::Textures
            | kind @ Self::Natures
            | kind @ Self::Personas
            | kind @ Self::Urges
            | kind @ Self::Status
            | kind @ Self::Pressure
            | kind @ Self::Sensations => &kind.to_string(),
            Self::Agent => "agent/{name}",
            Self::AgentCognitions => "agent/{name}/cognitions",
            Self::AgentMemories => "agent/{name}/memories",
            Self::AgentExperiences => "agent/{name}/experiences",
            Self::AgentConnections => "agent/{name}/connections",
            Self::AgentPressure => "agent/{name}/pressure",
            Self::Cognition => "cognition/{id}",
            Self::Memory => "memory/{id}",
            Self::Experience => "experience/{id}",
            Self::Connection => "connection/{id}",
        };
        format!("{SCHEME}{path}")
    }

    pub fn resource_def(&self, description: impl Into<Description>) -> ResourceDef {
        ResourceDef::new(self.uri(), self.to_string(), description)
    }

    pub fn into_template(&self, description: impl Into<Description>) -> ResourceTemplateDef {
        ResourceTemplateDef::new(self.uri_template(), self.to_string(), description)
    }
}

impl ResourcePath {
    pub fn uri(&self) -> String {
        let path = match self {
            this @ Self::Agents
            | this @ Self::Levels
            | this @ Self::Textures
            | this @ Self::Natures
            | this @ Self::Personas
            | this @ Self::Urges
            | this @ Self::Status
            | this @ Self::Pressure
            | this @ Self::Sensations => this.kind().to_string(),
            ResourcePath::Agent(name) => format!("agent/{name}"),
            ResourcePath::AgentCognitions(name) => format!("agent/{name}/cognitions"),
            ResourcePath::AgentMemories(name) => format!("agent/{name}/memories"),
            ResourcePath::AgentExperiences(name) => format!("agent/{name}/experiences"),
            ResourcePath::AgentConnections(name) => format!("agent/{name}/connections"),
            ResourcePath::AgentPressure(name) => format!("agent/{name}/pressure"),
            ResourcePath::Cognition(id) => format!("cognition/{id}"),
            ResourcePath::Memory(id) => format!("memory/{id}"),
            ResourcePath::Experience(id) => format!("experience/{id}"),
            ResourcePath::Connection(id) => format!("connection/{id}"),
        };
        format!("{SCHEME}{path}")
    }
}

/// A domain request derived from a resource path.
///
/// `ResourcePath::as_request()` produces this for all paths that map
/// cleanly to a domain read operation. Paths that need I/O to construct
/// a request (like `AgentConnections`, which needs an agent lookup to
/// resolve a `RefToken`) return `None`.
pub enum ResourceRequest {
    Agent(AgentRequest),
    Cognition(CognitionRequest),
    Memory(MemoryRequest),
    Experience(ExperienceRequest),
    Connection(ConnectionRequest),
    Level(LevelRequest),
    Texture(TextureRequest),
    Sensation(SensationRequest),
    Nature(NatureRequest),
    Persona(PersonaRequest),
    Urge(UrgeRequest),
    Pressure(PressureRequest),
    Continuity(ContinuityRequest),
}

impl ResourcePath {
    pub fn as_request(&self) -> Option<ResourceRequest> {
        match self {
            ResourcePath::Agents => Some(ResourceRequest::Agent(AgentRequest::ListAgents(
                ListAgents::default(),
            ))),
            ResourcePath::Agent(name) => {
                Some(ResourceRequest::Agent(AgentRequest::GetAgent(GetAgent {
                    key: ResourceKey::Key(name.clone()),
                })))
            }
            ResourcePath::AgentCognitions(name) => Some(ResourceRequest::Cognition(
                CognitionRequest::ListCognitions(ListCognitions {
                    agent: Some(name.clone()),
                    texture: None,
                    filters: SearchFilters::default(),
                }),
            )),
            ResourcePath::AgentMemories(name) => Some(ResourceRequest::Memory(
                MemoryRequest::ListMemories(ListMemories {
                    agent: Some(name.clone()),
                    filters: SearchFilters::default(),
                }),
            )),
            ResourcePath::AgentExperiences(name) => Some(ResourceRequest::Experience(
                ExperienceRequest::ListExperiences(ListExperiences {
                    agent: Some(name.clone()),
                    filters: SearchFilters::default(),
                }),
            )),
            ResourcePath::AgentConnections(_) => None,
            ResourcePath::AgentPressure(name) => Some(ResourceRequest::Pressure(
                PressureRequest::GetPressure(GetPressure {
                    agent: name.clone(),
                }),
            )),
            ResourcePath::Cognition(id) => Some(ResourceRequest::Cognition(
                CognitionRequest::GetCognition(GetCognition {
                    key: ResourceKey::Key(*id),
                }),
            )),
            ResourcePath::Memory(id) => Some(ResourceRequest::Memory(MemoryRequest::GetMemory(
                GetMemory {
                    key: ResourceKey::Key(*id),
                },
            ))),
            ResourcePath::Experience(id) => Some(ResourceRequest::Experience(
                ExperienceRequest::GetExperience(GetExperience {
                    key: ResourceKey::Key(*id),
                }),
            )),
            ResourcePath::Connection(id) => Some(ResourceRequest::Connection(
                ConnectionRequest::GetConnection(GetConnection {
                    key: ResourceKey::Key(*id),
                }),
            )),
            ResourcePath::Levels => Some(ResourceRequest::Level(LevelRequest::ListLevels(
                ListLevels::builder().build(),
            ))),
            ResourcePath::Textures => Some(ResourceRequest::Texture(TextureRequest::ListTextures(
                ListTextures::builder().build(),
            ))),
            ResourcePath::Sensations => Some(ResourceRequest::Sensation(
                SensationRequest::ListSensations(ListSensations::builder().build()),
            )),
            ResourcePath::Natures => Some(ResourceRequest::Nature(NatureRequest::ListNatures(
                ListNatures::builder().build(),
            ))),
            ResourcePath::Personas => Some(ResourceRequest::Persona(PersonaRequest::ListPersonas(
                ListPersonas::builder().build(),
            ))),
            ResourcePath::Urges => Some(ResourceRequest::Urge(UrgeRequest::ListUrges(
                ListUrges::builder().build(),
            ))),
            ResourcePath::Pressure => {
                Some(ResourceRequest::Pressure(PressureRequest::ListPressures))
            }
            ResourcePath::Status => Some(ResourceRequest::Continuity(
                ContinuityRequest::StatusAgent(StatusAgent {
                    filters: SearchFilters::default(),
                }),
            )),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ResourceUriError {
    #[error("Unknown URI scheme: {0}")]
    UnknownScheme(String),

    #[error("Unknown resource path: {0}")]
    UnknownPath(String),

    #[error("Invalid ID in path: {0}")]
    InvalidId(String),
}

impl FromStr for ResourceUri {
    type Err = ResourceUriError;

    fn from_str(uri: &str) -> Result<Self, Self::Err> {
        let Some(remainder) = uri.strip_prefix(SCHEME) else {
            return Err(ResourceUriError::UnknownScheme(uri.to_string()));
        };

        let segments: Vec<&str> = remainder.split('/').collect();

        let path = match segments.as_slice() {
            ["agents"] => ResourcePath::Agents,
            ["levels"] => ResourcePath::Levels,
            ["textures"] => ResourcePath::Textures,
            ["sensations"] => ResourcePath::Sensations,
            ["natures"] => ResourcePath::Natures,
            ["personas"] => ResourcePath::Personas,
            ["urges"] => ResourcePath::Urges,
            ["status"] => ResourcePath::Status,
            ["pressure"] => ResourcePath::Pressure,
            ["agent", name] => ResourcePath::Agent(AgentName::new(*name)),
            ["agent", name, "cognitions"] => ResourcePath::AgentCognitions(AgentName::new(*name)),
            ["agent", name, "memories"] => ResourcePath::AgentMemories(AgentName::new(*name)),
            ["agent", name, "experiences"] => ResourcePath::AgentExperiences(AgentName::new(*name)),
            ["agent", name, "connections"] => ResourcePath::AgentConnections(AgentName::new(*name)),
            ["agent", name, "pressure"] => ResourcePath::AgentPressure(AgentName::new(*name)),
            ["cognition", id] => ResourcePath::Cognition(
                id.parse()
                    .map_err(|_| ResourceUriError::InvalidId(id.to_string()))?,
            ),
            ["memory", id] => ResourcePath::Memory(
                id.parse()
                    .map_err(|_| ResourceUriError::InvalidId(id.to_string()))?,
            ),
            ["experience", id] => ResourcePath::Experience(
                id.parse()
                    .map_err(|_| ResourceUriError::InvalidId(id.to_string()))?,
            ),
            ["connection", id] => ResourcePath::Connection(
                id.parse()
                    .map_err(|_| ResourceUriError::InvalidId(id.to_string()))?,
            ),
            _ => return Err(ResourceUriError::UnknownPath(uri.to_string())),
        };

        Ok(ResourceUri {
            raw: uri.to_string(),
            path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_concrete_resources() {
        let uri: ResourceUri = "oneiros-mcp://agents".parse().unwrap();
        assert_eq!(*uri.path(), ResourcePath::Agents);

        let uri: ResourceUri = "oneiros-mcp://levels".parse().unwrap();
        assert_eq!(*uri.path(), ResourcePath::Levels);

        let uri: ResourceUri = "oneiros-mcp://pressure".parse().unwrap();
        assert_eq!(*uri.path(), ResourcePath::Pressure);
    }

    #[test]
    fn parses_agent_scoped_resources() {
        let uri: ResourceUri = "oneiros-mcp://agent/governor.process".parse().unwrap();
        assert_eq!(
            *uri.path(),
            ResourcePath::Agent(AgentName::new("governor.process"))
        );

        let uri: ResourceUri = "oneiros-mcp://agent/governor.process/cognitions"
            .parse()
            .unwrap();
        assert_eq!(
            *uri.path(),
            ResourcePath::AgentCognitions(AgentName::new("governor.process"))
        );
    }

    #[test]
    fn parses_entity_resources() {
        let id = CognitionId::new();
        let raw = format!("oneiros-mcp://cognition/{id}");
        let uri: ResourceUri = raw.parse().unwrap();
        assert_eq!(*uri.path(), ResourcePath::Cognition(id));
    }

    #[test]
    fn rejects_unknown_scheme() {
        let result: Result<ResourceUri, _> = "https://example.com".parse();
        assert!(result.is_err());
    }

    #[test]
    fn rejects_unknown_path() {
        let result: Result<ResourceUri, _> = "oneiros-mcp://nonexistent".parse();
        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_id() {
        let result: Result<ResourceUri, _> = "oneiros-mcp://cognition/not-a-uuid".parse();
        assert!(result.is_err());
    }

    #[test]
    fn preserves_raw_uri() {
        let raw = "oneiros-mcp://agent/governor.process/memories";
        let uri: ResourceUri = raw.parse().unwrap();
        assert_eq!(uri.raw(), raw);
        assert_eq!(uri.to_string(), raw);
    }

    #[test]
    fn kind_constructs_concrete_uri() {
        assert_eq!(ResourcePathKind::Agents.uri(), "oneiros-mcp://agents");
        assert_eq!(ResourcePathKind::Levels.uri(), "oneiros-mcp://levels");
    }

    #[test]
    fn kind_constructs_template_uri() {
        assert_eq!(
            ResourcePathKind::Agent.uri_template(),
            "oneiros-mcp://agent/{name}"
        );
        assert_eq!(
            ResourcePathKind::AgentCognitions.uri_template(),
            "oneiros-mcp://agent/{name}/cognitions"
        );
        assert_eq!(
            ResourcePathKind::Cognition.uri_template(),
            "oneiros-mcp://cognition/{id}"
        );
    }

    #[test]
    fn path_constructs_concrete_uri() {
        assert_eq!(ResourcePath::Agents.uri(), "oneiros-mcp://agents");
    }

    #[test]
    fn path_constructs_parameterized_uri() {
        let uri = ResourcePath::Agent(AgentName::new("governor.process")).uri();
        assert_eq!(uri, "oneiros-mcp://agent/governor.process");

        let id = CognitionId::new();
        let uri = ResourcePath::Cognition(id).uri();
        assert_eq!(uri, format!("oneiros-mcp://cognition/{id}"));
    }

    #[test]
    fn round_trip_concrete() {
        let uri = ResourcePath::Agents.uri();
        let parsed: ResourceUri = uri.parse().unwrap();
        assert_eq!(*parsed.path(), ResourcePath::Agents);
    }

    #[test]
    fn round_trip_parameterized() {
        let name = AgentName::new("test.agent");
        let uri = ResourcePath::AgentCognitions(name.clone()).uri();
        let parsed: ResourceUri = uri.parse().unwrap();
        assert_eq!(*parsed.path(), ResourcePath::AgentCognitions(name));
    }
}
