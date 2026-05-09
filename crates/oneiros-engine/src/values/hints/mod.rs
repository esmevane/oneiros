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

pub(crate) use agent_created_hints::*;
pub(crate) use cognition_added_hints::*;
pub(crate) use hint::*;
pub(crate) use hint_level::*;
pub(crate) use hint_set::*;
pub(crate) use hint_template::*;
pub(crate) use listing_hints::*;
pub(crate) use mutation_hints::*;
pub(crate) use reflect_hints::*;
pub(crate) use vocabulary_hints::*;
pub(crate) use wake_hints::*;
