//! Hint sets — named types that carry context and produce navigational
//! breadcrumbs for consuming agents.
//!
//! Each hint set is a first-class type with its own data requirements.
//! `HintSet` is the enum that wraps them all, providing a uniform
//! interface for hint production and rendering.
//!
//! The rendering template lives at `templates/hints.md`.
mod agent_created_hints;
mod cognition_added_hints;
mod hint;
mod hint_level;
mod hint_set;
mod hint_template;
mod listing_hints;
mod mutation_hints;
mod reflect_hints;
#[cfg(test)]
mod tests;
mod vocabulary_hints;
mod wake_hints;

pub use agent_created_hints::*;
pub use cognition_added_hints::*;
pub use hint::*;
pub use hint_level::*;
pub use hint_set::*;
pub use hint_template::*;
pub use listing_hints::*;
pub use mutation_hints::*;
pub use reflect_hints::*;
pub use vocabulary_hints::*;
pub use wake_hints::*;
