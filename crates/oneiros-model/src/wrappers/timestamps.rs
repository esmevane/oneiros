use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, thiserror::Error)]
#[error("failed to construct timestamps from database values: {0}")]
pub struct TimestampConstructionFailure(#[from] TimestampParseError);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Timestamps<T> {
    pub created_at: Timestamp,
    #[serde(flatten)]
    inner: T,
}

impl<T> Timestamps<T> {
    pub fn now(inner: T) -> Self {
        Self {
            created_at: Timestamp::now(),
            inner,
        }
    }

    pub fn new(
        created_at: impl AsRef<str>,
        inner: T,
    ) -> Result<Self, TimestampConstructionFailure> {
        Ok(Self {
            created_at: Timestamp::parse_str(created_at)?,
            inner,
        })
    }

    /// Consume the wrapper, returning the inner value.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: core::fmt::Display> core::fmt::Display for Timestamps<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} (created_at: {})", self.inner, self.created_at)
    }
}

impl<T> core::ops::Deref for Timestamps<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> AsLink for Timestamps<T>
where
    T: AsLink,
    T::Linkable: Linkable,
{
    type Linkable = T::Linkable;

    fn as_link(&self) -> Result<Self::Linkable, LinkError> {
        self.inner.as_link()
    }
}

impl<T> Linkable for Timestamps<T>
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
    use chrono::Utc;
    use oneiros_link::*;
    use pretty_assertions::assert_eq;

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

        let before = Timestamp::now();
        let identity = Timestamps::now(widget);
        let after = Timestamp::now();

        assert!(identity.created_at >= before && identity.created_at <= after);

        assert_eq!(identity.name, "sprocket");
    }

    #[test]
    fn serializes_flat() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };

        let identity = Timestamps::now(widget);
        let json = serde_json::to_value(&identity).unwrap();

        assert!(json.get("created_at").is_some());
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
        let identity = Timestamps::now(widget);

        assert_eq!(identity.as_link().unwrap(), expected);
    }

    #[test]
    fn different_identities_have_the_same_link() {
        let tenant_id = TenantId::new();
        let timestamp_one = Timestamps::new(
            Utc::now().to_rfc3339(),
            Actor {
                tenant_id,
                name: ActorName::new("red"),
            },
        )
        .unwrap();

        let timestamp_two = Timestamps::new(
            "2026-02-01T00:00:00Z",
            Actor {
                tenant_id,
                name: ActorName::new("red"),
            },
        )
        .unwrap();

        assert_eq!(
            timestamp_one.as_link().unwrap(),
            timestamp_two.as_link().unwrap()
        );
    }
}
