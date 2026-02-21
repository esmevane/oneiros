use data_encoding::BASE64URL_NOPAD;
use serde::{Deserialize, Serialize};

use crate::LinkError;

/// A content-addressable link: deterministic postcard bytes derived from an
/// entity's identity fields.
///
/// Links are the stable identity primitive in oneiros. Given the same identity
/// fields (resource type + content), the same link is always produced. Links
/// are displayed and parsed as base64url (no padding) for compact,
/// URL-safe representation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Link(Vec<u8>);

impl Link {
    /// Create a link from any serializable content.
    ///
    /// The content should include the resource type label as its first element
    /// to ensure type discrimination. Typically called as:
    ///
    /// ```
    /// # use oneiros_link::{Link, LinkError};
    /// # fn example() -> Result<(), LinkError> {
    /// let link = Link::new(&("agent", "governor.process", "process"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(content: &impl Serialize) -> Result<Self, LinkError> {
        postcard::to_allocvec(content).map(Self).map_err(Into::into)
    }

    /// The raw postcard bytes of this link.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consume the link and return the raw bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Create a link from raw bytes. No validation is performed.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Returns true if this link has no content.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Check whether this link's content begins with the given resource label.
    ///
    /// Links are postcard-encoded tuples whose first element is the
    /// `address_label` string. This method checks that prefix without
    /// decoding the entire link.
    pub fn has_label(&self, label: &str) -> bool {
        match postcard::to_allocvec(label) {
            Ok(prefix) => self.0.starts_with(&prefix),
            Err(_) => false,
        }
    }
}

impl core::fmt::Display for Link {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", BASE64URL_NOPAD.encode(&self.0))
    }
}

impl core::str::FromStr for Link {
    type Err = LinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BASE64URL_NOPAD
            .decode(s.as_bytes())
            .map(Self)
            .map_err(|e| LinkError::Decoding(e.to_string()))
    }
}

impl Serialize for Link {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Link {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
