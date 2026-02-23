use core::str::FromStr;
use oneiros_link::*;

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum KeyMisuseError {
    #[error("Key is not an id")]
    NoId,
    #[error("Key is not a link")]
    NoLink,
}

#[derive(Debug, thiserror::Error)]
pub enum KeyParseError {
    #[error("malformed link: {0}")]
    MalformedLink(#[from] LinkError),
    #[error("malformed key: {0}")]
    Malformed(String),
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Key<GivenId, GivenLink> {
    Id(GivenId),
    Link(GivenLink),
}

impl<GivenId, GivenLink> Key<GivenId, GivenLink> {
    pub fn to_id(self) -> Result<GivenId, KeyMisuseError> {
        match self {
            Self::Id(id) => Ok(id),
            _ => Err(KeyMisuseError::NoId),
        }
    }

    pub fn to_link(self) -> Result<GivenLink, KeyMisuseError> {
        match self {
            Self::Link(link) => Ok(link),
            _ => Err(KeyMisuseError::NoLink),
        }
    }
}

impl<GivenId, GivenLink> FromStr for Key<GivenId, GivenLink>
where
    GivenId: From<Id>,
    GivenLink: TryFrom<Link>,
    GivenLink::Error: Into<LinkError>,
{
    type Err = KeyParseError;

    fn from_str(given_str: &str) -> Result<Self, Self::Err> {
        if let Ok(id) = given_str.parse::<Id>() {
            return Ok(Self::Id(GivenId::from(id)));
        }

        if let Ok(link) = given_str.parse::<Link>() {
            let outcome = GivenLink::try_from(link).map_err(Into::into)?;

            return Ok(Self::Link(outcome));
        }

        Err(KeyParseError::Malformed(given_str.to_string()))
    }
}

impl<GivenId, GivenLink> core::fmt::Display for Key<GivenId, GivenLink>
where
    GivenId: core::fmt::Display,
    GivenLink: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Id(id) => id.fmt(f),
            Self::Link(link) => link.fmt(f),
        }
    }
}
