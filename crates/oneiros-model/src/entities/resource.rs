use crate::*;

pub enum SystemResource {
    Actor(Actor),
    Brain(Brain),
    Tenant(Tenant),
    Ticket(Ticket),
}

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

pub enum Resource {
    ProjectResource(ProjectResource),
    SystemResource(SystemResource),
}
