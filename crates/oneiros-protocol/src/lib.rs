mod requests;
mod responses;
mod sensing;

pub use events::*;
pub use requests::*;
pub use responses::*;
pub use sensing::*;

mod events {
    use oneiros_model::{
        Actor, Agent, AgentId, AgentName, Brain, Cognition, Content, DreamContext, Experience,
        ExperienceId, Identity, Level, LevelName, Memory, Persona, PersonaName, RecordRef,
        Sensation, SensationName, StorageEntry, StorageKey, Tenant, Texture, TextureName, Ticket,
    };

    use crate::SenseEvents;

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum TenantEvents {
        TenantCreated(Tenant),
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum ActorEvents {
        ActorCreated(Actor),
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum BrainEvents {
        BrainCreated(Brain),
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
        CognitionAdded(Cognition),
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "kebab-case", tag = "type", content = "data")]
    pub enum MemoryEvents {
        MemoryAdded(Memory),
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
        TicketIssued(Ticket),
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
        ExperienceCreated(Experience),
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
