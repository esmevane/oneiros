//! Serde stability tests for protocol event wire format.
//!
//! These tests lock the `"type"` strings produced by each event variant.
//! Moving events between files doesn't affect serialization — these tests
//! catch any accidental renames or serde attribute changes.

#[cfg(test)]
mod tests {
    use crate::*;

    /// Extract the `"type"` string from a serialized tagged enum.
    fn event_type(value: &serde_json::Value) -> &str {
        value.get("type").and_then(|v| v.as_str()).unwrap()
    }

    fn test_timestamp() -> Timestamp {
        Timestamp::now()
    }

    fn test_agent() -> Agent {
        Agent {
            id: AgentId::new(),
            name: AgentName::new("test-agent"),
            persona: PersonaName::new("test-persona"),
            description: Description::default(),
            prompt: Prompt::default(),
        }
    }

    #[test]
    fn tenant_event_type_strings() {
        let event = TenantEvents::TenantCreated(Tenant {
            id: TenantId::new(),
            name: TenantName::new("test"),
        });
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(event_type(&json), "tenant-created");
    }

    #[test]
    fn actor_event_type_strings() {
        let event = ActorEvents::ActorCreated(Actor {
            id: ActorId::new(),
            tenant_id: TenantId::new(),
            name: ActorName::new("test"),
        });
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(event_type(&json), "actor-created");
    }

    #[test]
    fn brain_event_type_strings() {
        let event = BrainEvents::BrainCreated(Brain {
            id: BrainId::new(),
            tenant_id: TenantId::new(),
            name: BrainName::new("test"),
            status: BrainStatus::Active,
            path: std::path::PathBuf::from("/tmp/test"),
        });
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(event_type(&json), "brain-created");
    }

    #[test]
    fn agent_event_type_strings() {
        let agent = test_agent();

        let created = serde_json::to_value(AgentEvents::AgentCreated(agent.clone())).unwrap();
        assert_eq!(event_type(&created), "agent-created");

        let updated = serde_json::to_value(AgentEvents::AgentUpdated(agent)).unwrap();
        assert_eq!(event_type(&updated), "agent-updated");

        let removed = serde_json::to_value(AgentEvents::AgentRemoved(SelectAgentByName {
            name: AgentName::new("test"),
        }))
        .unwrap();
        assert_eq!(event_type(&removed), "agent-removed");
    }

    #[test]
    fn cognition_event_type_strings() {
        let event = CognitionEvents::CognitionAdded(Cognition {
            id: CognitionId::new(),
            agent_id: AgentId::new(),
            texture: TextureName::new("observation"),
            content: Content::new("test"),
            created_at: test_timestamp(),
        });
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(event_type(&json), "cognition-added");
    }

    #[test]
    fn memory_event_type_strings() {
        let event = MemoryEvents::MemoryAdded(Memory {
            id: MemoryId::new(),
            agent_id: AgentId::new(),
            level: LevelName::new("session"),
            content: Content::new("test"),
            created_at: test_timestamp(),
        });
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(event_type(&json), "memory-added");
    }

    #[test]
    fn storage_event_type_strings() {
        let set = serde_json::to_value(StorageEvents::StorageSet(StorageEntry {
            key: StorageKey::new("test-key"),
            description: Description::new("test"),
            hash: ContentHash::new("abc123"),
        }))
        .unwrap();
        assert_eq!(event_type(&set), "storage-set");

        let removed = serde_json::to_value(StorageEvents::StorageRemoved(SelectStorageByKey {
            key: StorageKey::new("test-key"),
        }))
        .unwrap();
        assert_eq!(event_type(&removed), "storage-removed");

        let blob_stored = serde_json::to_value(StorageEvents::BlobStored(BlobContent {
            hash: ContentHash::new("abc123"),
            size: Size::new(42),
            data: Blob::encode(b"test blob data"),
        }))
        .unwrap();
        assert_eq!(event_type(&blob_stored), "blob-stored");
    }

