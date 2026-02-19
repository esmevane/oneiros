use oneiros_link::{Addressable, Link, LinkError};
use oneiros_model::{Identity, Record};
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
