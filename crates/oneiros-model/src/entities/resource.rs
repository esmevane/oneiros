use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SystemResource {
    Actor(Actor),
    Brain(Brain),
    Tenant(Tenant),
    Ticket(Ticket),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ProjectResource {
    Agent(Agent),
    Cognition(Cognition),
    Connection(Connection),
    Experience(Experience),
    Level(Level),
    Memory(Memory),
    Nature(Nature),
    Persona(Persona),
    Sensation(Sensation),
    StorageEntry(StorageEntry),
    Texture(Texture),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Resource {
    ProjectResource(ProjectResource),
    SystemResource(SystemResource),
}
