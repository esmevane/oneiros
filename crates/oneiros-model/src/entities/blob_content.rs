use serde::{Deserialize, Serialize};

use crate::*;

/// Error type for [`Blob`] decode operations.
#[derive(Debug, thiserror::Error)]
pub enum BlobContentError {
    #[error(transparent)]
    Blob(#[from] BlobError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobContent {
    pub hash: ContentHash,
    pub size: Size,
    pub data: Blob,
}

impl BlobContent {
    pub fn create(data: &[u8]) -> Result<Self, BlobContentError> {
        let hash = ContentHash::compute(data);
        let size = data.len().into();
        let blob = Blob::compressed(data)?;

        Ok(Self {
            hash,
            size,
            data: blob,
        })
    }

    // let mut hasher = Sha256::new();
    // hasher.update(data);
    // let hash_bytes = hasher.finalize();
    // let hash_hex = data_encoding::HEXLOWER.encode(&hash_bytes);

    // let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    // encoder.write_all(data)?;
    // let compressed = encoder.finish()?;

    pub fn decode(&self) -> Result<Vec<u8>, BlobError> {
        self.data.decode()
    }
}
