mod dream;
mod introspect;
mod reflect;

pub use dream::DreamTemplate;
pub use introspect::IntrospectTemplate;
pub use reflect::ReflectTemplate;

#[cfg(test)]
mod tests {
    use super::*;
    use oneiros_model::*;

    fn test_agent() -> Agent {
        Agent {
            id: AgentId::new(),
            name: AgentName::new("atlas"),
            persona: PersonaName::new("explorer"),
            description: Description::new("A curious explorer agent."),
            prompt: Prompt::new("You explore and discover."),
        }
    }

    fn test_persona() -> Persona {
        Persona {
            name: PersonaName::new("explorer"),
            description: Description::new("An explorer persona."),
            prompt: Prompt::new("Explore with curiosity."),
        }
    }

    #[test]
    fn dream_template_renders_identity_and_persona() {
        let context = DreamContext {
            agent: test_agent(),
            persona: test_persona(),
            memories: vec![],
            cognitions: vec![],
            textures: vec![],
            levels: vec![],
        };
        let rendered = DreamTemplate::new(&context).to_string();

        assert!(rendered.contains("You are waking as atlas."));
        assert!(rendered.contains("## Your Identity"));
        assert!(rendered.contains("A curious explorer agent."));
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
            textures: vec![],
            levels: vec![],
        };
        let rendered = DreamTemplate::new(&context).to_string();

        assert!(!rendered.contains("## Your Memories"));
        assert!(!rendered.contains("## Your Cognitions"));
        assert!(!rendered.contains("## Cognitive Textures"));
        assert!(!rendered.contains("## Memory Levels"));
    }

    #[test]
    fn dream_template_renders_memories() {
        let context = DreamContext {
            agent: test_agent(),
            persona: test_persona(),
            memories: vec![Memory {
                id: MemoryId::new(),
                agent_id: AgentId::new(),
                level: LevelName::new("core"),
                content: Content::new("I remember the beginning."),
                created_at: chrono::Utc::now(),
            }],
            cognitions: vec![],
            textures: vec![],
            levels: vec![],
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
            cognitions: vec![Cognition {
                id: CognitionId::new(),
                agent_id: AgentId::new(),
                texture: TextureName::new("analytical"),
                content: Content::new("Patterns emerge from repetition."),
                created_at: chrono::Utc::now(),
            }],
            textures: vec![],
            levels: vec![],
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
            textures: vec![Texture {
                name: TextureName::new("analytical"),
                description: Description::new("Analytical thinking"),
                prompt: Prompt::new("Think analytically."),
            }],
            levels: vec![Level {
                name: LevelName::new("core"),
                description: Description::new("Core memories"),
                prompt: Prompt::new("Fundamental knowledge."),
            }],
        };
        let rendered = DreamTemplate::new(&context).to_string();

        assert!(rendered.contains("## Cognitive Textures"));
        assert!(rendered.contains("analytical — Think analytically."));
        assert!(rendered.contains("## Memory Levels"));
        assert!(rendered.contains("core — Fundamental knowledge."));
    }

    #[test]
    fn introspect_template_renders_agent_name() {
        let agent = test_agent();
        let rendered = IntrospectTemplate::new(&agent).to_string();

        assert!(rendered.contains("You are atlas."));
        assert!(rendered.contains("oneiros memory add --agent atlas"));
        assert!(rendered.contains("oneiros cognition add --agent atlas"));
        assert!(rendered.contains("compaction"));
    }

    #[test]
    fn reflect_template_renders_agent_name() {
        let agent = test_agent();
        let rendered = ReflectTemplate::new(&agent).to_string();

        assert!(rendered.contains("You are atlas."));
        assert!(rendered.contains("oneiros memory add --agent atlas"));
        assert!(rendered.contains("oneiros cognition add --agent atlas"));
        assert!(rendered.contains("reflect"));
    }
}
