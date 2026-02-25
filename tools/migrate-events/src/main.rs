use oneiros_db::Database;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use uuid::Uuid;

fn is_uuid(s: &str) -> bool {
    s.len() == 36
        && s.as_bytes().get(8) == Some(&b'-')
        && s.as_bytes().get(13) == Some(&b'-')
        && s.as_bytes().get(18) == Some(&b'-')
        && s.as_bytes().get(23) == Some(&b'-')
}

fn map_id(mapping: &mut HashMap<String, String>, old: &str) -> String {
    if is_uuid(old) {
        mapping
            .entry(old.to_string())
            .or_insert_with(|| old.to_string());
        old.to_string()
    } else {
        mapping
            .entry(old.to_string())
            .or_insert_with(|| Uuid::now_v7().to_string())
            .clone()
    }
}

fn lookup_id(mapping: &HashMap<String, String>, old: &str) -> String {
    if is_uuid(old) {
        old.to_string()
    } else {
        mapping.get(old).cloned().unwrap_or_else(|| {
            eprintln!("WARNING: unmapped reference: {}", &old[..old.len().min(40)]);
            Uuid::now_v7().to_string()
        })
    }
}

fn migrate_agent(inner: &mut Map<String, Value>, mapping: &mut HashMap<String, String>) {
    if let Some(Value::String(old_id)) = inner.get("id") {
        let new_id = map_id(mapping, old_id);
        inner.insert("id".to_string(), Value::String(new_id));
    }
}

fn migrate_cognition(inner: &mut Map<String, Value>, mapping: &mut HashMap<String, String>) {
    if let Some(Value::String(old_id)) = inner.get("id") {
        let new_id = map_id(mapping, old_id);
        inner.insert("id".to_string(), Value::String(new_id));
    }

    if let Some(Value::String(old_agent)) = inner.get("agent_id") {
        let new_agent = lookup_id(mapping, old_agent);
        inner.insert("agent_id".to_string(), Value::String(new_agent));
    }
}

fn migrate_memory(inner: &mut Map<String, Value>, mapping: &mut HashMap<String, String>) {
    migrate_cognition(inner, mapping);
}

fn migrate_experience_created(
    inner: &mut Map<String, Value>,
    mapping: &mut HashMap<String, String>,
) {
    if let Some(Value::String(old_id)) = inner.get("id") {
        let new_id = map_id(mapping, old_id);
        inner.insert("id".to_string(), Value::String(new_id));
    }

    if let Some(Value::String(old_agent)) = inner.get("agent_id") {
        let new_agent = lookup_id(mapping, old_agent);
        inner.insert("agent_id".to_string(), Value::String(new_agent));
    }

    if let Some(Value::Array(refs)) = inner.get_mut("refs") {
        for r in refs.iter_mut() {
            if let Value::Object(ref_obj) = r
                && let Some(Value::String(ref_id)) = ref_obj.get("id")
            {
                let new_ref_id = lookup_id(mapping, ref_id);
                ref_obj.insert("id".to_string(), Value::String(new_ref_id));
            }
        }
    }
}

fn migrate_experience_ref_added(
    inner: &mut Map<String, Value>,
    mapping: &mut HashMap<String, String>,
) {
    if let Some(Value::String(old_eid)) = inner.get("experience_id") {
        let new_eid = lookup_id(mapping, old_eid);
        inner.insert("experience_id".to_string(), Value::String(new_eid));
    }

    inner.remove("created_at");

    if let Some(Value::Object(ref_obj)) = inner.get_mut("record_ref")
        && let Some(Value::String(ref_id)) = ref_obj.get("id")
    {
        let new_ref_id = lookup_id(mapping, ref_id);
        ref_obj.insert("id".to_string(), Value::String(new_ref_id));
    }
}

struct MigratedEvent {
    timestamp: String,
    data: Value,
}

