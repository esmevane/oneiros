use chrono::{DateTime, Utc};
use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

use super::ConnectionConstructionError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Connection {
    pub nature: NatureName,
    pub from_link: Link,
    pub to_link: Link,
    pub created_at: DateTime<Utc>,
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
            format!("  Created: {}", self.created_at),
        ];

        lines.join("\n")
    }
}

impl core::fmt::Display for Connection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_table_row())
    }
}

impl Connection {
    pub fn construct_from_db(
        (id, nature, from_link, to_link, created_at): (
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
        ),
    ) -> Result<Identity<ConnectionId, Self>, ConnectionConstructionError> {
        let id: ConnectionId = id
            .as_ref()
            .parse()
            .map_err(ConnectionConstructionError::InvalidId)?;
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
            created_at: created_at
                .as_ref()
                .parse::<DateTime<Utc>>()
                .map_err(ConnectionConstructionError::InvalidCreatedAt)?,
        };
        Ok(Identity::new(id, connection))
    }
}

impl Addressable for Connection {
    fn address_label() -> &'static str {
        "connection"
    }

    fn link(&self) -> Result<Link, LinkError> {
        // Identity: nature + from + to. Timestamp is context.
        Link::new(&(
            Self::address_label(),
            &self.nature,
            &self.from_link,
            &self.to_link,
        ))
    }
}

domain_id!(ConnectionId);
oneiros_link::domain_link!(ConnectionLink, "connection");

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
            created_at: Utc::now(),
        };

        // Different timestamp — same link
        let other = Connection {
            nature: NatureName::new("origin"),
            from_link: link_a,
            to_link: link_b,
            created_at: Utc::now(),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }

    #[test]
    fn connection_different_nature_different_link() {
        let link_a = Link::new(&("test", "entity-a")).unwrap();
        let link_b = Link::new(&("test", "entity-b")).unwrap();

        let origin = Connection {
            nature: NatureName::new("origin"),
            from_link: link_a.clone(),
            to_link: link_b.clone(),
            created_at: Utc::now(),
        };

        let context = Connection {
            nature: NatureName::new("context"),
            from_link: link_a,
            to_link: link_b,
            created_at: Utc::now(),
        };

        assert_ne!(origin.link().unwrap(), context.link().unwrap());
    }
}
