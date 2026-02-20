use oneiros_db::EventRow;
use oneiros_model::{Link, LinkError};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum RewriteError {
    #[error("Missing field '{0}' in event data")]
    MissingField(&'static str),

    #[error("Failed to compute link: {0}")]
    Link(#[from] LinkError),
}

/// Rewrite a brain's event log from UUID-based IDs to content-addressed links.
///
/// Processes events in chronological order. For each creation event, computes
/// the entity's content-addressed link and builds a mapping from old UUID → new
/// link string. Cross-reference fields (like `agent_id` in cognitions) are
/// replaced using the mapping.
///
/// Events that don't carry UUID-based IDs (name-keyed vocabularies, lifecycle
/// events, prompt events) pass through unchanged.
pub fn rewrite_events(events: Vec<EventRow>) -> Result<Vec<Value>, RewriteError> {
    let mut id_map: HashMap<String, String> = HashMap::new();
    let mut rewritten = Vec::new();

    for event in events {
        let mut data = event.data;
        let event_type = data["type"].as_str().unwrap_or("").to_string();

        match event_type.as_str() {
            // Creation events — compute link, map old ID, replace cross-refs.
            "agent-created" => rewrite_agent(&mut data, &mut id_map)?,
            "agent-updated" => rewrite_agent(&mut data, &mut id_map)?,
            "cognition-added" => rewrite_cognition(&mut data, &mut id_map)?,
            "memory-added" => rewrite_memory(&mut data, &mut id_map)?,
            "experience-created" => rewrite_experience(&mut data, &mut id_map)?,
            "connection-created" => rewrite_connection(&mut data, &mut id_map)?,

            // Cross-reference events — replace mapped UUID fields.
            "experience-ref-added" => {
                rewrite_experience_ref_added(&mut data, &mut id_map, &event.timestamp)?
            }
            "experience-description-updated" => {
                map_field(&mut data, &["data", "experience_id"], &id_map);
            }
            "connection-removed" => {
                map_field(&mut data, &["data", "id"], &id_map);
            }

            // Everything else passes through unchanged:
            // - Name-keyed set/removed events (persona, texture, level, sensation, nature, storage)
            // - Lifecycle events (woke, slept, emerged, receded)
            // - Prompt events (dream-begun/complete, introspection-*, reflection-*, sensed)
            // - agent-removed (name-keyed)
            _ => {}
        }

        rewritten.push(data);
    }

    Ok(rewritten)
}

/// Rewrite an agent-created or agent-updated event.
///
/// Computes a link from (name, persona), maps the old UUID, and replaces the
/// id field.
fn rewrite_agent(
    data: &mut Value,
    id_map: &mut HashMap<String, String>,
) -> Result<(), RewriteError> {
    let inner = &data["data"];
    let name = inner["name"]
        .as_str()
        .ok_or(RewriteError::MissingField("name"))?;
    let persona = inner["persona"]
        .as_str()
        .ok_or(RewriteError::MissingField("persona"))?;

    let link = Link::new(&("agent", name, persona))?;
    let link_str = link.to_string();

    if let Some(old_id) = inner["id"].as_str() {
        id_map.insert(old_id.to_string(), link_str.clone());
    }

    data["data"]["id"] = Value::String(link_str);
    Ok(())
}

/// Rewrite a cognition-added event.
///
/// Computes a link from (texture, content), maps the old UUID, replaces id
/// and agent_id.
fn rewrite_cognition(
    data: &mut Value,
    id_map: &mut HashMap<String, String>,
) -> Result<(), RewriteError> {
    let inner = &data["data"];
    let texture = inner["texture"]
        .as_str()
        .ok_or(RewriteError::MissingField("texture"))?;
    let content = inner["content"]
        .as_str()
        .ok_or(RewriteError::MissingField("content"))?;

    let link = Link::new(&("cognition", texture, content))?;
    let link_str = link.to_string();

    if let Some(old_id) = inner["id"].as_str() {
        id_map.insert(old_id.to_string(), link_str.clone());
    }

    data["data"]["id"] = Value::String(link_str);
    map_field(data, &["data", "agent_id"], id_map);
    Ok(())
}

/// Rewrite a memory-added event.
///
/// Computes a link from (level, content), maps the old UUID, replaces id
/// and agent_id.
fn rewrite_memory(
    data: &mut Value,
    id_map: &mut HashMap<String, String>,
) -> Result<(), RewriteError> {
    let inner = &data["data"];
    let level = inner["level"]
        .as_str()
        .ok_or(RewriteError::MissingField("level"))?;
    let content = inner["content"]
        .as_str()
        .ok_or(RewriteError::MissingField("content"))?;

    let link = Link::new(&("memory", level, content))?;
    let link_str = link.to_string();

    if let Some(old_id) = inner["id"].as_str() {
        id_map.insert(old_id.to_string(), link_str.clone());
    }

    data["data"]["id"] = Value::String(link_str);
    map_field(data, &["data", "agent_id"], id_map);
    Ok(())
}

/// Rewrite an experience-created event.
///
/// Computes a link from (sensation, description), maps the old UUID, replaces
/// id and agent_id.
fn rewrite_experience(
    data: &mut Value,
    id_map: &mut HashMap<String, String>,
) -> Result<(), RewriteError> {
    let inner = &data["data"];
    let sensation = inner["sensation"]
        .as_str()
        .ok_or(RewriteError::MissingField("sensation"))?;
    let description = inner["description"]
        .as_str()
        .ok_or(RewriteError::MissingField("description"))?;

    let link = Link::new(&("experience", sensation, description))?;
    let link_str = link.to_string();

    if let Some(old_id) = inner["id"].as_str() {
        id_map.insert(old_id.to_string(), link_str.clone());
    }

    data["data"]["id"] = Value::String(link_str);
    map_field(data, &["data", "agent_id"], id_map);
    Ok(())
}

/// Rewrite a connection-created event.
///
/// Computes a link from (nature, from_link, to_link), maps the old UUID,
/// replaces id.
fn rewrite_connection(
    data: &mut Value,
    id_map: &mut HashMap<String, String>,
) -> Result<(), RewriteError> {
    let inner = &data["data"];
    let nature = inner["nature"]
        .as_str()
        .ok_or(RewriteError::MissingField("nature"))?;
    let from_link_str = inner["from_link"]
        .as_str()
        .ok_or(RewriteError::MissingField("from_link"))?;
    let to_link_str = inner["to_link"]
        .as_str()
        .ok_or(RewriteError::MissingField("to_link"))?;

    // Parse the link strings back to Link objects so postcard serializes them
    // identically to the Addressable impl (which passes &Link, whose Serialize
    // outputs the base64url string).
    let from_link: Link = from_link_str
        .parse()
        .map_err(|e: LinkError| RewriteError::Link(e))?;
    let to_link: Link = to_link_str
        .parse()
        .map_err(|e: LinkError| RewriteError::Link(e))?;

    let link = Link::new(&("connection", nature, from_link, to_link))?;
    let link_str = link.to_string();

    if let Some(old_id) = inner["id"].as_str() {
        id_map.insert(old_id.to_string(), link_str.clone());
    }

    data["data"]["id"] = Value::String(link_str);
    Ok(())
}

/// Rewrite an experience-ref-added event.
///
/// Maps experience_id, maps the record_ref's id (if IdentifiedRef variant),
/// and backfills created_at from the event row's timestamp when absent.
fn rewrite_experience_ref_added(
    data: &mut Value,
    id_map: &mut HashMap<String, String>,
    event_timestamp: &str,
) -> Result<(), RewriteError> {
    // Map the experience_id.
    map_field(data, &["data", "experience_id"], id_map);

    // Map the record_ref's id if it's an IdentifiedRef (has "id" field).
    // LinkedRef variants (with "link" field) are already content-addressed.
    if data["data"]["record_ref"]["id"].is_string() {
        map_field(data, &["data", "record_ref", "id"], id_map);
    }

    // Backfill created_at from event timestamp when absent.
    if data["data"]["created_at"].is_null() {
        data["data"]["created_at"] = Value::String(event_timestamp.to_string());
    }

    Ok(())
}

/// Replace a string field at a nested path with its mapped value from id_map.
///
/// If the field doesn't exist, isn't a string, or isn't in the map, it's left
/// unchanged. This is intentionally best-effort — unmapped IDs pass through.
fn map_field(data: &mut Value, path: &[&str], id_map: &HashMap<String, String>) {
    // Navigate to the parent, then replace the leaf.
    let (leaf, parents) = match path.split_last() {
        Some(pair) => pair,
        None => return,
    };

    let mut current = &mut *data;
    for key in parents {
        current = &mut current[*key];
    }

    if let Some(old_val) = current[*leaf].as_str()
        && let Some(new_val) = id_map.get(old_val)
    {
        current[*leaf] = Value::String(new_val.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_event(id: &str, timestamp: &str, data: Value) -> EventRow {
        EventRow {
            id: id.to_string(),
            timestamp: timestamp.to_string(),
            data,
        }
    }

    #[test]
    fn agent_created_gets_content_addressed_id() {
        let events = vec![make_event(
            "001",
            "2025-01-01T00:00:00Z",
            json!({
                "type": "agent-created",
                "data": {
                    "id": "019c5ea2-ba84-7cf1-8113-3db9b418c82c",
                    "name": "architect",
                    "persona": "expert",
                    "description": "The system architect",
                    "prompt": "Design things."
                }
            }),
        )];

        let rewritten = rewrite_events(events).unwrap();
        let new_id = rewritten[0]["data"]["id"].as_str().unwrap();

        // Should no longer be the original UUID.
        assert_ne!(new_id, "019c5ea2-ba84-7cf1-8113-3db9b418c82c");

        // Should be deterministic: same name+persona → same link.
        let expected_link = Link::new(&("agent", "architect", "expert"))
            .unwrap()
            .to_string();
        assert_eq!(new_id, expected_link);
    }

    #[test]
    fn cognition_agent_id_is_mapped() {
        let events = vec![
            make_event(
                "001",
                "2025-01-01T00:00:00Z",
                json!({
                    "type": "agent-created",
                    "data": {
                        "id": "agent-uuid-1",
                        "name": "architect",
                        "persona": "expert",
                        "description": "",
                        "prompt": ""
                    }
                }),
            ),
            make_event(
                "002",
                "2025-01-01T00:01:00Z",
                json!({
                    "type": "cognition-added",
                    "data": {
                        "id": "cognition-uuid-1",
                        "agent_id": "agent-uuid-1",
                        "texture": "observation",
                        "content": "Something interesting.",
                        "created_at": "2025-01-01T00:01:00Z"
                    }
                }),
            ),
        ];

        let rewritten = rewrite_events(events).unwrap();

        // Agent's id should be its link.
        let agent_link = Link::new(&("agent", "architect", "expert"))
            .unwrap()
            .to_string();
        assert_eq!(rewritten[0]["data"]["id"].as_str().unwrap(), agent_link);

        // Cognition's agent_id should be mapped to the agent's link.
        assert_eq!(
            rewritten[1]["data"]["agent_id"].as_str().unwrap(),
            agent_link
        );

        // Cognition's own id should be its content link.
        let cognition_link = Link::new(&("cognition", "observation", "Something interesting."))
            .unwrap()
            .to_string();
        assert_eq!(rewritten[1]["data"]["id"].as_str().unwrap(), cognition_link);
    }

    #[test]
    fn experience_ref_added_maps_ids_and_backfills_timestamp() {
        let exp_link = Link::new(&("experience", "echoes", "A resonance"))
            .unwrap()
            .to_string();
        let cog_link = Link::new(&("cognition", "observation", "First thought"))
            .unwrap()
            .to_string();

        let events = vec![
            make_event(
                "001",
                "2025-01-01T00:00:00Z",
                json!({
                    "type": "agent-created",
                    "data": {
                        "id": "agent-uuid-1",
                        "name": "thinker",
                        "persona": "process",
                        "description": "",
                        "prompt": ""
                    }
                }),
            ),
            make_event(
                "002",
                "2025-01-01T00:01:00Z",
                json!({
                    "type": "cognition-added",
                    "data": {
                        "id": "cog-uuid-1",
                        "agent_id": "agent-uuid-1",
                        "texture": "observation",
                        "content": "First thought",
                        "created_at": "2025-01-01T00:01:00Z"
                    }
                }),
            ),
            make_event(
                "003",
                "2025-01-01T00:02:00Z",
                json!({
                    "type": "experience-created",
                    "data": {
                        "id": "exp-uuid-1",
                        "agent_id": "agent-uuid-1",
                        "sensation": "echoes",
                        "description": "A resonance",
                        "refs": [],
                        "created_at": "2025-01-01T00:02:00Z"
                    }
                }),
            ),
            // Legacy ref-added without created_at
            make_event(
                "004",
                "2025-01-01T00:03:00Z",
                json!({
                    "type": "experience-ref-added",
                    "data": {
                        "experience_id": "exp-uuid-1",
                        "record_ref": {
                            "id": "cog-uuid-1",
                            "kind": "cognition",
                            "role": "origin"
                        }
                    }
                }),
            ),
        ];

        let rewritten = rewrite_events(events).unwrap();
        let ref_event = &rewritten[3];

        // experience_id should be mapped.
        assert_eq!(
            ref_event["data"]["experience_id"].as_str().unwrap(),
            exp_link
        );

        // record_ref.id should be mapped.
        assert_eq!(
            ref_event["data"]["record_ref"]["id"].as_str().unwrap(),
            cog_link
        );

        // created_at should be backfilled from event timestamp.
        assert_eq!(
            ref_event["data"]["created_at"].as_str().unwrap(),
            "2025-01-01T00:03:00Z"
        );
    }

    #[test]
    fn name_keyed_events_pass_through() {
        let original = json!({
            "type": "texture-set",
            "data": {
                "name": "observation",
                "description": "Noticing things.",
                "prompt": "Be observant."
            }
        });

        let events = vec![make_event("001", "2025-01-01T00:00:00Z", original.clone())];
        let rewritten = rewrite_events(events).unwrap();
        assert_eq!(rewritten[0], original);
    }

    #[test]
    fn lifecycle_events_pass_through() {
        let original = json!({
            "type": "woke",
            "data": { "name": "governor.process" }
        });

        let events = vec![make_event("001", "2025-01-01T00:00:00Z", original.clone())];
        let rewritten = rewrite_events(events).unwrap();
        assert_eq!(rewritten[0], original);
    }

    #[test]
    fn connection_created_computes_link() {
        let from_link = Link::new(&("agent", "alpha", "expert")).unwrap();
        let to_link = Link::new(&("agent", "beta", "expert")).unwrap();

        let events = vec![make_event(
            "001",
            "2025-01-01T00:00:00Z",
            json!({
                "type": "connection-created",
                "data": {
                    "id": "conn-uuid-1",
                    "nature": "origin",
                    "from_link": from_link.to_string(),
                    "to_link": to_link.to_string(),
                    "created_at": "2025-01-01T00:00:00Z"
                }
            }),
        )];

        let rewritten = rewrite_events(events).unwrap();
        let new_id = rewritten[0]["data"]["id"].as_str().unwrap();

        let expected = Link::new(&("connection", "origin", from_link, to_link))
            .unwrap()
            .to_string();
        assert_eq!(new_id, expected);
    }

    #[test]
    fn connection_removed_maps_id() {
        let events = vec![
            make_event(
                "001",
                "2025-01-01T00:00:00Z",
                json!({
                    "type": "connection-created",
                    "data": {
                        "id": "conn-uuid-1",
                        "nature": "origin",
                        "from_link": "AAAA",
                        "to_link": "BBBB",
                        "created_at": "2025-01-01T00:00:00Z"
                    }
                }),
            ),
            make_event(
                "002",
                "2025-01-01T00:01:00Z",
                json!({
                    "type": "connection-removed",
                    "data": { "id": "conn-uuid-1" }
                }),
            ),
        ];

        let rewritten = rewrite_events(events).unwrap();

        // The removal event's id should be mapped to the same link as the creation.
        let created_id = rewritten[0]["data"]["id"].as_str().unwrap();
        let removed_id = rewritten[1]["data"]["id"].as_str().unwrap();
        assert_eq!(created_id, removed_id);
    }

    #[test]
    fn unmapped_ids_pass_through() {
        let events = vec![make_event(
            "001",
            "2025-01-01T00:00:00Z",
            json!({
                "type": "experience-description-updated",
                "data": {
                    "experience_id": "unknown-uuid",
                    "description": "Updated text"
                }
            }),
        )];

        let rewritten = rewrite_events(events).unwrap();
        // Unmapped UUID stays as-is.
        assert_eq!(
            rewritten[0]["data"]["experience_id"].as_str().unwrap(),
            "unknown-uuid"
        );
    }

    #[test]
    fn rewrite_is_deterministic() {
        let make = || {
            vec![make_event(
                "001",
                "2025-01-01T00:00:00Z",
                json!({
                    "type": "agent-created",
                    "data": {
                        "id": "some-uuid",
                        "name": "observer",
                        "persona": "process",
                        "description": "",
                        "prompt": ""
                    }
                }),
            )]
        };

        let run1 = rewrite_events(make()).unwrap();
        let run2 = rewrite_events(make()).unwrap();
        assert_eq!(run1, run2);
    }

    #[test]
    fn experience_ref_added_preserves_existing_created_at() {
        let events = vec![make_event(
            "001",
            "2025-01-01T00:03:00Z",
            json!({
                "type": "experience-ref-added",
                "data": {
                    "experience_id": "exp-uuid",
                    "record_ref": {
                        "link": "AAAA",
                        "role": "origin"
                    },
                    "created_at": "2025-01-01T00:01:00Z"
                }
            }),
        )];

        let rewritten = rewrite_events(events).unwrap();
        // Should keep the original created_at, NOT overwrite with event timestamp.
        assert_eq!(
            rewritten[0]["data"]["created_at"].as_str().unwrap(),
            "2025-01-01T00:01:00Z"
        );
    }

    #[test]
    fn linked_refs_are_not_touched() {
        let events = vec![make_event(
            "001",
            "2025-01-01T00:00:00Z",
            json!({
                "type": "experience-ref-added",
                "data": {
                    "experience_id": "exp-uuid",
                    "record_ref": {
                        "link": "SOME_LINK_VALUE",
                        "role": "origin"
                    },
                    "created_at": "2025-01-01T00:00:00Z"
                }
            }),
        )];

        let rewritten = rewrite_events(events).unwrap();
        // LinkedRef should pass through unchanged.
        assert_eq!(
            rewritten[0]["data"]["record_ref"]["link"].as_str().unwrap(),
            "SOME_LINK_VALUE"
        );
    }
}
