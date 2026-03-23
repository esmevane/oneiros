use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DreamContext {
    pub agent: Agent,
    #[serde(default)]
    pub persona: Option<Persona>,
    #[serde(default)]
    pub memories: Vec<Memory>,
    #[serde(default)]
    pub cognitions: Vec<Cognition>,
    #[serde(default)]
    pub experiences: Vec<Experience>,
    #[serde(default)]
    pub connections: Vec<Connection>,
    #[serde(default)]
    pub textures: Vec<Texture>,
    #[serde(default)]
    pub levels: Vec<Level>,
    #[serde(default)]
    pub sensations: Vec<Sensation>,
    #[serde(default)]
    pub natures: Vec<Nature>,
    #[serde(default)]
    pub urges: Vec<Urge>,
    #[serde(default)]
    pub pressures: Vec<PressureReading>,
}
