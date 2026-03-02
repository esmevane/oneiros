use axum::extract::State;
use axum::response::sse::{Event, KeepAlive, Sse};
use std::convert::Infallible;
use std::sync::Arc;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

use crate::state::ServiceState;

pub(crate) async fn handler(
    State(state): State<Arc<ServiceState>>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let rx = state.event_tx.subscribe();

    let stream = BroadcastStream::new(rx).filter_map(|result| match result {
        Ok(event) => {
            let json = serde_json::to_string(&event).unwrap_or_default();
            Some(Ok(Event::default().data(json)))
        }
        Err(_) => None,
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}
