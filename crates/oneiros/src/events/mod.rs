mod actor_events;
mod tenant_events;

pub(crate) use actor_events::ActorEvents;
pub(crate) use tenant_events::TenantEvents;

#[derive(serde::Serialize)]
#[serde(untagged)]
pub(crate) enum Events {
    Actor(ActorEvents),
    Tenant(TenantEvents),
}
