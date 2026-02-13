use serde::{Deserialize, Serialize};

use crate::{Agent, Cognition, Level, Memory, Persona, Texture};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DreamContext {
    pub agent: Agent,
    pub persona: Persona,
    pub memories: Vec<Memory>,
    pub cognitions: Vec<Cognition>,
    pub textures: Vec<Texture>,
    pub levels: Vec<Level>,
}
