use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HasDescription<T> {
    pub description: Description,
    #[serde(flatten)]
    inner: T,
}

impl<T> HasDescription<T> {
    pub fn new(description: impl Into<Description>, inner: T) -> Self {
        Self {
            description: description.into(),
            inner,
        }
    }
}

impl<T> core::ops::Deref for HasDescription<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> AsLink for HasDescription<T>
where
    T: AsLink,
    T::Linkable: Linkable,
{
    type Linkable = T::Linkable;

    fn as_link(&self) -> Result<Self::Linkable, LinkError> {
        self.inner.as_link()
    }
}

impl<T> Linkable for HasDescription<T>
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
    fn stores_id_with_shape() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };

        let identity = HasDescription::new("Super", widget);

        assert_eq!(identity.description, Description::new("Super"));
        assert_eq!(identity.name, "sprocket");
    }

    #[test]
    fn serializes_flat() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };

        let identity = HasDescription::new("Super", widget);
        let json = serde_json::to_value(&identity).unwrap();

        assert!(json.get("description").is_some());
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
        let identity = HasDescription::new("Super", widget);

        assert_eq!(identity.as_link().unwrap(), expected);
    }

    #[test]
    fn different_identities_have_the_same_link() {
        let tenant_id = TenantId::new();
        let has_content = HasDescription::new(
            "Super",
            Actor {
                tenant_id,
                name: ActorName::new("red"),
            },
        );

        let has_other_content = HasDescription::new(
            "Duper",
            Actor {
                tenant_id,
                name: ActorName::new("red"),
            },
        );

        assert_eq!(
            has_content.as_link().unwrap(),
            has_other_content.as_link().unwrap()
        );
    }
}