    #[test]
    fn persona_event_type_strings() {
        let set = serde_json::to_value(PersonaEvents::PersonaSet(Persona {
            name: PersonaName::new("test"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .unwrap();
        assert_eq!(event_type(&set), "persona-set");

        let removed = serde_json::to_value(PersonaEvents::PersonaRemoved(SelectPersonaByName {
            name: PersonaName::new("test"),
        }))
        .unwrap();
        assert_eq!(event_type(&removed), "persona-removed");
    }

    #[test]
    fn texture_event_type_strings() {
        let set = serde_json::to_value(TextureEvents::TextureSet(Texture {
            name: TextureName::new("test"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .unwrap();
        assert_eq!(event_type(&set), "texture-set");

        let removed = serde_json::to_value(TextureEvents::TextureRemoved(SelectTextureByName {
            name: TextureName::new("test"),
        }))
        .unwrap();
        assert_eq!(event_type(&removed), "texture-removed");
    }

    #[test]
    fn level_event_type_strings() {
        let set = serde_json::to_value(LevelEvents::LevelSet(Level {
            name: LevelName::new("test"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .unwrap();
        assert_eq!(event_type(&set), "level-set");

        let removed = serde_json::to_value(LevelEvents::LevelRemoved(SelectLevelByName {
            name: LevelName::new("test"),
        }))
        .unwrap();
        assert_eq!(event_type(&removed), "level-removed");
    }

    #[test]
    fn sensation_event_type_strings() {
        let set = serde_json::to_value(SensationEvents::SensationSet(Sensation {
            name: SensationName::new("test"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .unwrap();
        assert_eq!(event_type(&set), "sensation-set");

        let removed =
            serde_json::to_value(SensationEvents::SensationRemoved(SelectSensationByName {
                name: SensationName::new("test"),
            }))
            .unwrap();
        assert_eq!(event_type(&removed), "sensation-removed");
    }

    #[test]
    fn nature_event_type_strings() {
        let set = serde_json::to_value(NatureEvents::NatureSet(Nature {
            name: NatureName::new("test"),
            description: Description::default(),
            prompt: Prompt::default(),
        }))
        .unwrap();
        assert_eq!(event_type(&set), "nature-set");

        let removed = serde_json::to_value(NatureEvents::NatureRemoved(SelectNatureByName {
            name: NatureName::new("test"),
        }))
        .unwrap();
        assert_eq!(event_type(&removed), "nature-removed");
    }

    #[test]
    fn ticket_event_type_strings() {
        let event = TicketEvents::TicketIssued(Ticket {
            id: TicketId::new(),
            token: Token("test-token".into()),
            created_by: ActorId::new(),
        });
        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(event_type(&json), "ticket-issued");
    }

    #[test]
    fn connection_event_type_strings() {
        let created = serde_json::to_value(ConnectionEvents::ConnectionCreated(Connection {
            id: ConnectionId::new(),
            nature: NatureName::new("caused"),
            from_ref: Ref::agent(AgentId::new()),
            to_ref: Ref::cognition(CognitionId::new()),
            created_at: test_timestamp(),
        }))
        .unwrap();
        assert_eq!(event_type(&created), "connection-created");

        let removed =
            serde_json::to_value(ConnectionEvents::ConnectionRemoved(SelectConnectionById {
                id: ConnectionId::new(),
            }))
            .unwrap();
        assert_eq!(event_type(&removed), "connection-removed");
    }

    #[test]
    fn experience_event_type_strings() {
        let created = serde_json::to_value(ExperienceEvents::ExperienceCreated(Experience {
            id: ExperienceId::new(),
            agent_id: AgentId::new(),
            sensation: SensationName::new("echoes"),
            description: Description::new("test"),
            created_at: test_timestamp(),
        }))
        .unwrap();
        assert_eq!(event_type(&created), "experience-created");

        let desc_updated = serde_json::to_value(ExperienceEvents::ExperienceDescriptionUpdated(
            ExperienceDescriptionUpdate {
                experience_id: ExperienceId::new(),
                description: Description::new("updated"),
            },
        ))
        .unwrap();
        assert_eq!(event_type(&desc_updated), "experience-description-updated");

        let sensation_updated = serde_json::to_value(ExperienceEvents::ExperienceSensationUpdated(
            ExperienceSensationUpdate {
                experience_id: ExperienceId::new(),
                sensation: SensationName::new("continues"),
            },
        ))
        .unwrap();
        assert_eq!(
            event_type(&sensation_updated),
            "experience-sensation-updated"
        );
    }

    #[test]
    fn lifecycle_event_type_strings() {
        let woke = serde_json::to_value(LifecycleEvents::Woke(SelectAgentByName {
            name: AgentName::new("test"),
        }))
        .unwrap();
        assert_eq!(event_type(&woke), "woke");

        let slept = serde_json::to_value(LifecycleEvents::Slept(SelectAgentByName {
            name: AgentName::new("test"),
        }))
        .unwrap();
        assert_eq!(event_type(&slept), "slept");

        let emerged = serde_json::to_value(LifecycleEvents::Emerged(SelectAgentByName {
            name: AgentName::new("test"),
        }))
        .unwrap();
        assert_eq!(event_type(&emerged), "emerged");

        let receded = serde_json::to_value(LifecycleEvents::Receded(SelectAgentByName {
            name: AgentName::new("test"),
        }))
        .unwrap();
        assert_eq!(event_type(&receded), "receded");
    }

    #[test]
    fn dreaming_event_type_strings() {
        let begun = serde_json::to_value(DreamingEvents::DreamBegun(SelectAgentByName {
            name: AgentName::new("test"),
        }))
        .unwrap();
        assert_eq!(event_type(&begun), "dream-begun");

        let complete = serde_json::to_value(DreamingEvents::DreamComplete(DreamCompleteEvent {
            agent: test_agent(),
        }))
        .unwrap();
        assert_eq!(event_type(&complete), "dream-complete");
    }

    #[test]
    fn introspecting_event_type_strings() {
        let begun =
            serde_json::to_value(IntrospectingEvents::IntrospectionBegun(SelectAgentByName {
                name: AgentName::new("test"),
            }))
            .unwrap();
        assert_eq!(event_type(&begun), "introspection-begun");

        let complete = serde_json::to_value(IntrospectingEvents::IntrospectionComplete(
            SelectAgentByName {
                name: AgentName::new("test"),
            },
        ))
        .unwrap();
        assert_eq!(event_type(&complete), "introspection-complete");
    }

    #[test]
    fn reflecting_event_type_strings() {
        let begun = serde_json::to_value(ReflectingEvents::ReflectionBegun(SelectAgentByName {
            name: AgentName::new("test"),
        }))
        .unwrap();
        assert_eq!(event_type(&begun), "reflection-begun");

        let complete =
            serde_json::to_value(ReflectingEvents::ReflectionComplete(SelectAgentByName {
                name: AgentName::new("test"),
            }))
            .unwrap();
        assert_eq!(event_type(&complete), "reflection-complete");
    }

    #[test]
    fn sense_event_type_strings() {
        let sensed = serde_json::to_value(SenseEvents::Sensed(SelectAgentByName {
            name: AgentName::new("test"),
        }))
        .unwrap();
        assert_eq!(event_type(&sensed), "sensed");
    }

    #[test]
    fn event_requests_type_strings() {
        let import = serde_json::to_value(EventRequests::ImportEvents(vec![ImportEvent::Valid {
            id: EventId::new(),
            source: Source {
                actor_id: ActorId::new(),
                tenant_id: TenantId::new(),
            },
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            data: serde_json::json!({}),
        }]))
        .unwrap();
        assert_eq!(event_type(&import), "import-events");

        let replay = serde_json::to_value(&EventRequests::ReplayEvents).unwrap();
        assert_eq!(event_type(&replay), "replay-events");

        let list = serde_json::to_value(&EventRequests::ListEvents).unwrap();
        assert_eq!(event_type(&list), "list-events");

        let get = serde_json::to_value(EventRequests::GetEvent(SelectEventById {
            id: EventId::new(),
        }))
        .unwrap();
        assert_eq!(event_type(&get), "get-event");

        let export = serde_json::to_value(&EventRequests::ExportEvents).unwrap();
        assert_eq!(event_type(&export), "export-events");
    }

    #[test]
    fn event_responses_type_strings() {
        let imported = serde_json::to_value(EventResponses::Imported(ImportResponse {
            imported: 5,
            replayed: 3,
        }))
        .unwrap();
        assert_eq!(event_type(&imported), "imported");

        let replayed =
            serde_json::to_value(EventResponses::Replayed(ReplayResponse { replayed: 10 }))
                .unwrap();
        assert_eq!(event_type(&replayed), "replayed");
    }

    #[test]
    fn search_requests_type_strings() {
        let search = serde_json::to_value(SearchRequests::Search(SearchRequest {
            query: "test".to_string(),
            agent: None,
        }))
        .unwrap();
        assert_eq!(event_type(&search), "search");
    }

    #[test]
    fn search_responses_type_strings() {
        let complete = serde_json::to_value(SearchResponses::SearchComplete(SearchResults {
            query: "test".to_string(),
            results: vec![],
        }))
        .unwrap();
        assert_eq!(event_type(&complete), "search-complete");
    }

    // -- Response type string tests --

    fn test_persona() -> Persona {
        Persona {
            name: PersonaName::new("test"),
            description: Description::default(),
            prompt: Prompt::default(),
        }
    }

    fn test_cognition() -> Cognition {
        Cognition {
            id: CognitionId::new(),
            agent_id: AgentId::new(),
            texture: TextureName::new("observation"),
            content: Content::new("test"),
            created_at: test_timestamp(),
        }
    }

    fn test_memory() -> Memory {
        Memory {
            id: MemoryId::new(),
            agent_id: AgentId::new(),
            level: LevelName::new("session"),
            content: Content::new("test"),
            created_at: test_timestamp(),
        }
    }

    fn test_dream_context() -> DreamContext {
        DreamContext {
            agent: test_agent(),
            persona: test_persona(),
            memories: vec![],
            cognitions: vec![],
            experiences: vec![],
            connections: vec![],
            textures: vec![],
            levels: vec![],
            sensations: vec![],
            natures: vec![],
        }
    }

    #[test]
    fn agent_responses_type_strings() {
        let agent = test_agent();

        let created = serde_json::to_value(AgentResponses::AgentCreated(agent.clone())).unwrap();
        assert_eq!(event_type(&created), "agent-created");

        let updated = serde_json::to_value(AgentResponses::AgentUpdated(agent.clone())).unwrap();
        assert_eq!(event_type(&updated), "agent-updated");

        let found = serde_json::to_value(AgentResponses::AgentFound(agent)).unwrap();
        assert_eq!(event_type(&found), "agent-found");

        let listed = serde_json::to_value(AgentResponses::AgentsListed(vec![])).unwrap();
        assert_eq!(event_type(&listed), "agents-listed");

        let removed = serde_json::to_value(&AgentResponses::AgentRemoved).unwrap();
        assert_eq!(event_type(&removed), "agent-removed");
    }

    #[test]
    fn cognition_responses_type_strings() {
        let cog = test_cognition();

        let added = serde_json::to_value(CognitionResponses::CognitionAdded(cog.clone())).unwrap();
        assert_eq!(event_type(&added), "cognition-added");

        let found = serde_json::to_value(CognitionResponses::CognitionFound(cog)).unwrap();
        assert_eq!(event_type(&found), "cognition-found");

        let listed = serde_json::to_value(CognitionResponses::CognitionsListed(vec![])).unwrap();
        assert_eq!(event_type(&listed), "cognitions-listed");
    }

    #[test]
    fn memory_responses_type_strings() {
        let mem = test_memory();

        let added = serde_json::to_value(MemoryResponses::MemoryAdded(mem.clone())).unwrap();
        assert_eq!(event_type(&added), "memory-added");

        let found = serde_json::to_value(MemoryResponses::MemoryFound(mem)).unwrap();
        assert_eq!(event_type(&found), "memory-found");

        let listed = serde_json::to_value(MemoryResponses::MemoriesListed(vec![])).unwrap();
        assert_eq!(event_type(&listed), "memories-listed");
    }

    #[test]
    fn experience_responses_type_strings() {
        let exp = Experience {
            id: ExperienceId::new(),
            agent_id: AgentId::new(),
            sensation: SensationName::new("echoes"),
            description: Description::new("test"),
            created_at: test_timestamp(),
        };

        let created =
            serde_json::to_value(ExperienceResponses::ExperienceCreated(exp.clone())).unwrap();
        assert_eq!(event_type(&created), "experience-created");

        let updated =
            serde_json::to_value(ExperienceResponses::ExperienceUpdated(exp.clone())).unwrap();
        assert_eq!(event_type(&updated), "experience-updated");

        let found = serde_json::to_value(ExperienceResponses::ExperienceFound(exp)).unwrap();
        assert_eq!(event_type(&found), "experience-found");

        let listed = serde_json::to_value(ExperienceResponses::ExperiencesListed(vec![])).unwrap();
        assert_eq!(event_type(&listed), "experiences-listed");
    }

    #[test]
    fn connection_responses_type_strings() {
        let conn = Connection {
            id: ConnectionId::new(),
            nature: NatureName::new("caused"),
            from_ref: Ref::agent(AgentId::new()),
            to_ref: Ref::cognition(CognitionId::new()),
            created_at: test_timestamp(),
        };

        let created =
            serde_json::to_value(ConnectionResponses::ConnectionCreated(conn.clone())).unwrap();
        assert_eq!(event_type(&created), "connection-created");

        let found = serde_json::to_value(ConnectionResponses::ConnectionFound(conn)).unwrap();
        assert_eq!(event_type(&found), "connection-found");

        let listed = serde_json::to_value(ConnectionResponses::ConnectionsListed(vec![])).unwrap();
        assert_eq!(event_type(&listed), "connections-listed");

        let removed = serde_json::to_value(&ConnectionResponses::ConnectionRemoved).unwrap();
        assert_eq!(event_type(&removed), "connection-removed");
    }

    #[test]
    fn storage_responses_type_strings() {
        let entry = StorageEntry {
            key: StorageKey::new("test-key"),
            description: Description::new("test"),
            hash: ContentHash::new("abc123"),
        };

        let set = serde_json::to_value(StorageResponses::StorageSet(entry.clone())).unwrap();
        assert_eq!(event_type(&set), "storage-set");

        let found = serde_json::to_value(StorageResponses::StorageFound(entry)).unwrap();
        assert_eq!(event_type(&found), "storage-found");

        let listed = serde_json::to_value(StorageResponses::StorageListed(vec![])).unwrap();
        assert_eq!(event_type(&listed), "storage-listed");

        let removed = serde_json::to_value(&StorageResponses::StorageRemoved).unwrap();
        assert_eq!(event_type(&removed), "storage-removed");
    }

    #[test]
    fn persona_responses_type_strings() {
        let p = test_persona();

        let set = serde_json::to_value(PersonaResponses::PersonaSet(p.clone())).unwrap();
        assert_eq!(event_type(&set), "persona-set");

        let found = serde_json::to_value(PersonaResponses::PersonaFound(p)).unwrap();
        assert_eq!(event_type(&found), "persona-found");

        let listed = serde_json::to_value(PersonaResponses::PersonasListed(vec![])).unwrap();
        assert_eq!(event_type(&listed), "personas-listed");

        let removed = serde_json::to_value(&PersonaResponses::PersonaRemoved).unwrap();
        assert_eq!(event_type(&removed), "persona-removed");
    }

    #[test]
    fn texture_responses_type_strings() {
        let t = Texture {
            name: TextureName::new("test"),
            description: Description::default(),
            prompt: Prompt::default(),
        };

        let set = serde_json::to_value(TextureResponses::TextureSet(t.clone())).unwrap();
        assert_eq!(event_type(&set), "texture-set");

        let found = serde_json::to_value(TextureResponses::TextureFound(t)).unwrap();
        assert_eq!(event_type(&found), "texture-found");

        let listed = serde_json::to_value(TextureResponses::TexturesListed(vec![])).unwrap();
        assert_eq!(event_type(&listed), "textures-listed");

        let removed = serde_json::to_value(&TextureResponses::TextureRemoved).unwrap();
        assert_eq!(event_type(&removed), "texture-removed");
    }

    #[test]
    fn level_responses_type_strings() {
        let l = Level {
            name: LevelName::new("test"),
            description: Description::default(),
            prompt: Prompt::default(),
        };

        let set = serde_json::to_value(LevelResponses::LevelSet(l.clone())).unwrap();
        assert_eq!(event_type(&set), "level-set");

        let found = serde_json::to_value(LevelResponses::LevelFound(l)).unwrap();
        assert_eq!(event_type(&found), "level-found");

        let listed = serde_json::to_value(LevelResponses::LevelsListed(vec![])).unwrap();
        assert_eq!(event_type(&listed), "levels-listed");

        let removed = serde_json::to_value(&LevelResponses::LevelRemoved).unwrap();
        assert_eq!(event_type(&removed), "level-removed");
    }

    #[test]
    fn sensation_responses_type_strings() {
        let s = Sensation {
            name: SensationName::new("test"),
            description: Description::default(),
            prompt: Prompt::default(),
        };

        let set = serde_json::to_value(SensationResponses::SensationSet(s.clone())).unwrap();
        assert_eq!(event_type(&set), "sensation-set");

        let found = serde_json::to_value(SensationResponses::SensationFound(s)).unwrap();
        assert_eq!(event_type(&found), "sensation-found");

        let listed = serde_json::to_value(SensationResponses::SensationsListed(vec![])).unwrap();
        assert_eq!(event_type(&listed), "sensations-listed");

        let removed = serde_json::to_value(&SensationResponses::SensationRemoved).unwrap();
        assert_eq!(event_type(&removed), "sensation-removed");
    }

    #[test]
    fn nature_responses_type_strings() {
        let n = Nature {
            name: NatureName::new("test"),
            description: Description::default(),
            prompt: Prompt::default(),
        };

        let set = serde_json::to_value(NatureResponses::NatureSet(n.clone())).unwrap();
        assert_eq!(event_type(&set), "nature-set");

        let found = serde_json::to_value(NatureResponses::NatureFound(n)).unwrap();
        assert_eq!(event_type(&found), "nature-found");

        let listed = serde_json::to_value(NatureResponses::NaturesListed(vec![])).unwrap();
        assert_eq!(event_type(&listed), "natures-listed");

        let removed = serde_json::to_value(&NatureResponses::NatureRemoved).unwrap();
        assert_eq!(event_type(&removed), "nature-removed");
    }

    #[test]
    fn brain_responses_type_strings() {
        let info = BrainInfo {
            entity: BrainId::new(),
            token: Token("test-token".into()),
        };
        let created = serde_json::to_value(BrainResponses::BrainCreated(info)).unwrap();
        assert_eq!(event_type(&created), "brain-created");

        let brain = Brain {
            id: BrainId::new(),
            tenant_id: TenantId::new(),
            name: BrainName::new("test"),
            status: BrainStatus::Active,
            path: std::path::PathBuf::from("/tmp/test"),
        };
        let found = serde_json::to_value(BrainResponses::BrainFound(brain)).unwrap();
        assert_eq!(event_type(&found), "brain-found");

        let listed = serde_json::to_value(BrainResponses::BrainsListed(vec![])).unwrap();
        assert_eq!(event_type(&listed), "brains-listed");
    }

    #[test]
    fn lifecycle_responses_type_strings() {
        let woke =
            serde_json::to_value(LifecycleResponses::Woke(Box::new(test_dream_context()))).unwrap();
        assert_eq!(event_type(&woke), "woke");

        let slept = serde_json::to_value(LifecycleResponses::Slept(test_agent())).unwrap();
        assert_eq!(event_type(&slept), "slept");

        let emerged = serde_json::to_value(LifecycleResponses::Emerged(test_agent())).unwrap();
        assert_eq!(event_type(&emerged), "emerged");

        let receded = serde_json::to_value(&LifecycleResponses::Receded).unwrap();
        assert_eq!(event_type(&receded), "receded");
    }

    #[test]
    fn dreaming_responses_type_strings() {
        let complete = serde_json::to_value(DreamingResponses::DreamComplete(Box::new(
            test_dream_context(),
        )))
        .unwrap();
        assert_eq!(event_type(&complete), "dream-complete");
    }

    #[test]
    fn introspecting_responses_type_strings() {
        let complete =
            serde_json::to_value(IntrospectingResponses::IntrospectionComplete(test_agent()))
                .unwrap();
        assert_eq!(event_type(&complete), "introspection-complete");
    }

    #[test]
    fn reflecting_responses_type_strings() {
        let complete =
            serde_json::to_value(ReflectingResponses::ReflectionComplete(test_agent())).unwrap();
        assert_eq!(event_type(&complete), "reflection-complete");
    }

    #[test]
    fn sense_responses_type_strings() {
        let sensed = serde_json::to_value(SenseResponses::Sensed(test_agent())).unwrap();
        assert_eq!(event_type(&sensed), "sensed");
    }

    // -- Request filter param tests --

    #[test]
    fn cognition_requests_list_type_string() {
        let list = serde_json::to_value(CognitionRequests::ListCognitions(ListCognitionsFilter {
            agent: None,
            texture: None,
        }))
        .unwrap();
        assert_eq!(event_type(&list), "list-cognitions");

        // With filters — type string unchanged
        let filtered =
            serde_json::to_value(CognitionRequests::ListCognitions(ListCognitionsFilter {
                agent: Some(AgentName::new("test")),
                texture: Some(TextureName::new("observation")),
            }))
            .unwrap();
        assert_eq!(event_type(&filtered), "list-cognitions");
    }

    #[test]
    fn memory_requests_list_type_string() {
        let list = serde_json::to_value(MemoryRequests::ListMemories(ListMemoriesFilter {
            agent: None,
            level: None,
        }))
        .unwrap();
        assert_eq!(event_type(&list), "list-memories");
    }

    #[test]
    fn experience_requests_list_type_string() {
        let list =
            serde_json::to_value(ExperienceRequests::ListExperiences(ListExperiencesFilter {
                agent: None,
                sensation: None,
            }))
            .unwrap();
        assert_eq!(event_type(&list), "list-experiences");
    }

    #[test]
    fn connection_requests_list_type_string() {
        let list =
            serde_json::to_value(ConnectionRequests::ListConnections(ListConnectionsFilter {
                nature: None,
                entity_ref: None,
            }))
            .unwrap();
        assert_eq!(event_type(&list), "list-connections");
    }

    #[test]
    fn responses_super_enum_roundtrips() {
        let agent_response = AgentResponses::AgentCreated(test_agent());
        let json = serde_json::to_string(&agent_response).unwrap();
        let roundtripped: Responses = serde_json::from_str(&json).unwrap();
        assert!(matches!(
            roundtripped,
            Responses::Agent(AgentResponses::AgentCreated(_))
        ));
    }

    #[test]
    fn trust_event_type_strings() {
        let mode_configured = serde_json::to_value(TrustEvents::TrustModeConfigured(
            TrustModeConfigured {
                mode: "local".to_string(),
            },
        ))
        .unwrap();
        assert_eq!(event_type(&mode_configured), "trust-mode-configured");

        let ca_initialized = serde_json::to_value(TrustEvents::TrustCaInitialized(
            TrustCaInitialized {
                root_fingerprint: "sha256:abc123".to_string(),
                root_storage_key: None,
            },
        ))
        .unwrap();
        assert_eq!(event_type(&ca_initialized), "trust-ca-initialized");

        let leaf_issued =
            serde_json::to_value(TrustEvents::TrustLeafIssued(TrustLeafIssued {
                hostname: "localhost".to_string(),
                not_after: "2026-01-01T00:00:00Z".to_string(),
            }))
            .unwrap();
        assert_eq!(event_type(&leaf_issued), "trust-leaf-issued");

        let store_installed = serde_json::to_value(TrustEvents::TrustStoreInstalled).unwrap();
        assert_eq!(event_type(&store_installed), "trust-store-installed");

        let store_failed = serde_json::to_value(TrustEvents::TrustStoreInstallFailed(
            TrustStoreInstallFailed {
                reason: "permission denied".to_string(),
            },
        ))
        .unwrap();
        assert_eq!(event_type(&store_failed), "trust-store-install-failed");

        let peer_accepted =
            serde_json::to_value(TrustEvents::TrustPeerAccepted(TrustPeerAccepted {
                endpoint: "https://peer.example.com".to_string(),
                fingerprint: "sha256:def456".to_string(),
            }))
            .unwrap();
        assert_eq!(event_type(&peer_accepted), "trust-peer-accepted");

        let fingerprint_changed = serde_json::to_value(TrustEvents::TrustPeerFingerprintChanged(
            TrustPeerFingerprintChanged {
                endpoint: "https://peer.example.com".to_string(),
                old_fingerprint: "sha256:old".to_string(),
                new_fingerprint: "sha256:new".to_string(),
            },
        ))
        .unwrap();
        assert_eq!(
            event_type(&fingerprint_changed),
            "trust-peer-fingerprint-changed"
        );

        let insecure_allowed = serde_json::to_value(TrustEvents::TrustPeerInsecureAllowed(
            TrustPeerInsecureAllowed {
                endpoint: "http://dev.local".to_string(),
                reason: Some("local dev".to_string()),
            },
        ))
        .unwrap();
        assert_eq!(event_type(&insecure_allowed), "trust-peer-insecure-allowed");
    }

    #[test]
    fn events_super_enum_roundtrips() {
        // Verify that Events (untagged) can deserialize what its inner enums serialize.
        let agent_event = AgentEvents::AgentCreated(test_agent());
        let json = serde_json::to_string(&agent_event).unwrap();
        let roundtripped: Events = serde_json::from_str(&json).unwrap();
        assert!(matches!(
            roundtripped,
            Events::Agent(AgentEvents::AgentCreated(_))
        ));
    }
}
