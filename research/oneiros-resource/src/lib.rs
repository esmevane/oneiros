use std::future::Future;

/// A domain resource in oneiros.
///
/// Each resource declares its identity and message types. Scope participation
/// is expressed through which `Fulfill` impls exist — if
/// `impl Fulfill<Agent> for ProjectScope` exists, Agent is project-scoped.
pub trait Resource {
    /// Human-readable name — used for routing, logging, error messages.
    const NAME: &'static str;

    /// The request type this resource handles.
    type Request;

    /// The response type this resource produces.
    type Response;
}

/// A scope that can fulfill requests for resource R.
///
/// Inspired by `tower::Service` — abstracts the fulfillment contract,
/// not the mechanism. Each scope provides its own async strategy internally.
///
/// Uses RPITIT (`-> impl Future`) rather than `async fn` so implementations
/// can choose whether the returned future is `Send` or not. Scopes backed
/// by `!Send` resources (rusqlite) return `!Send` futures. Scopes backed
/// by async resources (HTTP, filesystem) return `Send` futures.
pub trait Fulfill<R: Resource> {
    type Error;

    fn fulfill(
        &self,
        request: R::Request,
    ) -> impl Future<Output = Result<R::Response, Self::Error>>;
}

// ── Layers ──────────────────────────────────────────────────────────
//
// Layer markers are zero-sized types that parameterize Feature<L>.
// Each represents a transport or presentation surface a resource
// can opt into.

/// HTTP handler surface — produces an axum Router or equivalent.
pub struct Server;

/// MCP tool surface — produces tool definitions and handles tool calls.
pub struct Tools;

/// CLI command surface — produces clap commands and handles execution.
pub struct Ops;

/// Remote client surface — fulfills by making HTTP calls.
pub struct Client;

/// Read model maintenance — produces projection definitions.
pub struct Projections;

/// A layer surface that a resource can provide.
///
/// `Feature<L>` is the outward-facing counterpart to `Fulfill<R>`:
/// - `Fulfill<R>` says "this scope can handle resource R" (inward)
/// - `Feature<L>` says "this resource presents as layer L" (outward)
///
/// Opt-in is natural: if a resource does not impl `Feature<Tools>`,
/// it has no MCP tools. The absence of the impl is the opt-out.
pub trait Feature<L> {
    /// What this feature produces — Router, Vec<Tool>, Command, etc.
    type Surface;

    /// Produce the layer surface.
    fn feature(&self) -> Self::Surface;
}

/// A resource (or middleware) that can install itself into an app.
///
/// Inspired by Bevy's `Plugin::build` pattern. Separate from `Resource`
/// because not everything that mounts is a resource (middleware,
/// cross-cutting concerns), and not every resource needs to mount
/// (internal/derived resources).
/// Extension trait that provides `self.feature::<L>()` turbofish syntax.
///
/// Without this, calling `self.feature()` on a type that implements
/// multiple `Feature<L>` impls requires UFCS:
///   `<Agent as Feature<Server>>::feature(&self)`
///
/// With this, you can write:
///   `self.feature::<Server>()`
pub trait HasFeature {
    fn feature<L>(&self) -> <Self as Feature<L>>::Surface
    where
        Self: Feature<L>;
}

impl<T> HasFeature for T {
    fn feature<L>(&self) -> <Self as Feature<L>>::Surface
    where
        Self: Feature<L>,
    {
        <Self as Feature<L>>::feature(self)
    }
}

pub trait Mountable<App> {
    fn mount(&self, app: &mut App);
}
