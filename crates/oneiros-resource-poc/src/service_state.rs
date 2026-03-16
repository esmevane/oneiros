use oneiros_db::{Database, Projection};
use oneiros_model::*;
use oneiros_resource::{Fulfill, Resource};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::{ProjectScope, ProjectScopeError};

/// Shared service state that can produce ProjectScopes on demand.
///
/// This is the axum-compatible `Clone + 'static` wrapper around
/// the database. Each request handler locks the mutex and creates
/// a short-lived ProjectScope for the duration of the request.
#[derive(Clone)]
pub struct ServiceState {
    db: Arc<Mutex<Database>>,
    source: Source,
    projections: &'static [&'static [Projection]],
}

impl ServiceState {
    pub fn new(
        db: Database,
        source: Source,
        projections: &'static [&'static [Projection]],
    ) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            source,
            projections,
        }
    }

    pub fn lock_db(&self) -> Result<MutexGuard<'_, Database>, ServiceStateError> {
        self.db.lock().map_err(|_| ServiceStateError::DatabasePoisoned)
    }

    /// Create a ProjectScope from the current state.
    ///
    /// The scope borrows the MutexGuard, so the caller must hold
    /// the guard for the lifetime of the scope.
    pub fn project_scope<'a>(
        &self,
        db: &'a Database,
    ) -> ProjectScope<'a> {
        ProjectScope::new(db, self.source, self.projections)
    }
    /// Fulfill a resource request synchronously.
    ///
    /// Locks the database, creates a ProjectScope, calls fulfill,
    /// and returns the result — all in one shot, no await boundary
    /// crossed while holding the lock.
    ///
    /// This is the bridge between axum's `Send` requirement and
    /// rusqlite's `!Send` reality.
    pub fn fulfill<R>(&self, request: R::Request) -> Result<R::Response, ServiceStateError>
    where
        R: Resource,
        for<'a> ProjectScope<'a>: Fulfill<R, Error = ProjectScopeError>,
    {
        let db = self.lock_db()?;
        let scope = self.project_scope(&db);

        // The fulfill future completes synchronously (wraps rusqlite calls).
        // We poll it once — no runtime needed.
        let future = scope.fulfill(request);
        let result = pollster_block_on(future);

        result.map_err(ServiceStateError::Scope)
    }
}

/// Minimal single-poll block_on for futures that complete immediately.
///
/// Our Fulfill impls wrap synchronous rusqlite calls in `async {}`.
/// The returned future always resolves on first poll. This avoids
/// pulling in a full async runtime for what is fundamentally sync code.
fn pollster_block_on<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = noop_waker();
    let mut cx = std::task::Context::from_waker(&waker);
    match future.as_mut().poll(&mut cx) {
        std::task::Poll::Ready(output) => output,
        std::task::Poll::Pending => unreachable!("fulfill futures complete synchronously"),
    }
}

fn noop_waker() -> std::task::Waker {
    use std::task::{RawWaker, RawWakerVTable};
    const VTABLE: RawWakerVTable = RawWakerVTable::new(|_| RAW, |_| {}, |_| {}, |_| {});
    const RAW: RawWaker = RawWaker::new(std::ptr::null(), &VTABLE);
    // SAFETY: the vtable functions are all no-ops on a null pointer
    unsafe { std::task::Waker::from_raw(RAW) }
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceStateError {
    #[error("Failed to acquire database lock")]
    DatabasePoisoned,

    #[error(transparent)]
    Scope(#[from] ProjectScopeError),
}
