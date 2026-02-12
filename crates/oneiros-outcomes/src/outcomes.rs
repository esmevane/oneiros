use std::panic::Location;

use crate::Reportable;

/// Dispatches a tracing event at a runtime-determined level.
///
/// `tracing::event!` requires compile-time constant levels, so we match
/// on the level and dispatch to the appropriate level-specific macro.
macro_rules! dyn_event {
    ($level:expr, $file:expr, $line:expr, $msg:expr) => {
        match $level {
            ::tracing::Level::TRACE => {
                ::tracing::trace!(caller.file = $file, caller.line = $line, "{}", $msg)
            }
            ::tracing::Level::DEBUG => {
                ::tracing::debug!(caller.file = $file, caller.line = $line, "{}", $msg)
            }
            ::tracing::Level::INFO => {
                ::tracing::info!(caller.file = $file, caller.line = $line, "{}", $msg)
            }
            ::tracing::Level::WARN => {
                ::tracing::warn!(caller.file = $file, caller.line = $line, "{}", $msg)
            }
            ::tracing::Level::ERROR => {
                ::tracing::error!(caller.file = $file, caller.line = $line, "{}", $msg)
            }
        }
    };
}

/// A collection of outcomes that emits tracing events as outcomes are added.
///
/// The only public way to add outcomes is [`emit`](Outcomes::emit), which
/// fires a tracing event with the caller's source location captured via
/// `#[track_caller]`. This ensures every outcome is traced at the point
/// it was produced, not when it's later inspected.
///
/// Use [`map_into`](Outcomes::map_into) to convert between outcome types
/// (e.g., wrapping child outcomes into parent enum variants) without
/// re-emitting.
pub struct Outcomes<T>(Vec<T>);

impl<T> Outcomes<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Consume the collection and return the inner Vec.
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Convert outcomes into a different type without re-emitting.
    ///
    /// Use this for wrapping child outcomes into parent enum variants
    /// (e.g., `InitOutcomes` -> `SystemOutcome`). The outcomes have
    /// already been emitted, so this is a pure data transformation.
    pub fn map_into<U: From<T>>(self) -> Outcomes<U> {
        Outcomes(self.0.into_iter().map(Into::into).collect())
    }
}

impl<T: Reportable> Outcomes<T> {
    /// Push an outcome and emit it as a tracing event.
    ///
    /// The `caller.file` and `caller.line` fields on the tracing event
    /// will point to wherever `emit` was called, not to this function.
    #[track_caller]
    pub fn emit(&mut self, outcome: T) {
        let loc = Location::caller();
        let file = loc.file();
        let line = loc.line();
        let message = outcome.log_message();
        let level = outcome.level();

        dyn_event!(level, file, line, message);

        self.0.push(outcome);
    }
}

impl<T> Default for Outcomes<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for Outcomes<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Outcomes<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
