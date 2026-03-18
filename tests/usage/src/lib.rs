use std::future::Future;

use serde_json::Value;

pub type TestResult = Result<(), Box<dyn core::error::Error>>;

/// A test backend that can execute CLI commands and return JSON results.
///
/// Backends manage their own lifecycle: temp directories, database,
/// and HTTP service. Commands are passed as subcommand strings
/// (e.g. "system init test --yes", "level set working --description '...'").
pub trait Backend: Sized {
    /// Create a new backend instance ready to execute commands.
    ///
    /// The backend should set up isolated temp directories but not
    /// perform any initialization — system init, project init, and
    /// service startup are the responsibility of test cases.
    fn start() -> impl Future<Output = Result<Self, Box<dyn core::error::Error>>>;

    /// Execute a CLI subcommand string and return the result as JSON.
    ///
    /// Commands that go through the HTTP service require the service
    /// to be running (see `start_service`). Local commands like
    /// `system init` work without a running service.
    fn exec(
        &self,
        command: &str,
    ) -> impl Future<Output = Result<Value, Box<dyn core::error::Error>>>;

    /// Start the HTTP service. Required before executing commands
    /// that communicate with the service (brain-scoped operations).
    ///
    /// Must be called after `system init` and `project init` have
    /// created the necessary database state.
    fn start_service(&mut self) -> impl Future<Output = Result<(), Box<dyn core::error::Error>>>;
}
