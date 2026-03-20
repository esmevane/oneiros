#[derive(Debug, thiserror::Error)]
pub enum SystemError {
    #[error("{0}")]
    Context(String),
}