fn migrate_line(
    line: &str,
    line_num: usize,
    mapping: &mut HashMap<String, String>,
    stats: &mut MigrationStats,
) -> MigratedEvent {
    let mut event: Value = serde_json::from_str(line).unwrap_or_else(|e| {
        panic!("Line {}: invalid JSON: {}", line_num + 1, e);
    });

    let timestamp = event
        .get("timestamp")
        .and_then(Value::as_str)
        .expect("event missing timestamp")
        .to_string();

    let event_type = event
        .pointer("/data/type")
        .and_then(Value::as_str)
        .map(String::from);

    let modified = if let Some(ref etype) = event_type {
        if let Some(inner) = event
            .pointer_mut("/data/data")
            .and_then(Value::as_object_mut)
        {
            match etype.as_str() {
                "agent-created" | "agent-updated" => {
                    migrate_agent(inner, mapping);
                    stats.rewritten += 1;
                    true
                }
                "cognition-added" => {
                    migrate_cognition(inner, mapping);
                    stats.rewritten += 1;
                    true
                }
                "memory-added" => {
                    migrate_memory(inner, mapping);
                    stats.rewritten += 1;
                    true
                }
                "experience-created" => {
                    migrate_experience_created(inner, mapping);
                    stats.rewritten += 1;
                    true
                }
                "experience-ref-added" => {
                    migrate_experience_ref_added(inner, mapping);
                    stats.rewritten += 1;
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    } else {
        false
    };

    if !modified {
        stats.passthrough += 1;
    }

    stats.total += 1;

    let data = event.get("data").cloned().unwrap_or(Value::Null);

    MigratedEvent { timestamp, data }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut input_path = None;
    let mut db_path = None;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--db" => {
                db_path = Some(args.get(i + 1).expect("--db requires a path").clone());
                i += 2;
            }
            _ => {
                input_path = Some(args[i].clone());
                i += 1;
            }
        }
    }

    let input_path = input_path.expect("Usage: migrate-events <input.jsonl> [--db <brain.db>]");

    let file = std::fs::File::open(&input_path).expect("Failed to open input file");
    let reader = io::BufReader::new(file);

    let mut mapping: HashMap<String, String> = HashMap::new();
    let mut stats = MigrationStats::default();

    if let Some(db_path) = db_path {
        let db = Database::open_brain(&db_path)
            .unwrap_or_else(|e| panic!("Failed to open brain DB at {db_path}: {e}"));

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.expect("Failed to read line");
            if line.trim().is_empty() {
                continue;
            }

            let migrated = migrate_line(&line, line_num, &mut mapping, &mut stats);

            db.import_event(&migrated.timestamp, &migrated.data)
                .unwrap_or_else(|e| panic!("Line {}: failed to import: {e}", line_num + 1));
        }

        eprintln!(
            "Import complete: {} total, {} rewritten, {} passthrough, {} IDs mapped",
            stats.total,
            stats.rewritten,
            stats.passthrough,
            mapping.len()
        );
        eprintln!("Events inserted into {db_path}");
    } else {
        let stdout = io::stdout();
        let mut out = io::BufWriter::new(stdout.lock());

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.expect("Failed to read line");
            if line.trim().is_empty() {
                continue;
            }

            let migrated = migrate_line(&line, line_num, &mut mapping, &mut stats);

            let envelope = serde_json::json!({
                "timestamp": migrated.timestamp,
                "data": migrated.data,
            });

            serde_json::to_writer(&mut out, &envelope).expect("Failed to write event");
            out.write_all(b"\n").expect("Failed to write newline");
        }

        eprintln!(
            "Migration complete: {} total, {} rewritten, {} passthrough, {} IDs mapped",
            stats.total,
            stats.rewritten,
            stats.passthrough,
            mapping.len()
        );
    }
}

#[derive(Default)]
struct MigrationStats {
    total: usize,
    rewritten: usize,
    passthrough: usize,
}
