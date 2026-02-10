use crate::{Actor, Brain, Tenant};

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
#[serde(untagged)]
pub enum Events {
    Actor(ActorEvents),
    Brain(BrainEvents),
    Tenant(TenantEvents),
}
