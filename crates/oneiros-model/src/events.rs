use crate::{
    Actor, Agent, AgentName, Brain, Level, LevelName, Persona, PersonaName, Tenant, Texture,
    TextureName, Ticket,
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
#[serde(untagged)]
pub enum Events {
    Actor(ActorEvents),
    Agent(AgentEvents),
    Brain(BrainEvents),
    Level(LevelEvents),
    Persona(PersonaEvents),
    Tenant(TenantEvents),
    Texture(TextureEvents),
    Ticket(TicketEvents),
}
