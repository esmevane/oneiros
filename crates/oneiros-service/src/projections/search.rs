use oneiros_db::*;
use oneiros_model::*;
use serde_json::Value;

/// Search projections emit expressions into the FTS5 index.
///
/// These run alongside brain projections — same events, different target table.
/// The expressions table + FTS5 virtual table enable full-text search across
/// the cognitive stream.
pub const ALL: &[Projection] = &[
    COGNITION_ADDED,
    MEMORY_ADDED,
    EXPERIENCE_CREATED,
    EXPERIENCE_DESCRIPTION_UPDATED,
    AGENT_CREATED,
    AGENT_UPDATED,
    AGENT_REMOVED,
    PERSONA_SET,
    PERSONA_REMOVED,
];

// -- Cognition ----------------------------------------------------------------

const COGNITION_ADDED: Projection = Projection {
    name: "search:cognition-added",
    events: &["cognition-added"],
    apply: apply_cognition_added,
    reset: |db| db.reset_expressions_by_kind("cognition-content"),
};

fn apply_cognition_added(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let cognition: Cognition = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::cognition(cognition.id);

    db.insert_expression(
        &resource_ref,
        "cognition-content",
        cognition.content.as_str(),
    )?;

    Ok(())
}

// -- Memory -------------------------------------------------------------------

const MEMORY_ADDED: Projection = Projection {
    name: "search:memory-added",
    events: &["memory-added"],
    apply: apply_memory_added,
    reset: |db| db.reset_expressions_by_kind("memory-content"),
};

fn apply_memory_added(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let memory: Memory = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::memory(memory.id);

    db.insert_expression(&resource_ref, "memory-content", memory.content.as_str())?;

    Ok(())
}

// -- Experience ---------------------------------------------------------------

const EXPERIENCE_CREATED: Projection = Projection {
    name: "search:experience-created",
    events: &["experience-created"],
    apply: apply_experience_created,
    reset: |db| db.reset_expressions_by_kind("experience-description"),
};

fn apply_experience_created(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let experience: Experience = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::experience(experience.id);

    db.insert_expression(
        &resource_ref,
        "experience-description",
        experience.description.as_str(),
    )?;

    Ok(())
}

const EXPERIENCE_DESCRIPTION_UPDATED: Projection = Projection {
    name: "search:experience-description-updated",
    events: &["experience-description-updated"],
    apply: apply_experience_description_updated,
    reset: |_| Ok(()),
};

#[derive(serde::Deserialize)]
struct DescriptionUpdated {
    experience_id: ExperienceId,
    description: Description,
}

fn apply_experience_description_updated(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let updated: DescriptionUpdated = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::experience(updated.experience_id);

    db.delete_expressions_by_ref(&resource_ref)?;
    db.insert_expression(
        &resource_ref,
        "experience-description",
        updated.description.as_str(),
    )?;

    Ok(())
}

// -- Agent --------------------------------------------------------------------

const AGENT_CREATED: Projection = Projection {
    name: "search:agent-created",
    events: &["agent-created"],
    apply: apply_agent_created,
    reset: |db| {
        db.reset_expressions_by_kind("agent-description")?;
        db.reset_expressions_by_kind("agent-prompt")
    },
};

fn apply_agent_created(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let agent: Agent = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::agent(agent.id);

    db.insert_expression(
        &resource_ref,
        "agent-description",
        agent.description.as_str(),
    )?;
    db.insert_expression(&resource_ref, "agent-prompt", agent.prompt.as_str())?;

    Ok(())
}

const AGENT_UPDATED: Projection = Projection {
    name: "search:agent-updated",
    events: &["agent-updated"],
    apply: apply_agent_updated,
    reset: |_| Ok(()),
};

fn apply_agent_updated(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let agent: Agent = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::agent(agent.id);

    db.delete_expressions_by_ref(&resource_ref)?;
    db.insert_expression(
        &resource_ref,
        "agent-description",
        agent.description.as_str(),
    )?;
    db.insert_expression(&resource_ref, "agent-prompt", agent.prompt.as_str())?;

    Ok(())
}

const AGENT_REMOVED: Projection = Projection {
    name: "search:agent-removed",
    events: &["agent-removed"],
    apply: apply_agent_removed,
    reset: |_| Ok(()),
};

#[derive(serde::Deserialize)]
struct AgentRemoved {
    name: AgentName,
}

fn apply_agent_removed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: AgentRemoved = serde_json::from_value(data.clone())?;

    if let Some(agent) = db.get_agent(&removed.name)? {
        let resource_ref = Ref::agent(agent.id);
        db.delete_expressions_by_ref(&resource_ref)?;
    }

    Ok(())
}

// -- Persona ------------------------------------------------------------------

const PERSONA_SET: Projection = Projection {
    name: "search:persona-set",
    events: &["persona-set"],
    apply: apply_persona_set,
    reset: |db| {
        db.reset_expressions_by_kind("persona-description")?;
        db.reset_expressions_by_kind("persona-prompt")
    },
};

