use serde::{Deserialize, Serialize};

use crate::{Agent, Cognition, Experience, Level, Memory, Persona, Sensation, Texture};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DreamContext {
    pub agent: Agent,
    pub persona: Persona,
    pub memories: Vec<Memory>,
    pub cognitions: Vec<Cognition>,
    pub experiences: Vec<Experience>,
    pub textures: Vec<Texture>,
    pub levels: Vec<Level>,
    pub sensations: Vec<Sensation>,
}
