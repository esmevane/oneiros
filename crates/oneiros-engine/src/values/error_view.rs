//! Error presentation for CLI output — parallels `Rendered<T>` for the
//! error path. Formats engine errors consistently for console display
//! rather than letting them fall through to Rust's default error output.

use std::fmt;

use crate::{Error, Paint};

/// A formatted view of an engine error for CLI output.
///
/// Where `Rendered<Responses>` carries successful output with prompt
/// and text modes, `ErrorView` carries a failed result with formatted
/// text for terminal display. The CLI uses this to render errors
/// through the same output pipeline as successful responses.
pub(crate) struct ErrorView {
    error: Error,
}

impl ErrorView {
    pub(crate) fn new(error: Error) -> Self {
        Self { error }
    }

    /// Formatted text for terminal output — styled with the error palette.
    pub(crate) fn text(&self) -> String {
        format!("{}: {}", "error".error(), self.error)
    }
}

impl fmt::Display for ErrorView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl From<Error> for ErrorView {
    fn from(error: Error) -> Self {
        Self::new(error)
    }
}
