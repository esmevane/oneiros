use serde::{Serialize, de::DeserializeOwned};

use crate::{Link, LinkError};

pub trait Linkable: Serialize + DeserializeOwned {
    fn from_string(given_string: impl AsRef<str>) -> Result<Self, LinkError> {
        Ok(postcard::from_bytes(
            &data_encoding::BASE64URL_NOPAD.decode(given_string.as_ref().as_bytes())?,
        )?)
    }

    fn to_link(&self) -> Result<Link, LinkError> {
        self.to_link_string().and_then(Link::from_string)
    }

    fn to_link_string(&self) -> Result<String, LinkError> {
        Ok(data_encoding::BASE64URL_NOPAD.encode(&postcard::to_allocvec(self)?))
    }
}

impl Linkable for Link {}
