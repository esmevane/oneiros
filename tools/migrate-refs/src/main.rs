use rusqlite::{Connection, params};
use serde_json::{Map, Value, json};

/// Migrate experience ref fields in brain event data to current format.
///
/// Old events store refs as `{id, kind, role}` objects with a `record_ref` key.
/// Current code expects `{entity: Ref, role}` objects with an `experience_ref` key,
/// where Ref is `{"V0": {"Kind": "id"}}`.
///
/// Affected event types and conversions:
///   experience-ref-added:  record_ref → experience_ref, {id,kind} → {entity: Ref},
///                          adds created_at from event timestamp if missing
///   experience-created:    refs[*] from {id,kind,role} → {entity: Ref, role}
///
/// Connection-created events are not affected — they were created after the Ref
/// refactor and already use structural JSON.
fn kind_to_resource_variant(kind: &str) -> Option<&str> {
    match kind {
        "agent" => Some("Agent"),
        "actor" => Some("Actor"),
        "brain" => Some("Brain"),
        "cognition" => Some("Cognition"),
        "connection" => Some("Connection"),
        "experience" => Some("Experience"),
        "memory" => Some("Memory"),
        "storage" => Some("Storage"),
        _ => None,
    }
}

/// Convert `{"id": "019c...", "kind": "cognition", "role": "origin"}`
/// into `{"entity": {"V0": {"Cognition": "019c..."}}, "role": "origin"}`.
fn convert_old_ref(obj: &Map<String, Value>) -> Option<Value> {
    let id = obj.get("id")?.as_str()?;
    let kind = obj.get("kind")?.as_str()?;
    let variant = kind_to_resource_variant(kind)?;

    let mut new_ref = serde_json::Map::new();
    new_ref.insert("entity".to_string(), json!({"V0": {variant: id}}));

    if let Some(role) = obj.get("role") {
        new_ref.insert("role".to_string(), role.clone());
    }

    Some(Value::Object(new_ref))
}

struct Stats {
    scanned: usize,
    rewritten: usize,
    refs_converted: usize,
    errors: usize,
}

fn migrate_experience_created(data: &mut Value, stats: &mut Stats) -> bool {
    let refs = match data.get_mut("refs").and_then(Value::as_array_mut) {
        Some(r) => r,
        None => return false,
    };

    let mut changed = false;

    // Collect converted refs, then replace in place.
    for r in refs.iter_mut() {
        // Only convert old-format refs (those with "id" and "kind" fields,
        // but no "entity" field).
        if let Some(obj) = r.as_object()
            && obj.contains_key("id")
            && obj.contains_key("kind")
            && !obj.contains_key("entity")
        {
            if let Some(converted) = convert_old_ref(obj) {
                *r = converted;
                stats.refs_converted += 1;
                changed = true;
            } else {
                eprintln!("  WARNING: could not convert ref: {r}");
                stats.errors += 1;
            }
        }
    }

    changed
}

fn migrate_experience_ref_added(
    data: &mut Value,
    event_timestamp: &str,
    stats: &mut Stats,
) -> bool {
    let obj = match data.as_object_mut() {
        Some(o) => o,
        None => return false,
    };

    let mut changed = false;

    // Rename record_ref → experience_ref and convert the inner format.
    if let Some(record_ref) = obj.remove("record_ref") {
        if let Some(record_ref_obj) = record_ref.as_object() {
            if let Some(converted) = convert_old_ref(record_ref_obj) {
                obj.insert("experience_ref".to_string(), converted);
                stats.refs_converted += 1;
                changed = true;
            } else {
                eprintln!("  WARNING: could not convert record_ref: {record_ref}");
                // Put it back so we don't lose data.
                obj.insert("record_ref".to_string(), record_ref);
                stats.errors += 1;
            }
        } else {
            eprintln!("  WARNING: record_ref is not an object: {record_ref}");
            obj.insert("record_ref".to_string(), record_ref);
            stats.errors += 1;
        }
    }

    // Add created_at from event timestamp if missing.
    if !obj.contains_key("created_at") {
        obj.insert(
            "created_at".to_string(),
            Value::String(event_timestamp.to_string()),
        );
        changed = true;
    }

    changed
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let db_path = args.get(1).expect("Usage: migrate-refs <brain.db>");

    let conn = Connection::open(db_path).expect("Failed to open database");

    // Read all events that might contain refs, including the event timestamp
    // for backfilling created_at on old experience-ref-added events.
    let mut stmt = conn
        .prepare(
            "SELECT id, timestamp, data FROM events \
             WHERE json_extract(data, '$.type') IN \
             ('experience-created', 'experience-ref-added')",
        )
        .expect("Failed to prepare query");

    let events: Vec<(String, String, String)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .expect("Failed to query events")
        .collect::<Result<_, _>>()
        .expect("Failed to collect events");

    let mut stats = Stats {
        scanned: events.len(),
        rewritten: 0,
        refs_converted: 0,
        errors: 0,
    };

    eprintln!("Found {} events to check", stats.scanned);

    for (id, timestamp, data_str) in &events {
        let mut data: Value = match serde_json::from_str(data_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!(
                    "  Skipping event {}: malformed JSON: {e}",
                    &id[..8.min(id.len())]
                );
                stats.errors += 1;
                continue;
            }
        };

        let event_type = data["type"].as_str().unwrap_or("").to_string();
        let inner = match data.get_mut("data") {
            Some(d) => d,
            None => continue,
        };

        let changed = match event_type.as_str() {
            "experience-created" => migrate_experience_created(inner, &mut stats),
            "experience-ref-added" => migrate_experience_ref_added(inner, timestamp, &mut stats),
            _ => false,
        };

        if changed {
            let new_data = serde_json::to_string(&data).expect("Failed to serialize");

            conn.execute(
                "UPDATE events SET data = ?1 WHERE id = ?2",
                params![new_data, id],
            )
            .expect("Failed to update event");

            stats.rewritten += 1;
        }
    }

    eprintln!(
        "Migration complete: {} scanned, {} rewritten, {} refs converted, {} errors",
        stats.scanned, stats.rewritten, stats.refs_converted, stats.errors
    );

    if stats.errors > 0 {
        std::process::exit(1);
    }
}
