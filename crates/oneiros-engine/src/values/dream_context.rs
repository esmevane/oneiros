use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Builder, Serialize, Deserialize, schemars::JsonSchema)]
pub(crate) struct DreamContext {
    pub(crate) agent: Agent,
    #[serde(default)]
    pub(crate) persona: Option<Persona>,
    #[builder(default)]
    #[serde(default)]
    pub(crate) memories: Vec<Memory>,
    #[builder(default)]
    #[serde(default)]
    pub(crate) cognitions: Vec<Cognition>,
    #[builder(default)]
    #[serde(default)]
    pub(crate) experiences: Vec<Experience>,
    #[builder(default)]
    #[serde(default)]
    pub(crate) connections: Vec<Connection>,
    #[builder(default)]
    #[serde(default)]
    pub(crate) textures: Vec<Texture>,
    #[builder(default)]
    #[serde(default)]
    pub(crate) levels: Vec<Level>,
    #[builder(default)]
    #[serde(default)]
    pub(crate) sensations: Vec<Sensation>,
    #[builder(default)]
    #[serde(default)]
    pub(crate) natures: Vec<Nature>,
    #[builder(default)]
    #[serde(default)]
    pub(crate) urges: Vec<Urge>,
    #[builder(default)]
    #[serde(default)]
    pub(crate) pressures: Vec<PressureReading>,
}
