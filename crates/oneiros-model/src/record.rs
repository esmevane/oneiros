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
pub struct Record<T> {
    link: Link,
    #[serde(flatten)]
    inner: T,
}

impl<T: Addressable> Record<T> {
    /// Create a record by computing the link from the inner value.
    pub fn new(inner: T) -> Result<Self, LinkError> {
        let link = inner.link()?;
        Ok(Self { link, inner })
    }
}

impl<T> Record<T> {
    /// The precomputed content-addressable link.
    pub fn link(&self) -> &Link {
        &self.link
    }

    /// Consume the record, returning the inner value.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Deref for Record<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
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
            // Only name is identity; color is mutable content.
            Link::new(&(Self::address_label(), &self.name))
        }
    }

    #[test]
    fn stores_precomputed_link() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };
        let expected = widget.link().unwrap();
        let record = Record::new(widget).unwrap();
        assert_eq!(*record.link(), expected);
    }

    #[test]
    fn derefs_to_inner() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };
        let record = Record::new(widget).unwrap();
        assert_eq!(record.name, "sprocket");
        assert_eq!(record.color, "red");
    }

    #[test]
    fn serializes_flat() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };
        let record = Record::new(widget).unwrap();
        let json = serde_json::to_value(&record).unwrap();

        assert!(json.get("link").is_some());
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
        let record = Record::new(widget).unwrap();
        let json = serde_json::to_string(&record).unwrap();
        let deserialized: Record<Widget> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, record);
    }

    #[test]
    fn link_invariant_across_non_identity_fields() {
        let r1 = Record::new(Widget {
            name: "sprocket".into(),
            color: "red".into(),
        })
        .unwrap();

        let r2 = Record::new(Widget {
            name: "sprocket".into(),
            color: "blue".into(),
        })
        .unwrap();

        assert_eq!(*r1.link(), *r2.link());
    }

    #[test]
    fn different_identity_produces_different_link() {
        let r1 = Record::new(Widget {
            name: "sprocket".into(),
            color: "red".into(),
        })
        .unwrap();

        let r2 = Record::new(Widget {
            name: "gear".into(),
            color: "red".into(),
        })
        .unwrap();

        assert_ne!(*r1.link(), *r2.link());
    }

    #[test]
    fn into_inner_returns_shape() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };
        let record = Record::new(widget.clone()).unwrap();
        assert_eq!(record.into_inner(), widget);
    }
}
