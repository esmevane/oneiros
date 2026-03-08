use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::sse::{Event as SseEvent, KeepAlive, Sse};
use std::convert::Infallible;
use std::sync::Arc;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

use oneiros_service::ServiceState;

pub(crate) async fn handler(
    State(state): State<Arc<ServiceState>>,
    headers: HeaderMap,
) -> Sse<impl tokio_stream::Stream<Item = Result<SseEvent, Infallible>>> {
    let last_event_id: Option<u64> = headers
        .get("last-event-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok());

    let rx = state.subscribe();

    let mut last_sent = last_event_id.unwrap_or(0);

    let stream = BroadcastStream::new(rx).filter_map(move |result| match result {
        Ok(event) => {
            let sequence = match &event {
                oneiros_model::Event::Known(k) => k.sequence,
                oneiros_model::Event::Unknown(u) => u.sequence,
                oneiros_model::Event::New(_) => return None,
            };

            if sequence <= last_sent {
                return None;
            }
            last_sent = sequence;

            let json = serde_json::to_string(&event).unwrap_or_default();
            Some(Ok(SseEvent::default().id(sequence.to_string()).data(json)))
        }
        Err(_) => None,
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}
