use aide::axum::routing::ApiMethodRouter;
use noteworthy::Notation;

use crate::{ResourceDocs, ServerState};

/// HTTP method for a resource route.
#[derive(Clone, Copy)]
pub(crate) enum ResourceMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl ResourceMethod {
    pub(crate) fn router<H, I, O, T, F>(
        self,
        handler: H,
        transform: F,
    ) -> ApiMethodRouter<ServerState>
    where
        H: axum::handler::Handler<T, ServerState> + aide::operation::OperationHandler<I, O>,
        I: aide::operation::OperationInput,
        O: aide::operation::OperationOutput,
        T: 'static,
        F: FnOnce(
            aide::transform::TransformOperation<'_>,
        ) -> aide::transform::TransformOperation<'_>,
    {
        use aide::axum::routing;

        match self {
            Self::Get => routing::get_with(handler, transform),
            Self::Post => routing::post_with(handler, transform),
            Self::Put => routing::put_with(handler, transform),
            Self::Delete => routing::delete_with(handler, transform),
        }
    }
}

/// Function pointer that builds an `ApiMethodRouter` given the resource
/// docs (tag, nickname, summary, description). Type-level info (Response
/// types, status codes, inputs) is baked inside the function body — this
/// is the "function pointers as values" insight.
pub(crate) type RouteBuilder = fn(&ResourceDocs) -> ApiMethodRouter<ServerState>;

/// Route handler notation — carries the HTTP method and a function pointer
/// that builds the `ApiMethodRouter` with all type-level info baked inside.
///
/// The `build` function pointer is where aide's type-level calls
/// (`.response::<200, Json<T>>()`, `.input::<PathParam>()`, etc.) live.
/// Those calls require generic parameters that can't be carried as runtime
/// data — the function pointer captures them at definition site.
///
/// The `build` function receives `&ResourceDocs` for the base transform
/// setup (tag, nickname, summary, description); the variant-specific
/// transform (security requirements, response types) lives inside.
#[derive(Notation)]
pub(crate) struct ResourceHandler {
    #[allow(dead_code)]
    pub method: ResourceMethod,
    pub build: RouteBuilder,
}
