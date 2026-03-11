mod dispatch;
mod dream_collector;
mod error;
mod state;

pub mod projections;

pub use dispatch::OneirosService;
pub use dream_collector::DreamCollector;
pub use error::*;
pub use projections::{brain, system};
pub use state::ServiceState;
