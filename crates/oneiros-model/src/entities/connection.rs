use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum ConnectionConstructionError {
    #[error("invalid connection id: {0}")]
    InvalidId(#[from] IdParseError),
    #[error("invalid from_ref: {0}")]
    InvalidFromRef(String),
    #[error("invalid to_ref: {0}")]
    InvalidToRef(String),
    #[error("invalid created_at timestamp: {0}")]
    InvalidCreatedAt(#[from] TimestampParseError),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Connection {
    pub id: ConnectionId,
    pub nature: NatureName,
    pub from_ref: Ref,
    pub to_ref: Ref,
    pub created_at: Timestamp,
}

impl Connection {
    pub fn create(nature: NatureName, from_ref: Ref, to_ref: Ref) -> Self {
        Self {
            id: ConnectionId::from(Id::new()),
            nature,
            from_ref,
            to_ref,
            created_at: Timestamp::now(),
        }
    }

    pub fn ref_token(&self) -> RefToken {
        RefToken::new(Ref::connection(self.id))
    }

    pub fn as_table_row(&self) -> String {
        let nature = format!("{}", self.nature);
        let from = RefToken::new(self.from_ref.clone()).to_string();
        let to = RefToken::new(self.to_ref.clone()).to_string();

        let from_short = if from.len() > 32 {
            let end = from.floor_char_boundary(32);
            format!("{}...", &from[..end])
        } else {
            from
        };

        let to_short = if to.len() > 32 {
            let end = to.floor_char_boundary(32);
            format!("{}...", &to[..end])
        } else {
            to
        };

        format!("{nature:<14} {from_short} → {to_short}")
    }

    pub fn as_detail(&self) -> String {
        let from_token = RefToken::new(self.from_ref.clone());
        let to_token = RefToken::new(self.to_ref.clone());

        let lines = [
            format!("  Nature: {}", self.nature),
            format!("  From: {from_token}"),
            format!("  To: {to_token}"),
        ];

        lines.join("\n")
    }

    pub fn construct_from_db(
        (id, nature, from_ref, to_ref, created_at): (
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
        ),
    ) -> Result<Self, ConnectionConstructionError> {
        let from = Self::parse_ref(from_ref.as_ref())
            .map_err(ConnectionConstructionError::InvalidFromRef)?;
        let to =
            Self::parse_ref(to_ref.as_ref()).map_err(ConnectionConstructionError::InvalidToRef)?;

        Ok(Connection {
            id: id.as_ref().parse()?,
            nature: NatureName::new(nature),
            from_ref: from,
            to_ref: to,
            created_at: Timestamp::parse_str(created_at)?,
        })
    }

    /// Parse a ref string, trying JSON first (new format), then RefToken (legacy).
    fn parse_ref(s: &str) -> Result<Ref, String> {
        serde_json::from_str::<Ref>(s)
            .or_else(|_| s.parse::<RefToken>().map(RefToken::into_inner))
            .map_err(|e| e.to_string())
    }
}

impl core::fmt::Display for Connection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}", self.ref_token(), self.as_table_row())
    }
}

domain_id!(ConnectionId);
