use crate::{
    Actor, Agent, AgentName, Brain, Cognition, DreamContext, Level, LevelName, Memory, Persona,
    PersonaName, StorageEntry, StorageKey, Tenant, Texture, TextureName, Ticket,
};

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TenantEvents {
    TenantCreated(Tenant),
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ActorEvents {
    ActorCreated(Actor),
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum BrainEvents {
    BrainCreated(Brain),
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum AgentEvents {
    AgentCreated(Agent),
    AgentUpdated(Agent),
    AgentRemoved { name: AgentName },
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum CognitionEvents {
    CognitionAdded(Cognition),
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum MemoryEvents {
    MemoryAdded(Memory),
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum StorageEvents {
    StorageSet(StorageEntry),
    StorageRemoved { key: StorageKey },
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum PersonaEvents {
    PersonaSet(Persona),
    PersonaRemoved { name: PersonaName },
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TextureEvents {
    TextureSet(Texture),
    TextureRemoved { name: TextureName },
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LevelEvents {
    LevelSet(Level),
    LevelRemoved { name: LevelName },
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TicketEvents {
    TicketIssued(Ticket),
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum DreamingEvents {
    DreamBegun { agent: AgentName },
    DreamComplete(Box<DreamContext>),
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum IntrospectingEvents {
    IntrospectionBegun { agent: AgentName },
    IntrospectionComplete { agent: AgentName },
}

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ReflectingEvents {
    ReflectionBegun { agent: AgentName },
    ReflectionComplete { agent: AgentName },
}

#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum Events {
    Actor(ActorEvents),
    Agent(AgentEvents),
    Brain(BrainEvents),
    Cognition(CognitionEvents),
    Dreaming(DreamingEvents),
    Introspecting(IntrospectingEvents),
    Level(LevelEvents),
    Memory(MemoryEvents),
    Persona(PersonaEvents),
    Reflecting(ReflectingEvents),
    Storage(StorageEvents),
    Tenant(TenantEvents),
    Texture(TextureEvents),
    Ticket(TicketEvents),
}
