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
pub struct ErrorView {
    error: Error,
}

impl ErrorView {
    pub fn new(error: Error) -> Self {
        Self { error }
    }

    /// The underlying error.
    pub fn error(&self) -> &Error {
        &self.error
    }

    /// Formatted text for terminal output — styled with the error palette.
    pub fn text(&self) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_includes_error_message() {
        let error = Error::Context("something went wrong".into());
        let view = ErrorView::new(error);
        let text = view.text();

        assert!(text.contains("something went wrong"));
        assert!(text.contains("error"));
    }

    #[test]
    fn display_matches_text() {
        let error = Error::Context("test failure".into());
        let view = ErrorView::new(error);

        assert_eq!(view.to_string(), view.text());
    }

    #[test]
    fn from_error_conversion() {
        let error = Error::Context("converted".into());
        let view: ErrorView = error.into();

        assert!(view.text().contains("converted"));
    }
}
