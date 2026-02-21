use std::ops::Deref;

use oneiros_link::*;
use serde::{Deserialize, Serialize};

/// A shape paired with its content-addressable [`Link`].
///
/// `Record<T>` is the addressed form of a value — it carries a precomputed
/// link derived from the value's identity fields. The link is computed once
/// at construction time via `T`'s [`Addressable`] implementation.
///
/// The inner value is accessible via [`Deref`], and serde flattening means
/// the serialized form includes the link alongside the inner fields with no
/// nesting.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Linked<T> {
    link: Link,
    #[serde(flatten)]
    inner: T,
}

impl<T> Linked<T>
where
    T: AsLink,
    T::Linkable: Linkable,
{
    /// Create a record by computing the link from the inner value.
    pub fn new(inner: T) -> Result<Self, LinkError> {
        let link = inner.as_link()?.to_link()?;
        Ok(Self { link, inner })
    }
}

impl<T> Linked<T> {
    /// The precomputed content-addressable link.
    pub fn link(&self) -> &Link {
        &self.link
    }

    /// Consume the record, returning the inner value.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Deref for Linked<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use oneiros_link::*;
    use pretty_assertions::assert_eq;

    use crate::*;

    use super::*;

    #[test]
    fn stores_precomputed_link() {
        let actor = Actor {
            tenant_id: TenantId::new(),
            name: ActorName::new("sprocket"),
        };

        let expected = actor.as_link().unwrap();
        let record = Linked::new(actor).unwrap();

        assert_eq!(record.as_link().unwrap(), expected);
    }

    #[test]
    fn derefs_to_inner() {
        let actor = Actor {
            tenant_id: TenantId::new(),
            name: ActorName::new("sprocket"),
        };

        let record = Linked::new(actor).unwrap();

        assert_eq!(record.name, ActorName::new("sprocket"));
    }

    #[test]
    fn serializes_flat() {
        let actor = Actor {
            tenant_id: TenantId::new(),
            name: ActorName::new("sprocket"),
        };

        let record = Linked::new(actor).unwrap();
        let json = serde_json::to_value(&record).unwrap();

        assert!(json.get("link").is_some());
        assert!(json.get("name").is_some());
        assert!(json.get("tenant_id").is_some());
    }

    #[test]
    fn link_invariant_across_non_identity_fields() {
        let tenant_id = TenantId::new();

        let record_one = Linked::new(Actor {
            tenant_id,
            name: ActorName::new("sprocket"),
        })
        .unwrap();

        let record_two = Linked::new(Actor {
            tenant_id,
            name: ActorName::new("sprocket"),
        })
        .unwrap();

        let record_three = Linked::new(Actor {
            tenant_id,
            name: ActorName::new("gear"),
        })
        .unwrap();

        assert_eq!(record_one.as_link().unwrap(), record_two.as_link().unwrap());
        assert_ne!(
            record_one.as_link().unwrap(),
            record_three.as_link().unwrap()
        );
    }

    #[test]
    fn into_inner_returns_shape() {
        let actor = Actor {
            tenant_id: TenantId::new(),
            name: ActorName::new("sprocket"),
        };

        let record = Linked::new(actor.clone()).unwrap();
        assert_eq!(record.into_inner(), actor);
    }
}
