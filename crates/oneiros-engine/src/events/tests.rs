use crate::*;

#[test]
fn known_event_round_trips_through_event_record() {
    let level: LevelSet = LevelSet::builder_v1()
        .level(
            Level::builder()
                .name("working")
                .description("Short-term")
                .prompt("")
                .build(),
        )
        .build()
        .into();
    let original = Events::Level(LevelEvents::LevelSet(level));

    let json = serde_json::to_string(&original).unwrap();
    let record: Event = serde_json::from_str(&json).unwrap();

    match record {
        Event::Known(Events::Level(LevelEvents::LevelSet(_))) => {}
        other => panic!("expected Known LevelSet, got {other:?}"),
    }
}

#[test]
fn unrecognized_type_tag_becomes_unknown_with_tag_preserved() {
    let raw = r#"{"type": "future-event", "data": {"anything": 42}}"#;
    let record: Event = serde_json::from_str(raw).unwrap();

    match record {
        Event::Unknown(UnknownEvent { type_tag, data }) => {
            assert_eq!(type_tag, "future-event");
            assert_eq!(data["anything"], 42);
        }
        other => panic!("expected Unknown, got {other:?}"),
    }
}

#[test]
fn type_tag_defaults_to_empty_when_missing() {
    let raw = r#"{"type": "flargunnstow", "data": {"nothing": "here"}}"#;
    let record: Event = serde_json::from_str(raw).unwrap();

    match record {
        Event::Unknown(UnknownEvent { type_tag, .. }) => assert_eq!(type_tag, "flargunnstow"),
        other => panic!("expected Unknown, got {other:?}"),
    }
}

#[test]
fn known_event_record_serializes_transparently() {
    let level: LevelSet = LevelSet::builder_v1()
        .level(
            Level::builder()
                .name("working")
                .description("Short-term")
                .prompt("")
                .build(),
        )
        .build()
        .into();
    let record = Event::Known(Events::Level(LevelEvents::LevelSet(level.clone())));
    let record_json = serde_json::to_value(&record).unwrap();

    let direct = Events::Level(LevelEvents::LevelSet(level));
    let direct_json = serde_json::to_value(&direct).unwrap();

    assert_eq!(record_json, direct_json);
}

#[test]
fn unknown_event_record_serializes_raw_data() {
    let data = serde_json::json!({"type": "future-event", "data": {"x": 1}});
    let record = Event::Unknown(UnknownEvent {
        type_tag: "future-event".to_string(),
        data: serde_json::json!({ "x": 1 }),
    });

    assert_eq!(serde_json::to_value(&record).unwrap(), data);
}
