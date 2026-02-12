use crate::StorageKey;

#[derive(serde::Serialize, serde::Deserialize)]
enum StorageRefVersion {
    V0(StorageKey),
}

#[derive(Debug, thiserror::Error)]
pub enum StorageRefError {
    #[error("Invalid storage ref encoding")]
    Encoding,

    #[error("Invalid storage ref format: {0}")]
    Format(#[from] postcard::Error),
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct StorageRef(pub String);

impl StorageRef {
    /// Encode a storage key into a URL-safe reference.
    pub fn encode(key: &StorageKey) -> Self {
        let versioned = StorageRefVersion::V0(key.clone());
        let bytes =
            postcard::to_allocvec(&versioned).expect("storage ref serialization should not fail");
        Self(data_encoding::BASE32_NOPAD.encode(&bytes).to_lowercase())
    }

    /// Decode this reference back to a storage key.
    pub fn decode(&self) -> Result<StorageKey, StorageRefError> {
        let upper = self.0.to_uppercase();
        let bytes = data_encoding::BASE32_NOPAD
            .decode(upper.as_bytes())
            .map_err(|_| StorageRefError::Encoding)?;
        let versioned: StorageRefVersion = postcard::from_bytes(&bytes)?;
        let StorageRefVersion::V0(key) = versioned;
        Ok(key)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for StorageRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl core::str::FromStr for StorageRef {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
