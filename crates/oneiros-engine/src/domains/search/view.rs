//! Search view — re-exports SearchPresenter under the View name.
//!
//! The presenter already owns the full rendering logic. This alias keeps
//! the naming consistent with other domains that expose a `*View` type.

pub use crate::SearchPresenter as SearchView;
