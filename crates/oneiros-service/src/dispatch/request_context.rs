use crate::*;

/// Bundles a request with its effects channel.
///
/// The dispatcher receives the request it needs to process and the
/// channel through which it communicates consequences back to the
/// caller. Single arg, clean signature.
pub struct RequestContext<'a, R> {
    pub request: R,
    pub scope: Box<&'a dyn Scope<'a>>,
}

impl<'a, R> RequestContext<'a, R> {
    pub fn new(request: R, scope: &'a dyn Scope<'a>) -> Self {
        Self {
            request,
            scope: Box::new(scope),
        }
    }
}
