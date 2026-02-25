use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TenantEvents {
    TenantCreated(Identity<TenantId, Tenant>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ActorEvents {
    ActorCreated(Identity<ActorId, Actor>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum BrainEvents {
    BrainCreated(Identity<BrainId, HasPath<Brain>>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum AgentEvents {
    AgentCreated(AgentRecord),
    AgentUpdated(AgentRecord),
    AgentRemoved { name: AgentName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum CognitionEvents {
    CognitionAdded(Record<CognitionId, Cognition>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum MemoryEvents {
    MemoryAdded(Record<MemoryId, Memory>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageEvents {
    StorageSet(StorageEntryRecord),
    StorageRemoved { key: StorageKey },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum PersonaEvents {
    PersonaSet(PersonaRecord),
    PersonaRemoved { name: PersonaName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TextureEvents {
    TextureSet(TextureRecord),
    TextureRemoved { name: TextureName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LevelEvents {
    LevelSet(LevelRecord),
    LevelRemoved { name: LevelName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TicketEvents {
    TicketIssued(Identity<TicketId, Ticket>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SensationEvents {
    SensationSet(SensationRecord),
    SensationRemoved { name: SensationName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum NatureEvents {
    NatureSet(NatureRecord),
    NatureRemoved { name: NatureName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ConnectionEvents {
    ConnectionCreated(Record<ConnectionId, Connection>),
    ConnectionRemoved { id: ConnectionId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceEvents {
    ExperienceCreated(ExperienceRecord),
    ExperienceRefAdded {
        experience_id: ExperienceId,
        record_ref: RecordRef,
    },
    ExperienceDescriptionUpdated {
        experience_id: ExperienceId,
        description: Description,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LifecycleEvents {
    Woke { name: AgentName },
    Slept { name: AgentName },
    Emerged { name: AgentName },
    Receded { name: AgentName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum DreamingEvents {
    DreamBegun { agent: AgentName },
    DreamComplete { agent: Agent },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum IntrospectingEvents {
    IntrospectionBegun { agent: AgentName },
    IntrospectionComplete { agent: AgentName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ReflectingEvents {
    ReflectionBegun { agent: AgentName },
    ReflectionComplete { agent: AgentName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SenseEvents {
    Sensed { agent: AgentName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Events {
    Actor(ActorEvents),
    Agent(AgentEvents),
    Brain(BrainEvents),
    Cognition(CognitionEvents),
    Connection(ConnectionEvents),
    Nature(NatureEvents),
    Dreaming(DreamingEvents),
    Experience(ExperienceEvents),
    Introspecting(IntrospectingEvents),
    Level(LevelEvents),
    Lifecycle(LifecycleEvents),
    Memory(MemoryEvents),
    Persona(PersonaEvents),
    Reflecting(ReflectingEvents),
    Sensation(SensationEvents),
    Sense(SenseEvents),
    Storage(StorageEvents),
    Tenant(TenantEvents),
    Texture(TextureEvents),
    Ticket(TicketEvents),
}
