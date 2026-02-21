mod dream;
mod guidebook;
mod introspect;
mod reflect;
mod sense;

pub use dream::DreamTemplate;
pub use guidebook::GuidebookTemplate;
pub use introspect::IntrospectTemplate;
pub use reflect::ReflectTemplate;
pub use sense::SenseTemplate;

#[cfg(test)]
mod tests {
    use super::*;
    use oneiros_model::*;

    fn test_agent() -> AgentRecord {
        AgentRecord::init(
            "A curious explorer agent",
            "You explore and discover.",
            Agent {
                name: AgentName::new("atlas"),
                persona: PersonaName::new("explorer"),
            },
        )
    }

    fn test_persona() -> PersonaRecord {
        PersonaRecord::init(
            "An explorer persona.",
            "Explore with curiosity.",
            Persona {
                name: PersonaName::new("explorer"),
            },
        )
    }

    #[test]
    fn dream_template_renders_identity_and_persona() {
        let context = DreamContext {
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
        };
        let rendered = DreamTemplate::new(&context).to_string();

        assert!(rendered.contains("You are waking as atlas."));
        assert!(rendered.contains("## Your Identity"));
        assert!(rendered.contains("A curious explorer agent"));
        assert!(rendered.contains("You explore and discover."));
        assert!(rendered.contains("## Your Persona"));
        assert!(rendered.contains("An explorer persona."));
        assert!(rendered.contains("## Agent Definition"));
        assert!(rendered.contains(".claude/agents/atlas.md"));
        assert!(rendered.contains(".agents/atlas.md"));
        assert!(rendered.contains("## Instructions"));
    }

    #[test]
    fn dream_template_omits_empty_sections() {
        let context = DreamContext {
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
        };
        let rendered = DreamTemplate::new(&context).to_string();

        assert!(!rendered.contains("## Your Memories"));
        assert!(!rendered.contains("## Your Cognitions"));
        assert!(!rendered.contains("## Your Connections"));
        assert!(!rendered.contains("## Cognitive Textures"));
        assert!(!rendered.contains("## Memory Levels"));
    }

    #[test]
    fn dream_template_renders_memories() {
        let context = DreamContext {
            agent: test_agent(),
            persona: test_persona(),
            memories: vec![Record::create(Memory {
                agent_id: AgentId::new(),
                level: LevelName::new("core"),
                content: Content::new("I remember the beginning."),
            })],
            cognitions: vec![],
            experiences: vec![],
            connections: vec![],
            textures: vec![],
            levels: vec![],
            sensations: vec![],
            natures: vec![],
        };
        let rendered = DreamTemplate::new(&context).to_string();

        assert!(rendered.contains("## Your Memories"));
        assert!(rendered.contains("[core] I remember the beginning."));
    }

    #[test]
    fn dream_template_renders_cognitions() {
        let context = DreamContext {
            agent: test_agent(),
            persona: test_persona(),
            memories: vec![],
            cognitions: vec![Record::create(Cognition {
                agent_id: AgentId::new(),
                texture: TextureName::new("analytical"),
                content: Content::new("Patterns emerge from repetition."),
            })],
            experiences: vec![],
            connections: vec![],
            textures: vec![],
            levels: vec![],
            sensations: vec![],
            natures: vec![],
        };
        let rendered = DreamTemplate::new(&context).to_string();

        assert!(rendered.contains("## Your Cognitions"));
        assert!(rendered.contains("[analytical] Patterns emerge from repetition."));
    }

    #[test]
    fn dream_template_renders_textures_and_levels() {
        let context = DreamContext {
            agent: test_agent(),
            persona: test_persona(),
            memories: vec![],
            cognitions: vec![],
            experiences: vec![],
            connections: vec![],
            textures: vec![TextureRecord::init(
                "Analytical thinking",
                "Think analytically.",
                Texture {
                    name: TextureName::new("analytical"),
                },
            )],
            levels: vec![LevelRecord::init(
                "Core memories",
                "Fundamental knowledge.",
                Level {
                    name: LevelName::new("core"),
                },
            )],
            sensations: vec![],
            natures: vec![],
        };
        let rendered = DreamTemplate::new(&context).to_string();

        assert!(rendered.contains("## Cognitive Textures"));
        assert!(rendered.contains("analytical — Think analytically."));
        assert!(rendered.contains("## Memory Levels"));
        assert!(rendered.contains("core — Fundamental knowledge."));
    }

