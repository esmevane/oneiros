use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Tracing subscriber error: {0}")]
    TracingSubscriber(#[from] tracing_subscriber::filter::ParseError),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
}
