use std::path::PathBuf;

use crate::{Id, Label};

#[derive(serde::Serialize)]
pub struct Tenant {
    pub tenant_id: Id,
    pub name: Label,
}

#[derive(serde::Serialize)]
pub struct Actor {
    pub tenant_id: Id,
    pub actor_id: Id,
    pub name: Label,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BrainStatus {
    Active,
}

#[derive(serde::Serialize)]
pub struct Brain {
    pub brain_id: Id,
    pub tenant_id: Id,
    pub name: Label,
    pub path: PathBuf,
}
