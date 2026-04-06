use crate::*;

/// A pure function that folds an event into a canon state.
///
/// Move semantics: takes ownership of the current state, returns the
/// new state. Only clones internally if the event touches data that
/// needs it — most events mutate a single table in place.
#[derive(Clone)]
pub struct Reducer<T> {
    pub reduce: fn(T, &Events) -> T,
}

impl<T> Reducer<T> {
    pub const fn new(reduce: fn(T, &Events) -> T) -> Self {
        Self { reduce }
    }
}
