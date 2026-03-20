use std::future::Future;

pub type TestResult = Result<(), Box<dyn core::error::Error>>;

/// A test backend that can execute CLI commands and return typed responses.
///
/// The engine's `Response<Responses>` envelope is the specification.
/// Both backends must produce it — the engine directly, the legacy
/// by deserializing HTTP responses into the same types.
pub trait Backend: Sized {
    /// Create a new backend instance ready to execute commands.
    fn start() -> impl Future<Output = Result<Self, Box<dyn core::error::Error>>>;

    /// Execute a CLI subcommand string and return the typed response envelope.
    fn exec(
        &self,
        command: &str,
    ) -> impl Future<Output = Result<oneiros_engine::Response<oneiros_engine::Responses>, oneiros_engine::Error>>;

    /// Start the service. Required before executing brain-scoped commands.
    fn start_service(&mut self) -> impl Future<Output = Result<(), Box<dyn core::error::Error>>>;
}
