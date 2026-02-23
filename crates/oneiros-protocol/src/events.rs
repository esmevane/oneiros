use oneiros_model::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TenantEvents {
    TenantCreated(Identity<TenantId, Tenant>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ActorEvents {
    ActorCreated(Identity<ActorId, Actor>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum BrainEvents {
    BrainCreated(Identity<BrainId, HasPath<Brain>>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum AgentEvents {
    AgentCreated(AgentRecord),
    AgentUpdated(AgentRecord),
    AgentRemoved { name: AgentName },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum CognitionEvents {
    CognitionAdded(Record<CognitionId, Cognition>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum MemoryEvents {
    MemoryAdded(Record<MemoryId, Memory>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageEvents {
    StorageSet(StorageEntryRecord),
    StorageRemoved { key: StorageKey },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum PersonaEvents {
    PersonaSet(PersonaRecord),
    PersonaRemoved { name: PersonaName },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TextureEvents {
    TextureSet(TextureRecord),
    TextureRemoved { name: TextureName },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LevelEvents {
    LevelSet(LevelRecord),
    LevelRemoved { name: LevelName },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TicketEvents {
    TicketIssued(Identity<TicketId, Ticket>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SensationEvents {
    SensationSet(SensationRecord),
    SensationRemoved { name: SensationName },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum NatureEvents {
    NatureSet(NatureRecord),
    NatureRemoved { name: NatureName },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ConnectionEvents {
    ConnectionCreated(Record<ConnectionId, Connection>),
    ConnectionRemoved { id: ConnectionId },
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LifecycleEvents {
    Woke { name: AgentName },
    Slept { name: AgentName },
    Emerged { name: AgentName },
    Receded { name: AgentName },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum DreamingEvents {
    DreamBegun { agent: AgentName },
    DreamComplete(Box<DreamContext>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum IntrospectingEvents {
    IntrospectionBegun { agent: AgentName },
    IntrospectionComplete { agent: AgentName },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ReflectingEvents {
    ReflectionBegun { agent: AgentName },
    ReflectionComplete { agent: AgentName },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SenseEvents {
    Sensed { agent: oneiros_model::AgentName },
}

#[derive(Serialize, Deserialize)]
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
