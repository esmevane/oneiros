use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

use super::ConnectionConstructionError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Connection {
    pub nature: NatureName,
    pub from_link: Link,
    pub to_link: Link,
}

impl Connection {
    pub fn as_table_row(&self) -> String {
        let nature = format!("{}", self.nature);
        let from = format!("{}", self.from_link);
        let to = format!("{}", self.to_link);

        let from_short = if from.len() > 16 {
            format!("{}...", &from[..16])
        } else {
            from
        };

        let to_short = if to.len() > 16 {
            format!("{}...", &to[..16])
        } else {
            to
        };

        format!("{nature:<14} {from_short} → {to_short}")
    }

    pub fn as_detail(&self) -> String {
        let lines = [
            format!("  Nature: {}", self.nature),
            format!("  From: {}", self.from_link),
            format!("  To: {}", self.to_link),
        ];

        lines.join("\n")
    }

    pub fn construct_from_db(
        (id, nature, from_link, to_link, created_at): (
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
        ),
    ) -> Result<Record<ConnectionId, Self>, ConnectionConstructionError> {
        let id: ConnectionId = id.as_ref().parse()?;

        let connection = Connection {
            nature: NatureName::new(nature),
            from_link: from_link
                .as_ref()
                .parse()
                .map_err(ConnectionConstructionError::InvalidFromLink)?,
            to_link: to_link
                .as_ref()
                .parse()
                .map_err(ConnectionConstructionError::InvalidToLink)?,
        };

        Ok(Record::build(id, connection, created_at)?)
    }
}

impl core::fmt::Display for Connection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_table_row())
    }
}

domain_link!(Connection => ConnectionLink);
domain_id!(ConnectionId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection_identity() {
        let link_a = Link::new(&("test", "entity-a")).unwrap();
        let link_b = Link::new(&("test", "entity-b")).unwrap();

        let primary = Connection {
            nature: NatureName::new("origin"),
            from_link: link_a.clone(),
            to_link: link_b.clone(),
        };

        // Different timestamp — same link
        let other = Connection {
            nature: NatureName::new("origin"),
            from_link: link_a,
            to_link: link_b,
        };

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }

    #[test]
    fn connection_different_nature_different_link() {
        let link_a = Link::new(&("test", "entity-a")).unwrap();
        let link_b = Link::new(&("test", "entity-b")).unwrap();

        let origin = Connection {
            nature: NatureName::new("origin"),
            from_link: link_a.clone(),
            to_link: link_b.clone(),
        };

        let context = Connection {
            nature: NatureName::new("context"),
            from_link: link_a,
            to_link: link_b,
        };

        assert_ne!(origin.as_link().unwrap(), context.as_link().unwrap());
    }
}
