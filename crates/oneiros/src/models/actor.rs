use crate::*;

#[derive(serde::Serialize)]
pub(crate) struct Actor {
    pub(crate) tenant_id: Id,
    pub(crate) actor_id: Id,
    pub(crate) name: Label,
}
