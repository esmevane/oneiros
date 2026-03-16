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