fn apply_persona_set(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let persona: Persona = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::persona(persona.name.clone());

    db.delete_expressions_by_ref(&resource_ref)?;
    db.insert_expression(
        &resource_ref,
        "persona-description",
        persona.description.as_str(),
    )?;
    db.insert_expression(&resource_ref, "persona-prompt", persona.prompt.as_str())?;

    Ok(())
}

const PERSONA_REMOVED: Projection = Projection {
    name: "search:persona-removed",
    events: &["persona-removed"],
    apply: apply_persona_removed,
    reset: |_| Ok(()),
};

#[derive(serde::Deserialize)]
struct PersonaRemoved {
    name: PersonaName,
}

fn apply_persona_removed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: PersonaRemoved = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::persona(removed.name);

    db.delete_expressions_by_ref(&resource_ref)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::projections;
    use oneiros_db::Database;
    use oneiros_model::*;
    use tempfile::TempDir;

    #[test]
    fn export_enriches_storage_set_with_blob_stored() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test-enrich.db");
        let db = Database::create_brain_db(&db_path).unwrap();

        let source = Source::default();

        // Seed a blob directly (bypassing events, as normal workflow does)
        db.put_blob(&BlobContent {
            hash: ContentHash::new("test-hash"),
            data: Blob::encode(b"binary conent here"),
            size: 19.into(),
        })
        .unwrap();

        // Log a storage-set event
        let event = NewEvent::new(
            Events::Storage(StorageEvents::StorageSet(StorageEntry::init(
                StorageKey::new("my-file"),
                "a test file",
                ContentHash::new("test-hash"),
            ))),
            source,
        );
        db.log_event(&event, projections::BRAIN).unwrap();

        // Export should have 2 events: blob-stored then storage-set
        let exported = db.read_events(None).unwrap();
        assert_eq!(exported.len(), 2);

        // First event should be blob-stored (synthetic — no sequence)
        if let Event::New(ref first) = exported[0] {
            let json = serde_json::to_value(&first.data).unwrap();
            assert_eq!(json["type"], "blob-stored");
        } else {
            panic!("expected new (synthetic) event");
        }

        // Second event should be storage-set
        if let Event::Known(ref second) = exported[1] {
            let json = serde_json::to_value(&second.data).unwrap();
            assert_eq!(json["type"], "storage-set");
        } else {
            panic!("expected known event");
        }
    }

    #[test]
    fn export_skips_blob_stored_when_blob_missing() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test-skip.db");
        let db = Database::create_brain_db(&db_path).unwrap();

        let source = Source::default();

        // Log a storage-set event WITHOUT seeding the blob
        let event = NewEvent::new(
            Events::Storage(StorageEvents::StorageSet(StorageEntry::init(
                StorageKey::new("orphaned-file"),
                "missing blob",
                ContentHash::new("nonexistent-hash"),
            ))),
            source,
        );
        db.log_event(&event, projections::BRAIN).unwrap();

        // Export should have just the storage-set, no blob-stored
        let exported = db.read_events(None).unwrap();
        assert_eq!(exported.len(), 1);
    }

    #[test]
    fn blob_stored_projection_writes_blob_and_self_cleans() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test-blob.db");
        let db = Database::create_brain_db(&db_path).unwrap();

        let source = Source::default();
        let blob_data = b"test binary content";
        let blob = Blob::encode(blob_data);
        let hash = ContentHash::new("test-hash-123");

        let event = NewEvent::new(
            Events::Storage(StorageEvents::BlobStored(BlobContent {
                hash: hash.clone(),
                size: blob_data.len().into(),
                data: blob,
            })),
            source,
        );

        db.log_event(&event, projections::BRAIN).unwrap();

        // Blob should be in the blob table
        let blob_content = db.get_blob(&hash).unwrap().expect("blob should exist");

        assert_eq!(blob_content.data, Blob::encode(blob_data));
        assert_eq!(blob_content.size, blob_data.len().into());

        // Event should have been cleaned from the events table
        assert_eq!(db.event_count().unwrap(), 0);
    }

    #[test]
    fn full_export_import_cycle_preserves_blobs() {
        // Source brain: seed blob + storage-set
        let source_temp = TempDir::new().unwrap();
        let source_path = source_temp.path().join("source.db");
        let source_db = Database::create_brain_db(&source_path).unwrap();
        let source = Source::default();

        source_db
            .put_blob(&BlobContent {
                hash: ContentHash::new("cycle-hash"),
                data: Blob::encode(b"portable blob data"),
                size: 18.into(),
            })
            .unwrap();

        let event = NewEvent::new(
            Events::Storage(StorageEvents::StorageSet(StorageEntry::init(
                StorageKey::new("cycle-file"),
                "cycle test",
                ContentHash::new("cycle-hash"),
            ))),
            source,
        );
        source_db.log_event(&event, projections::BRAIN).unwrap();

        // Export from source
        let exported = source_db.read_events(None).unwrap();
        assert_eq!(exported.len(), 2, "should have blob-stored + storage-set");

        // Serialize to JSONL (simulating the wire format)
        let jsonl: Vec<String> = exported
            .iter()
            .map(|e| serde_json::to_string(e).unwrap())
            .collect();

        // Target brain: import from JSONL
        let target_temp = TempDir::new().unwrap();
        let target_path = target_temp.path().join("target.db");
        let target_db = Database::create_brain_db(&target_path).unwrap();

        for line in &jsonl {
            let import: ImportEvent = serde_json::from_str(line).unwrap();
            target_db.import_event(&import).unwrap();
        }
        target_db.replay(projections::BRAIN).unwrap();

        // Blob should exist in target
        let blob_content = target_db
            .get_blob("cycle-hash")
            .unwrap()
            .expect("blob should exist in target");

        assert_eq!(blob_content.data, Blob::encode(b"portable blob data"));
        assert_eq!(blob_content.size, 18.into());

        // Storage entry should resolve
        let storage = target_db
            .get_storage(StorageKey::new("cycle-file"))
            .unwrap();
        assert!(storage.is_some(), "storage entry should exist");

        // blob-stored event should be cleaned from the durable event store.
        // Note: read_events(None) is the export path and synthesizes blob-stored events
        // dynamically on export — it is not the right surface for this check.
        // The durable event count should be 1 (just the storage-set).
        let durable_count = target_db.event_count().unwrap();
        assert_eq!(
            durable_count, 1,
            "only storage-set should remain in the durable event store; blob-stored is transient"
        );
    }

    fn setup_brain() -> (TempDir, Database) {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test-brain.db");
        let db = Database::create_brain_db(&db_path).unwrap();
        (temp, db)
    }

    #[test]
    fn replay_produces_same_expression_count() {
        let (_temp, db) = setup_brain();
        let source = Source::default();

        // Seed prerequisites
        let persona = Persona::init(
            PersonaName::new("test-persona"),
            Description::new("A test persona"),
            Prompt::new("test persona prompt"),
        );
        db.log_event(
            &NewEvent::new(Events::Persona(PersonaEvents::PersonaSet(persona)), source),
            projections::BRAIN,
        )
        .unwrap();

        let texture = Texture::init(
            TextureName::new("observation"),
            Description::new("Observations"),
            Prompt::default(),
        );
        db.log_event(
            &NewEvent::new(Events::Texture(TextureEvents::TextureSet(texture)), source),
            projections::BRAIN,
        )
        .unwrap();

        let level = Level::init(
            LevelName::new("session"),
            Description::new("Session-level"),
            Prompt::default(),
        );
        db.log_event(
            &NewEvent::new(Events::Level(LevelEvents::LevelSet(level)), source),
            projections::BRAIN,
        )
        .unwrap();

        let sensation = Sensation::init(
            SensationName::new("caused"),
            Description::new("Causal connection"),
            Prompt::default(),
        );
        db.log_event(
            &NewEvent::new(
                Events::Sensation(SensationEvents::SensationSet(sensation)),
                source,
            ),
            projections::BRAIN,
        )
        .unwrap();

        // Seed entities
        let agent = Agent::init(
            "test agent description",
            "test agent prompt",
            AgentName::new("test-agent"),
            PersonaName::new("test-persona"),
        );
        db.log_event(
            &NewEvent::new(
                Events::Agent(AgentEvents::AgentCreated(agent.clone())),
                source,
            ),
            projections::BRAIN,
        )
        .unwrap();

        let cognition = Cognition::create(
            agent.id,
            TextureName::new("observation"),
            Content::new("an interesting observation about architecture"),
        );
        db.log_event(
            &NewEvent::new(
                Events::Cognition(CognitionEvents::CognitionAdded(cognition)),
                source,
            ),
            projections::BRAIN,
        )
        .unwrap();

        let memory = Memory::create(
            agent.id,
            LevelName::new("session"),
            Content::new("a consolidated memory about patterns"),
        );
        db.log_event(
            &NewEvent::new(Events::Memory(MemoryEvents::MemoryAdded(memory)), source),
            projections::BRAIN,
        )
        .unwrap();

        let experience = Experience::create(
            agent.id,
            SensationName::new("caused"),
            Description::new("one thought produced another"),
        );
        db.log_event(
            &NewEvent::new(
                Events::Experience(ExperienceEvents::ExperienceCreated(experience)),
                source,
            ),
            projections::BRAIN,
        )
        .unwrap();

        // Count expressions after initial seeding
        let count_before = db.count_expressions().unwrap();
        assert!(count_before > 0, "should have expressions after seeding");

        // Replay all events — resets then re-applies
        db.replay(projections::BRAIN).unwrap();

        // Count again — must be identical
        let count_after = db.count_expressions().unwrap();
        assert_eq!(
            count_before, count_after,
            "replay should produce identical expression count"
        );
    }
}
