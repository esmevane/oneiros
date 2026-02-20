use std::fmt;
use std::ops::Deref;

use oneiros_link::*;
use serde::{Deserialize, Serialize};

/// A shape paired with an external identifier.
///
/// `Identity<I, T>` wraps a value with an identifier of type `I` (typically
/// a domain ID like `AgentId`). The identifier is a database/persistence
/// concern — it does not participate in content addressing. When `T`
/// implements [`Addressable`], `Identity<I, T>` delegates to it: the link
/// is computed from `T`'s identity fields, not from `I`.
///
/// Serde flattening means the serialized form includes the id alongside
/// the inner fields with no nesting.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity<I, T> {
    pub id: I,
    #[serde(flatten)]
    inner: T,
}

impl<I, T> Identity<I, T> {
    pub fn new(id: I, inner: T) -> Self {
        Self { id, inner }
    }

    /// Consume the wrapper, returning the inner value.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<I: fmt::Display, T: fmt::Display> fmt::Display for Identity<I, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = self.id.to_string();
        write!(f, "{id:<30}{}", self.inner)
    }
}

impl<I, T> Deref for Identity<I, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<I, T: Addressable> Addressable for Identity<I, T> {
    fn address_label() -> &'static str {
        T::address_label()
    }

    fn link(&self) -> Result<Link, LinkError> {
        self.inner.link()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
    fn stores_id_with_shape() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };
        let identity = Identity::new(42u64, widget);
        assert_eq!(identity.id, 42);
        assert_eq!(identity.name, "sprocket");
    }

    #[test]
    fn derefs_to_inner() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };
        let identity = Identity::new(42u64, widget);
        assert_eq!(identity.color, "red");
    }

    #[test]
    fn serializes_flat() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };
        let identity = Identity::new(42u64, widget);
        let json = serde_json::to_value(&identity).unwrap();

        assert!(json.get("id").is_some());
        assert!(json.get("name").is_some());
        assert!(json.get("color").is_some());
        assert!(json.get("inner").is_none());
    }

    #[test]
    fn roundtrips_through_serde() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };
        let identity = Identity::new(42u64, widget);
        let json = serde_json::to_string(&identity).unwrap();
        let deserialized: Identity<u64, Widget> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, identity);
    }

    #[test]
    fn delegates_addressable_to_inner() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };
        let expected = widget.link().unwrap();
        let identity = Identity::new(42u64, widget);
        assert_eq!(identity.link().unwrap(), expected);
    }

    #[test]
    fn different_id_same_address() {
        let i1 = Identity::new(
            1u64,
            Widget {
                name: "sprocket".into(),
                color: "red".into(),
            },
        );
        let i2 = Identity::new(
            2u64,
            Widget {
                name: "sprocket".into(),
                color: "red".into(),
            },
        );
        assert_eq!(i1.link().unwrap(), i2.link().unwrap());
    }

    #[test]
    fn into_inner_returns_shape() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };
        let identity = Identity::new(42u64, widget.clone());
        assert_eq!(identity.into_inner(), widget);
    }
}
