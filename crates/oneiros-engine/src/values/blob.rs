use std::io::{Read, Write};

use data_encoding::BASE64URL_NOPAD;
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use serde::{Deserialize, Serialize};

/// Error type for [`Blob`] decode operations.
#[derive(Debug, thiserror::Error)]
pub enum BlobError {
    #[error("invalid blob encoding: {0}")]
    Encoding(#[from] data_encoding::DecodeError),
    #[error("invalid blob format: {0}")]
    Format(#[from] postcard::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Opaque binary payload encoded as postcard+base64url for safe JSON embedding.
///
/// `Blob` wraps arbitrary `&[u8]` into a string-typed value that round-trips
/// through JSON, SQLite text columns, and query parameters without escaping
/// issues. The inner string is `BASE64URL_NOPAD`-encoded postcard bytes.
///
/// # Examples
///
/// ```
/// # use oneiros_engine::Blob;
/// let blob = Blob::encode(b"hello \x00\xff binary");
/// let decoded = blob.decode().unwrap();
/// assert_eq!(decoded, b"hello \x00\xff binary");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(transparent)]
pub struct Blob(String);

impl Blob {
    /// Encode raw bytes into a [`Blob`].
    ///
    /// Serializes `bytes` with postcard, then base64url-encodes the result.
    /// The encoding is deterministic for the same input.
    pub fn encode(bytes: &[u8]) -> Self {
        let postcard_bytes =
            postcard::to_allocvec(bytes).expect("blob serialization should not fail");
        Self(BASE64URL_NOPAD.encode(&postcard_bytes))
    }

    /// Decode this [`Blob`] back into raw bytes.
    ///
    /// Returns [`BlobError::Encoding`] if the inner string is not valid
    /// base64url, or [`BlobError::Format`] if the decoded bytes are not valid
    /// postcard.
    pub fn decode(&self) -> Result<Vec<u8>, BlobError> {
        let postcard_bytes = BASE64URL_NOPAD.decode(self.0.as_bytes())?;
        Ok(postcard::from_bytes(&postcard_bytes)?)
    }

    pub fn decompressed(&self) -> Result<Vec<u8>, BlobError> {
        let compressed_bytes = self.decode()?;
        let mut decoder = ZlibDecoder::new(&compressed_bytes[..]);
        let mut decompressed = Vec::new();

        decoder.read_to_end(&mut decompressed)?;

        Ok(decompressed)
    }

    pub fn compressed(bytes: &[u8]) -> Result<Self, BlobError> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

        encoder.write_all(bytes)?;

        let compressed_bytes = encoder.finish()?;

        Ok(Self::encode(&compressed_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blob_roundtrip() {
        let original = b"hello world, this is binary \x00\xff data";
        let blob = Blob::encode(original);
        let decoded = blob.decode().unwrap();
        assert_eq!(original.as_slice(), &decoded);
    }

    #[test]
    fn blob_serde_as_string() {
        let blob = Blob::encode(b"test data");
        let json = serde_json::to_value(&blob).unwrap();
        assert!(json.is_string(), "should serialize as plain string");
        let decoded: Blob = serde_json::from_value(json).unwrap();
        assert_eq!(blob.decode().unwrap(), decoded.decode().unwrap());
    }

    #[test]
    fn blob_empty_roundtrip() {
        let blob = Blob::encode(b"");
        let decoded = blob.decode().unwrap();
        assert!(decoded.is_empty());
    }

    #[test]
    fn blob_invalid_base64_fails() {
        let blob = Blob("!!!invalid!!!".into());
        assert!(blob.decode().is_err());
    }
}
