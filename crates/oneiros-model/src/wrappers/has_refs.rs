use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HasRefs<T> {
    #[serde(default)]
    pub refs: Vec<RecordRef>,
    #[serde(flatten)]
    inner: T,
}

impl<T> HasRefs<T> {
    pub fn new(refs: Vec<RecordRef>, inner: T) -> Self {
        Self { refs, inner }
    }
}

impl<T> core::ops::Deref for HasRefs<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T> AsLink for HasRefs<T>
where
    T: AsLink,
    T::Linkable: Linkable,
{
    type Linkable = T::Linkable;

    fn as_link(&self) -> Result<Self::Linkable, LinkError> {
        self.inner.as_link()
    }
}

impl<T> Linkable for HasRefs<T>
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
    fn stores_refs_with_shape() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };

        let wrapped = HasRefs::new(vec![], widget);

        assert!(wrapped.refs.is_empty());
        assert_eq!(wrapped.name, "sprocket");
    }

    #[test]
    fn serializes_flat() {
        let widget = Widget {
            name: "sprocket".into(),
            color: "red".into(),
        };

        let wrapped = HasRefs::new(vec![], widget);
        let json = serde_json::to_value(&wrapped).unwrap();

        assert!(json.get("refs").is_some());
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
        let wrapped = HasRefs::new(vec![], widget);

        assert_eq!(wrapped.as_link().unwrap(), expected);
    }

    #[test]
    fn different_refs_have_the_same_link() {
        let tenant_id = TenantId::new();
        let has_refs = HasRefs::new(
            vec![],
            Actor {
                tenant_id,
                name: ActorName::new("red"),
            },
        );

        let link = Link::new(&("cognition", "thought")).unwrap();
        let has_other_refs = HasRefs::new(
            vec![RecordRef::linked(link, Some(Label::new("origin")))],
            Actor {
                tenant_id,
                name: ActorName::new("red"),
            },
        );

        assert_eq!(
            has_refs.as_link().unwrap(),
            has_other_refs.as_link().unwrap()
        );
    }
}
