//! Pressure view — re-exports PressurePresenter under the View name.
//!
//! The presenter already owns the full rendering logic. This alias keeps
//! the naming consistent with other domains that expose a `*View` type.

pub(crate) use crate::PressurePresenter as PressureView;
