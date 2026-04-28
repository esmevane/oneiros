//! Bridge protocol — peer-to-peer chronicle sync over iroh.
//!
//! These shapes carry over the `/oneiros/sync/1` ALPN via QUIC. They are
//! the wire format for the Merkle diff exchange between two peers that
//! share a bookmark.
//!
//! # TODO: this module is intentionally exempt from `versioned!`
//!
//! Every other protocol module in oneiros wraps its shapes in the
//! `versioned!` macro (see `agent/protocol/`). Bridge does not. The
//! exemption is deliberate — and worth revisiting when any of the
//! conditions below change.
//!
//! ## Why bridge isn't versioned today
//!
//! `versioned!`'s inline form hardcodes a `JsonSchema` derive on every
//! V_n struct. Bridge requests/responses carry `LedgerNode` (HAMT
//! internals from `values/ledger.rs`) and `StoredEvent` (the canonical
//! persisted-event envelope from `domains/event/model.rs`) directly.
//! Wrapping bridge with `versioned!` would force `JsonSchema` onto:
//!
//! - `LedgerNode` and `BTreeMap<u8, ContentHash>` (HAMT shape)
//! - `StoredEvent`, `Event`, `Events`, every `*Events` enum, and every
//!   V_n payload struct in every domain
//!
//! That's ~50+ types acquiring a derive that nothing reads. The
//! derive's purpose is to publish shapes in OpenAPI for API consumers;
//! the bridge wire isn't an API surface — it's machine-to-machine sync.
//!
//! ## What bridge actually transports
//!
//! Bridge does not create or consume domain events. It moves
//! already-canonical `StoredEvent`s between peers. `BridgeEvents.events`
//! carries items that already passed through the domain → `NewEvent` →
//! `EventLog::append` pipeline on the source peer. The destination
//! imports them via `EventLog::import` (preserves id, sequence,
//! source, timestamp) and replays locally. The "events" flowing through
//! bridge are the same events as everywhere else — just in motion.
//!
//! ## How bridge versions today
//!
//! Protocol-level, not shape-level: the ALPN identifier
//! `/oneiros/sync/1` is the version. If the protocol shape changes,
//! the path is to introduce `/oneiros/sync/2` and run both during
//! transition. That's how QUIC application protocols typically
//! evolve.
//!
//! ## Open threads worth preserving
//!
//! - **Bridge as part of "our interface"**: even though it isn't an
//!   API consumer surface, it *is* a contract between peers. If we
//!   ever want shape-level evolvability (add a field to
//!   `BridgeFetchEvents`, change `BridgeNodes` pagination), the
//!   shape-level pattern would help.
//!
//! - **Ephemeral framing**: bridge requests/responses *could* be
//!   modeled as `Events::Ephemeral` events (RPCs-as-events), which
//!   would make them consistent with the rest of the codebase's
//!   event-driven shape. They aren't today because they're synchronous
//!   query/response, not state-change facts. The framing is worth
//!   considering if cross-peer telemetry or auditing becomes a need.
//!
//! - **A `versioned!` variant without `JsonSchema`**: the right path
//!   to fold bridge back in is making the macro's schemars derive
//!   optional (e.g., `versioned_no_schema!` or an attribute to opt
//!   out). Then bridge could use `versioned!` without dragging
//!   `JsonSchema` across the chronicle/event infrastructure.
//!
//! ## When to revisit
//!
//! Take this exemption back out when:
//! 1. We need to evolve a bridge shape and want shape-level versioning
//!    rather than ALPN-level versioning, OR
//! 2. We extend `versioned!` with an optional-schema variant, OR
//! 3. The chronicle/event infrastructure gains `JsonSchema` for some
//!    other reason and the cascade cost disappears.

mod errors;
mod requests;
mod responses;

pub use errors::*;
pub use requests::*;
pub use responses::*;