    #[test]
    fn dream_template_references_guidebook() {
        let context = DreamContext {
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
        };
        let rendered = DreamTemplate::new(&context).to_string();

        assert!(rendered.contains("oneiros guidebook atlas"));
        assert!(rendered.contains("garden"));
    }

    #[test]
    fn guidebook_template_renders_identity_and_sections() {
        let context = DreamContext {
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
        };
        let rendered = GuidebookTemplate::new(&context).to_string();

        assert!(rendered.contains("# Cognitive Guidebook for atlas"));
        assert!(rendered.contains("## Your Identity"));
        assert!(rendered.contains("A curious explorer agent"));
        assert!(rendered.contains("## Your Capabilities"));
        assert!(rendered.contains("## Your Lifecycle"));
        assert!(rendered.contains("## Your Agency"));
    }

    #[test]
    fn guidebook_template_renders_cli_commands_with_agent_name() {
        let context = DreamContext {
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
        };
        let rendered = GuidebookTemplate::new(&context).to_string();

        assert!(rendered.contains("oneiros cognition add atlas"));
        assert!(rendered.contains("oneiros memory add atlas"));
        assert!(rendered.contains("oneiros cognition list atlas"));
        assert!(rendered.contains("oneiros memory list atlas"));
        assert!(rendered.contains("oneiros dream atlas"));
        assert!(rendered.contains("oneiros reflect atlas"));
        assert!(rendered.contains("oneiros introspect atlas"));
    }

    #[test]
    fn guidebook_template_renders_textures_and_levels() {
        let context = DreamContext {
            agent: test_agent(),
            persona: test_persona(),
            memories: vec![],
            cognitions: vec![],
            experiences: vec![],
            connections: vec![],
            textures: vec![TextureRecord::init(
                "Analytical thinking",
                "Think analytically.",
                Texture {
                    name: TextureName::new("analytical"),
                },
            )],
            levels: vec![LevelRecord::init(
                "Core memories",
                "Fundamental knowledge.",
                Level {
                    name: LevelName::new("core"),
                },
            )],
            sensations: vec![],
            natures: vec![],
        };
        let rendered = GuidebookTemplate::new(&context).to_string();

        assert!(rendered.contains("**analytical** — Think analytically."));
        assert!(rendered.contains("**core** — Fundamental knowledge."));
    }

    #[test]
    fn guidebook_template_omits_empty_textures_and_levels() {
        let context = DreamContext {
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
        };
        let rendered = GuidebookTemplate::new(&context).to_string();

        assert!(!rendered.contains("Your current textures:"));
        assert!(!rendered.contains("Your current levels:"));
    }

    #[test]
    fn guidebook_template_documents_agency() {
        let context = DreamContext {
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
        };
        let rendered = GuidebookTemplate::new(&context).to_string();

        assert!(rendered.contains("oneiros texture set"));
        assert!(rendered.contains("oneiros level set"));
        assert!(rendered.contains("oneiros agent create"));
    }

    #[test]
    fn introspect_template_renders_agent_name() {
        let agent = test_agent();
        let rendered = IntrospectTemplate::new(&agent).to_string();

        assert!(rendered.contains("You are atlas."));
        assert!(rendered.contains("oneiros memory add atlas"));
        assert!(rendered.contains("oneiros cognition add atlas"));
        assert!(rendered.contains("compact"));
    }

    #[test]
    fn reflect_template_renders_agent_name() {
        let agent = test_agent();
        let rendered = ReflectTemplate::new(&agent).to_string();

        assert!(rendered.contains("You are atlas."));
        assert!(rendered.contains("oneiros memory add atlas"));
        assert!(rendered.contains("oneiros cognition add atlas"));
        assert!(rendered.contains("shifted"));
    }

    #[test]
    fn sense_template_renders_agent_name() {
        let agent = test_agent();
        let rendered = SenseTemplate::new(&agent, "").to_string();

        assert!(rendered.contains("You are atlas."));
        assert!(rendered.contains("oneiros cognition add atlas"));
        assert!(rendered.contains("sensing"));
    }

    #[test]
    fn sense_template_includes_event_data_when_present() {
        let agent = test_agent();
        let event_json = r#"{"event": "PreCompact", "data": {}}"#;
        let rendered = SenseTemplate::new(&agent, event_json).to_string();

        assert!(rendered.contains("## What You Sensed"));
        assert!(rendered.contains("PreCompact"));
    }

    #[test]
    fn sense_template_omits_event_section_when_empty() {
        let agent = test_agent();
        let rendered = SenseTemplate::new(&agent, "").to_string();

        assert!(!rendered.contains("## What You Sensed"));
    }
}
