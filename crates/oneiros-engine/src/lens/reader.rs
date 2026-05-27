use crate::*;

pub(crate) trait Reader {
    fn read(&self, read: &Read) -> Option<Result<Selection, ReaderError>>;
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ReaderError {
    #[error("reader failed: {0}")]
    Internal(String),
}
