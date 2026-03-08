mod common;
use common::*;

#[tokio::test]
async fn cursor_catchup_flow() {
    let (_temp, state, token) = setup();

    ensure_persona(
        &state,
        &token,
        "process",
        "Process agent",
        "You are a process agent.",
    )
    .await;

    seed_agent(&state, &token, "agent-a", "process").await;
    seed_agent(&state, &token, "agent-b", "process").await;

    // Read all events to get sequences
    let app = router(state.clone());
    let response = app.oneshot(get_auth("/events", &token)).await.unwrap();
    let all_events: Vec<serde_json::Value> = body_bytes(response).await;
    assert!(all_events.len() >= 2, "should have multiple events");

    // Verify all events have sequence numbers
    for event in &all_events {
        assert!(
            event.get("sequence").is_some(),
            "event should have sequence field: {event}"
        );
    }

    // Verify sequences are monotonically increasing
    let sequences: Vec<u64> = all_events
        .iter()
        .filter_map(|e| e["sequence"].as_u64())
        .collect();
    for window in sequences.windows(2) {
        assert!(
            window[1] > window[0],
            "sequences should be increasing: {} -> {}",
            window[0],
            window[1]
        );
    }

    // Use cursor to get subset
    let midpoint = sequences[sequences.len() / 2];
    let app = router(state.clone());
    let uri = format!("/events?after={midpoint}");
    let response = app.oneshot(get_auth(&uri, &token)).await.unwrap();
    let later_events: Vec<serde_json::Value> = body_bytes(response).await;

    // All returned events should have sequence > midpoint
    for event in &later_events {
        let seq = event["sequence"].as_u64().unwrap();
        assert!(
            seq > midpoint,
            "event sequence {seq} should be > {midpoint}"
        );
    }

    assert!(
        later_events.len() < all_events.len(),
        "cursor query should return fewer events than total"
    );
}

#[tokio::test]
async fn events_index_returns_all_events() {
    let (_temp, state, token) = setup();

    ensure_persona(
        &state,
        &token,
        "process",
        "A process agent",
        "You are a process agent.",
    )
    .await;

    let app = router(state.clone());
    let response = app.oneshot(get_auth("/events", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let events: Vec<serde_json::Value> = body_bytes(response).await;
    assert!(!events.is_empty(), "should have events from seeding");
}

#[tokio::test]
async fn events_index_with_after_filters_by_sequence() {
    let (_temp, state, token) = setup();

    ensure_persona(
        &state,
        &token,
        "process",
        "A process agent",
        "You are a process agent.",
    )
    .await;

    seed_agent(&state, &token, "agent-1", "process").await;
    seed_agent(&state, &token, "agent-2", "process").await;

    // Get all events to find a sequence to filter on
    let app = router(state.clone());
    let response = app.oneshot(get_auth("/events", &token)).await.unwrap();
    let all_events: Vec<serde_json::Value> = body_bytes(response).await;
    let total = all_events.len();
    assert!(total >= 2);

    // Get the sequence of the first event
    let first_sequence = all_events[0]["sequence"].as_u64().unwrap();

    // Query with after= first sequence
    let app = router(state.clone());
    let uri = format!("/events?after={first_sequence}");
    let response = app.oneshot(get_auth(&uri, &token)).await.unwrap();
    let filtered: Vec<serde_json::Value> = body_bytes(response).await;

    assert_eq!(filtered.len(), total - 1, "should exclude the first event");
}
