use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = ActorEventsType, display = "kebab-case")]
pub enum ActorEvents {
    ActorCreated(ActorCreated),
}

impl ActorEvents {
    pub fn maybe_actor(&self) -> Option<Actor> {
        match self {
            ActorEvents::ActorCreated(event) => event.clone().current().ok().map(|v| v.actor),
        }
    }
}

versioned! {
    pub enum ActorCreated {
        V1 => {
            #[serde(flatten)] pub actor: Actor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        assert_eq!(&ActorEventsType::ActorCreated.to_string(), "actor-created");
    }

    #[test]
    fn actor_created_wire_format_is_flat() {
        // The embedded `actor: Actor` field is `#[serde(flatten)]`, so the
        // wire shape stays at the model fields rather than gaining an
        // `actor` envelope.
        let actor = Actor::builder()
            .tenant_id(TenantId::new())
            .name(ActorName::new("alice"))
            .build();

        let event = ActorEvents::ActorCreated(ActorCreated::V1(ActorCreatedV1 {
            actor: actor.clone(),
        }));
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "actor-created");
        assert!(
            json["data"].get("actor").is_none(),
            "flatten must elide the actor envelope on the wire"
        );
        assert_eq!(json["data"]["id"], actor.id.to_string());
        assert_eq!(json["data"]["name"], "alice");
        assert!(json["data"].get("tenant_id").is_some());
        assert!(json["data"].get("created_at").is_some());
    }

    #[test]
    fn actor_created_round_trips() {
        let actor = Actor::builder()
            .tenant_id(TenantId::new())
            .name(ActorName::new("alice"))
            .build();

        let original = ActorEvents::ActorCreated(ActorCreated::V1(ActorCreatedV1 {
            actor: actor.clone(),
        }));
        let json = serde_json::to_string(&original).unwrap();
        let decoded: ActorEvents = serde_json::from_str(&json).unwrap();

        match decoded {
            ActorEvents::ActorCreated(creation) => {
                let v1 = creation.current().unwrap();
                assert_eq!(v1.actor.name, actor.name);
                assert_eq!(v1.actor.id, actor.id);
            }
        }
    }
}
