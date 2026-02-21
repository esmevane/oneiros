use oneiros_link::*;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{fmt, ops::Deref};

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
        let prefix = if id.len() >= 8 { &id[..8] } else { &id };
        write!(f, "{prefix:<10}{}", self.inner)
    }
}

impl<I, T> Deref for Identity<I, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<I, T> AsLink for Identity<I, T>
where
    T: AsLink,
    T::Linkable: Linkable,
{
    type Linkable = T::Linkable;

    fn as_link(&self) -> Result<Self::Linkable, LinkError> {
        self.inner.as_link()
    }
}

impl<I, T> Linkable for Identity<I, T>
where
    I: Serialize + DeserializeOwned,
    T: Linkable,
{
    fn to_link(&self) -> Result<Link, LinkError> {
        self.inner.to_link()
    }

    fn to_link_string(&self) -> Result<String, LinkError> {
        self.inner.to_link_string()
    }
}

#[cfg(test)]
mod tests {
    use oneiros_link::*;
    use pretty_assertions::assert_eq;

    use crate::*;

    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Widget {
        name: String,
        color: String,
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
    fn delegates_as_link_to_inner() {
        let widget = Actor {
            tenant_id: TenantId::new(),
            name: ActorName::new("red"),
        };

        let expected = widget.as_link().unwrap();
        let identity = Identity::new(42u64, widget);

        assert_eq!(identity.as_link().unwrap(), expected);
    }

    #[test]
    fn different_identities_have_the_same_link() {
        let tenant_id = TenantId::new();
        let identity_one = Identity::new(
            1u64,
            Actor {
                tenant_id,
                name: ActorName::new("red"),
            },
        );

        let identity_two = Identity::new(
            2u64,
            Actor {
                tenant_id,
                name: ActorName::new("red"),
            },
        );

        assert_eq!(
            identity_one.as_link().unwrap(),
            identity_two.as_link().unwrap()
        );
    }

    #[test]
    fn into_inner_returns_shape() {
        let tenant_id = TenantId::new();
        let widget = Actor {
            tenant_id,
            name: ActorName::new("red"),
        };

        let identity = Identity::new(42u64, widget.clone());

        assert_eq!(identity.into_inner(), widget);
    }
}
