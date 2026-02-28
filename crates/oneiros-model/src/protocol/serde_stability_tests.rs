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

        let removed = serde_json::to_value(&AgentEvents::AgentRemoved {
            name: AgentName::new("test"),
        })
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

        let removed = serde_json::to_value(&StorageEvents::StorageRemoved {
            key: StorageKey::new("test-key"),
        })
        .unwrap();
        assert_eq!(event_type(&removed), "storage-removed");
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

        let removed = serde_json::to_value(&PersonaEvents::PersonaRemoved {
            name: PersonaName::new("test"),
        })
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

        let removed = serde_json::to_value(&TextureEvents::TextureRemoved {
            name: TextureName::new("test"),
        })
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

        let removed = serde_json::to_value(&LevelEvents::LevelRemoved {
            name: LevelName::new("test"),
        })
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

        let removed = serde_json::to_value(&SensationEvents::SensationRemoved {
            name: SensationName::new("test"),
        })
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

        let removed = serde_json::to_value(&NatureEvents::NatureRemoved {
            name: NatureName::new("test"),
        })
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

        let removed = serde_json::to_value(&ConnectionEvents::ConnectionRemoved {
            id: ConnectionId::new(),
        })
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
            refs: vec![],
            created_at: test_timestamp(),
        }))
        .unwrap();
        assert_eq!(event_type(&created), "experience-created");

        let ref_added = serde_json::to_value(&ExperienceEvents::ExperienceRefAdded {
            experience_id: ExperienceId::new(),
            experience_ref: ExperienceRef::new(Ref::cognition(CognitionId::new()), None),
            created_at: test_timestamp(),
        })
        .unwrap();
        assert_eq!(event_type(&ref_added), "experience-ref-added");

        let desc_updated = serde_json::to_value(&ExperienceEvents::ExperienceDescriptionUpdated {
            experience_id: ExperienceId::new(),
            description: Description::new("updated"),
        })
        .unwrap();
        assert_eq!(event_type(&desc_updated), "experience-description-updated");
    }

    #[test]
    fn lifecycle_event_type_strings() {
        let woke = serde_json::to_value(&LifecycleEvents::Woke {
            name: AgentName::new("test"),
        })
        .unwrap();
        assert_eq!(event_type(&woke), "woke");

        let slept = serde_json::to_value(&LifecycleEvents::Slept {
            name: AgentName::new("test"),
        })
        .unwrap();
        assert_eq!(event_type(&slept), "slept");

        let emerged = serde_json::to_value(&LifecycleEvents::Emerged {
            name: AgentName::new("test"),
        })
        .unwrap();
        assert_eq!(event_type(&emerged), "emerged");

        let receded = serde_json::to_value(&LifecycleEvents::Receded {
            name: AgentName::new("test"),
        })
        .unwrap();
        assert_eq!(event_type(&receded), "receded");
    }

    #[test]
    fn dreaming_event_type_strings() {
        let begun = serde_json::to_value(&DreamingEvents::DreamBegun {
            agent: AgentName::new("test"),
        })
        .unwrap();
        assert_eq!(event_type(&begun), "dream-begun");

        let complete = serde_json::to_value(&DreamingEvents::DreamComplete {
            agent: test_agent(),
        })
        .unwrap();
        assert_eq!(event_type(&complete), "dream-complete");
    }

    #[test]
    fn introspecting_event_type_strings() {
        let begun = serde_json::to_value(&IntrospectingEvents::IntrospectionBegun {
            agent: AgentName::new("test"),
        })
        .unwrap();
        assert_eq!(event_type(&begun), "introspection-begun");

        let complete = serde_json::to_value(&IntrospectingEvents::IntrospectionComplete {
            agent: AgentName::new("test"),
        })
        .unwrap();
        assert_eq!(event_type(&complete), "introspection-complete");
    }

    #[test]
    fn reflecting_event_type_strings() {
        let begun = serde_json::to_value(&ReflectingEvents::ReflectionBegun {
            agent: AgentName::new("test"),
        })
        .unwrap();
        assert_eq!(event_type(&begun), "reflection-begun");

        let complete = serde_json::to_value(&ReflectingEvents::ReflectionComplete {
            agent: AgentName::new("test"),
        })
        .unwrap();
        assert_eq!(event_type(&complete), "reflection-complete");
    }

    #[test]
    fn sense_event_type_strings() {
        let sensed = serde_json::to_value(&SenseEvents::Sensed {
            agent: AgentName::new("test"),
        })
        .unwrap();
        assert_eq!(event_type(&sensed), "sensed");
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
