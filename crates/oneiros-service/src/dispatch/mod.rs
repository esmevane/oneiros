mod brain;
mod effects;
mod request_context;
mod system;

pub use effects::*;
pub use oneiros_service::*;
pub use request_context::*;

mod oneiros_service;

/// The atomic dispatch unit: one domain, one backend.
///
/// Implementors declare what a request means (what response to produce,
/// what effects to emit). The caller provides the effects channel.
///
/// Rhymes with `Future` (the runtime provides Waker) and `tower::Service`
/// (the caller provides the runtime). The pattern is causal reciprocity:
/// the implementor declares intent, the environment fulfills it.
pub trait Dispatch<R> {
    type Response;
    type Error;

    fn dispatch(&self, envelope: RequestContext<'_, R>) -> Result<Self::Response, Self::Error>;
}
