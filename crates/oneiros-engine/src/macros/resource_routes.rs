/// Generate `http_handler()` for an operations struct.
///
/// # Syntax
///
/// ```ignore
/// resource_routes! {
///     RequestType::Variant => |this| { routing::post_with(handler, |op| { resource_op!(op, this).response::<200, Json<ResponseType>>() }) },
///     RequestType::Variant => |this| { routing::get_with(handler, |op| { resource_op!(op, this).input::<PathParam>().response::<200, Json<ResponseType>>() }) },
/// };
/// ```
///
/// Each arm receives `this: &Self` and must return an `ApiMethodRouter<ServerState>`.
macro_rules! resource_routes {
    ($($pat:path => |$this:ident| $body:expr),* $(,)?) => {
        fn http_handler(&self) -> aide::axum::routing::ApiMethodRouter<ServerState> {
            match self.kind {
                $($pat => {
                    let $this = self;
                    $body
                }),*
            }
        }
    };
}

pub(crate) use resource_routes;
