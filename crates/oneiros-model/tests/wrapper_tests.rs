use oneiros_link::{Addressable, Key, Link, LinkError, LinkNarrowingError};
use oneiros_model::*;
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Widget {
    name: String,
    color: String,
}

impl Addressable for Widget {
    fn address_label() -> &'static str {
        "widget"
    }

    fn link(&self) -> Result<Link, LinkError> {
        Link::new(&(Self::address_label(), &self.name))
    }
}

#[test]
fn record_of_identity_composes() {
    let widget = Widget {
        name: "sprocket".into(),
        color: "red".into(),
    };
    let expected_link = widget.link().unwrap();
    let identity = Identity::new(42u64, widget);
    let record = Record::new(identity).unwrap();

    // Link comes from Widget's Addressable, through Identity delegation.
    assert_eq!(*record.link(), expected_link);

    // Deref chain: Record → Identity. Access id on Identity.
    assert_eq!(record.id, 42);

    // Double deref: Record → Identity → Widget. Access field on Widget.
    assert_eq!(record.name, "sprocket");
    assert_eq!(record.color, "red");
}

#[test]
fn record_of_identity_serializes_flat() {
    let widget = Widget {
        name: "sprocket".into(),
        color: "red".into(),
    };
    let identity = Identity::new(42u64, widget);
    let record = Record::new(identity).unwrap();
    let json = serde_json::to_value(&record).unwrap();

    // All fields at top level.
    assert!(json.get("link").is_some());
    assert!(json.get("id").is_some());
    assert!(json.get("name").is_some());
    assert!(json.get("color").is_some());

    // No nesting artifacts.
    assert!(json.get("inner").is_none());
}

#[test]
fn record_of_identity_roundtrips_through_serde() {
    let widget = Widget {
        name: "sprocket".into(),
        color: "red".into(),
    };
    let identity = Identity::new(42u64, widget);
    let record = Record::new(identity).unwrap();
    let json = serde_json::to_string(&record).unwrap();
    let deserialized: Record<Identity<u64, Widget>> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, record);
}

#[test]
fn address_invariant_across_all_layers() {
    let widget = Widget {
        name: "sprocket".into(),
        color: "red".into(),
    };

    // Bare shape.
    let bare_link = widget.link().unwrap();

    // Identity wrapping.
    let identity = Identity::new(42u64, widget);
    let identity_link = identity.link().unwrap();

    // Record wrapping.
    let record = Record::new(identity).unwrap();
    let record_link = record.link().clone();

    // All three produce the same link.
    assert_eq!(bare_link, identity_link);
    assert_eq!(bare_link, record_link);
}

// --- Key<I, L> with domain types ---

#[test]
fn key_with_domain_id() {
    let id = AgentId::new();
    let key: Key<AgentId, AgentLink> = Key::Id(id.clone());
    assert_eq!(key.try_id(), Some(&id));
    assert_eq!(key.try_link(), None);
}

#[test]
fn key_with_domain_link() {
    let agent = Agent {
        name: AgentName::new("test"),
        persona: PersonaName::new("expert"),
        description: Description::default(),
        prompt: Prompt::default(),
    };
    let link = agent.link().unwrap();
    let typed_link = AgentLink::try_from(link).unwrap();
    let key: Key<AgentId, AgentLink> = Key::Link(typed_link.clone());
    assert_eq!(key.try_link(), Some(&typed_link));
    assert_eq!(key.try_id(), None);
}

#[test]
fn key_broadens_from_typed_to_erased() {
    let id = AgentId::new();
    let agent = Agent {
        name: AgentName::new("test"),
        persona: PersonaName::new("expert"),
        description: Description::default(),
        prompt: Prompt::default(),
    };
    let typed_link = AgentLink::try_from(agent.link().unwrap()).unwrap();
    let key: Key<AgentId, AgentLink> = Key::Both(id, typed_link);

    // Broaden to erased types
    let erased: Key<AgentId, Link> = key.map_link(Into::into);
    assert!(erased.try_id().is_some());
    assert!(erased.try_link().is_some());
}

#[test]
fn key_narrow_succeeds_for_matching_link() {
    let agent = Agent {
        name: AgentName::new("test"),
        persona: PersonaName::new("expert"),
        description: Description::default(),
        prompt: Prompt::default(),
    };
    let link = agent.link().unwrap();

    // Start with an erased key
    let erased: Key<AgentId, Link> = Key::Link(link);

    // Narrow the link side
    let result: Result<Key<AgentId, AgentLink>, LinkNarrowingError> = erased.narrow();
    assert!(result.is_ok());
}

#[test]
fn key_narrow_fails_for_wrong_link() {
    // A cognition link passed where an agent link is expected
    let link = Link::new(&("cognition", "working", "some thought")).unwrap();
    let erased: Key<AgentId, Link> = Key::Link(link);

    let result: Result<Key<AgentId, AgentLink>, LinkNarrowingError> = erased.narrow();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err.expected, "agent");
}

#[test]
fn key_narrow_preserves_id_on_both() {
    let id = AgentId::new();
    let agent = Agent {
        name: AgentName::new("test"),
        persona: PersonaName::new("expert"),
        description: Description::default(),
        prompt: Prompt::default(),
    };
    let link = agent.link().unwrap();

    let erased: Key<AgentId, Link> = Key::Both(id.clone(), link);
    let narrowed: Key<AgentId, AgentLink> = erased.narrow().unwrap();

    assert_eq!(narrowed.try_id(), Some(&id));
    assert!(narrowed.try_link().is_some());
}

#[test]
fn key_display_shows_id_when_both() {
    let id = AgentId::new();
    let agent = Agent {
        name: AgentName::new("test"),
        persona: PersonaName::new("expert"),
        description: Description::default(),
        prompt: Prompt::default(),
    };
    let typed_link = AgentLink::try_from(agent.link().unwrap()).unwrap();

    let key: Key<AgentId, AgentLink> = Key::Both(id.clone(), typed_link);
    assert_eq!(key.to_string(), id.to_string());
}

#[test]
fn key_display_shows_link_when_link_only() {
    let agent = Agent {
        name: AgentName::new("test"),
        persona: PersonaName::new("expert"),
        description: Description::default(),
        prompt: Prompt::default(),
    };
    let typed_link = AgentLink::try_from(agent.link().unwrap()).unwrap();
    let expected = typed_link.to_string();

    let key: Key<AgentId, AgentLink> = Key::Link(typed_link);
    assert_eq!(key.to_string(), expected);
}
