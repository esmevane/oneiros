//! Presentation types — rendered results that carry typed data
//! alongside presentation forms.
//!
//! These are crate-level types that domain presenters produce.
//! Each domain owns its own presenter; these types are the shared
//! vocabulary they return through.

use crate::*;

/// A rendered result that carries typed data alongside presentation.
///
/// Always retains `data` for programmatic access. The prompt
/// and text fields carry richer representations when a domain presenter
/// produced them. Empty strings indicate no presentation is available
/// for that mode — the caller falls back to serializing `data`.
#[derive(Debug)]
pub struct Rendered<T> {
    data: T,
    prompt: String,
    text: String,
}

impl<T> Rendered<T> {
    /// Construct with all representations.
    pub(crate) fn new(data: T, prompt: String, text: String) -> Self {
        Self { data, prompt, text }
    }

    /// The typed response — always available.
    pub fn response(&self) -> &T {
        &self.data
    }

    /// Consume into the typed response, discarding presentation.
    pub(crate) fn into_response(self) -> T {
        self.data
    }

    /// The rendered prompt, if a presenter produced one.
    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    /// The text summary, if a presenter produced one.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Whether this has a richer representation than raw data.
    pub fn has_prompt(&self) -> bool {
        !self.prompt.is_empty()
    }

    /// Whether this has a text summary.
    pub fn has_text(&self) -> bool {
        !self.text.is_empty()
    }
}

/// Default rendering — data only, no presentation.
impl From<Responses> for Rendered<Responses> {
    fn from(data: Responses) -> Self {
        Self {
            data,
            prompt: String::new(),
            text: String::new(),
        }
    }
}
