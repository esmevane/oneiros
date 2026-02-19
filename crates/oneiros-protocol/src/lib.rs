mod requests;
mod responses;
mod sensing;

pub use events::*;
pub use requests::*;
pub use responses::*;
pub use sensing::*;

mod events {
    use oneiros_model::{
        Actor, ActorId, Agent, AgentId, AgentName, Brain, BrainId, Cognition, CognitionId, Content,
        DreamContext, Experience, ExperienceId, Identity, Level, LevelName, Memory, MemoryId,
        Persona, PersonaName, RecordRef, Sensation, SensationName, StorageEntry, StorageKey,
        Tenant, TenantId, Texture, TextureName, Ticket, TicketId,
    };

    use crate::SenseEvents;

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum TenantEvents {
        TenantCreated(Identity<TenantId, Tenant>),
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum ActorEvents {
        ActorCreated(Identity<ActorId, Actor>),
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum BrainEvents {
        BrainCreated(Identity<BrainId, Brain>),
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum AgentEvents {
        AgentCreated(Identity<AgentId, Agent>),
        AgentUpdated(Identity<AgentId, Agent>),
        AgentRemoved { name: AgentName },
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum CognitionEvents {
        CognitionAdded(Identity<CognitionId, Cognition>),
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum MemoryEvents {
        MemoryAdded(Identity<MemoryId, Memory>),
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum StorageEvents {
        StorageSet(StorageEntry),
        StorageRemoved { key: StorageKey },
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum PersonaEvents {
        PersonaSet(Persona),
        PersonaRemoved { name: PersonaName },
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum TextureEvents {
        TextureSet(Texture),
        TextureRemoved { name: TextureName },
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum LevelEvents {
        LevelSet(Level),
        LevelRemoved { name: LevelName },
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum TicketEvents {
        TicketIssued(Identity<TicketId, Ticket>),
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum SensationEvents {
        SensationSet(Sensation),
        SensationRemoved { name: SensationName },
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum ExperienceEvents {
        ExperienceCreated(Identity<ExperienceId, Experience>),
        ExperienceRefAdded {
            experience_id: ExperienceId,
            record_ref: RecordRef,
        },
        ExperienceDescriptionUpdated {
            experience_id: ExperienceId,
            description: Content,
        },
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum LifecycleEvents {
        Woke { name: AgentName },
        Slept { name: AgentName },
        Emerged { name: AgentName },
        Receded { name: AgentName },
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum DreamingEvents {
        DreamBegun { agent: AgentName },
        DreamComplete(Box<DreamContext>),
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum IntrospectingEvents {
        IntrospectionBegun { agent: AgentName },
        IntrospectionComplete { agent: AgentName },
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum ReflectingEvents {
        ReflectionBegun { agent: AgentName },
        ReflectionComplete { agent: AgentName },
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    pub enum Events {
        Actor(ActorEvents),
        Agent(AgentEvents),
        Brain(BrainEvents),
        Cognition(CognitionEvents),
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
}
