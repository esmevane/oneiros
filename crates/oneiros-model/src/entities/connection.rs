use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum ConnectionConstructionError {
    #[error("invalid connection id: {0}")]
    InvalidId(#[from] IdParseError),
    #[error("invalid from_link: {0}")]
    InvalidFromLink(oneiros_link::LinkError),
    #[error("invalid to_link: {0}")]
    InvalidToLink(oneiros_link::LinkError),
    #[error("invalid created_at timestamp: {0}")]
    InvalidCreatedAt(#[from] TimestampParseError),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Connection {
    pub id: ConnectionId,
    pub nature: NatureName,
    pub from_link: Link,
    pub to_link: Link,
    pub created_at: Timestamp,
}

impl Connection {
    pub fn create(nature: NatureName, from_link: Link, to_link: Link) -> Self {
        Self {
            id: ConnectionId::from(Id::new()),
            nature,
            from_link,
            to_link,
            created_at: Timestamp::now(),
        }
    }

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
    ) -> Result<Self, ConnectionConstructionError> {
        Ok(Connection {
            id: id.as_ref().parse()?,
            nature: NatureName::new(nature),
            from_link: from_link
                .as_ref()
                .parse()
                .map_err(ConnectionConstructionError::InvalidFromLink)?,
            to_link: to_link
                .as_ref()
                .parse()
                .map_err(ConnectionConstructionError::InvalidToLink)?,
            created_at: Timestamp::parse_str(created_at)?,
        })
    }
}

impl core::fmt::Display for Connection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let id = self.id.to_string();
        let prefix = if id.len() >= 8 { &id[..8] } else { &id };
        write!(f, "{prefix:<10}{}", self.as_table_row())
    }
}

domain_link!(Connection => ConnectionLink);
domain_id!(ConnectionId);
