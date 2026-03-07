use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ContentHash(pub String);

impl ContentHash {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn compute(data: &[u8]) -> ContentHash {
        let mut hasher = Sha256::new();

        hasher.update(data);

        let hash_bytes = hasher.finalize();
        let hash_hex = data_encoding::HEXLOWER.encode(&hash_bytes);

        Self::new(hash_hex)
    }
}

impl core::fmt::Display for ContentHash {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for ContentHash {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl core::str::FromStr for ContentHash {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}
