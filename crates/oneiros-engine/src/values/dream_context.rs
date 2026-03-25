use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Builder, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DreamContext {
    pub agent: Agent,
    #[serde(default)]
    pub persona: Option<Persona>,
    #[builder(default)]
    #[serde(default)]
    pub memories: Vec<Memory>,
    #[builder(default)]
    #[serde(default)]
    pub cognitions: Vec<Cognition>,
    #[builder(default)]
    #[serde(default)]
    pub experiences: Vec<Experience>,
    #[builder(default)]
    #[serde(default)]
    pub connections: Vec<Connection>,
    #[builder(default)]
    #[serde(default)]
    pub textures: Vec<Texture>,
    #[builder(default)]
    #[serde(default)]
    pub levels: Vec<Level>,
    #[builder(default)]
    #[serde(default)]
    pub sensations: Vec<Sensation>,
    #[builder(default)]
    #[serde(default)]
    pub natures: Vec<Nature>,
    #[builder(default)]
    #[serde(default)]
    pub urges: Vec<Urge>,
    #[builder(default)]
    #[serde(default)]
    pub pressures: Vec<PressureReading>,
}
