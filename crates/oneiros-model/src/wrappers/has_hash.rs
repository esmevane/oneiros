use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HasHash<T> {
    pub hash: ContentHash,
    #[serde(flatten)]
    inner: T,
}

impl<T> HasHash<T> {
    pub fn new(hash: impl Into<ContentHash>, inner: T) -> Self {
        Self {
            hash: hash.into(),
            inner,
        }
    }
}

impl<T> core::ops::Deref for HasHash<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> AsLink for HasHash<T>
where
    T: AsLink,
    T::Linkable: Linkable,
{
    type Linkable = T::Linkable;

    fn as_link(&self) -> Result<Self::Linkable, LinkError> {
        self.inner.as_link()
    }
}

impl<T> Linkable for HasHash<T>
where
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

    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Widget {
        name: String,
        color: String,
    }

    #[test]
    fn stores_hash_with_shape() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };

        let wrapped = HasHash::new(ContentHash::new("abc123"), widget);

        assert_eq!(wrapped.hash, ContentHash::new("abc123"));
        assert_eq!(wrapped.name, "sprocket");
    }

    #[test]
    fn serializes_flat() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };

        let wrapped = HasHash::new(ContentHash::new("abc123"), widget);
        let json = serde_json::to_value(&wrapped).unwrap();

        assert!(json.get("hash").is_some());
        assert!(json.get("name").is_some());
        assert!(json.get("color").is_some());
        assert!(json.get("inner").is_none());
    }

    #[test]
    fn delegates_as_link_to_inner() {
        let widget = Actor {
            tenant_id: TenantId::new(),
            name: ActorName::new("red"),
        };

        let expected = widget.as_link().unwrap();
        let wrapped = HasHash::new(ContentHash::new("abc123"), widget);

        assert_eq!(wrapped.as_link().unwrap(), expected);
    }

    #[test]
    fn different_hashes_have_the_same_link() {
        let tenant_id = TenantId::new();
        let has_hash = HasHash::new(
            ContentHash::new("abc123"),
            Actor {
                tenant_id,
                name: ActorName::new("red"),
            },
        );

        let has_other_hash = HasHash::new(
            ContentHash::new("def456"),
            Actor {
                tenant_id,
                name: ActorName::new("red"),
            },
        );

        assert_eq!(
            has_hash.as_link().unwrap(),
            has_other_hash.as_link().unwrap()
        );
    }
}
