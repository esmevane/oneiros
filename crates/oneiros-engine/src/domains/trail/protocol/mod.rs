mod errors;
mod requests;
mod responses;

pub(crate) use errors::*;
pub(crate) use requests::*;
pub(crate) use responses::*;

// Trail emits no events — it's a projection over every other domain's
// events. The derivation rule lives in the projection's `apply`. There is
// no `TrailEvents`.
