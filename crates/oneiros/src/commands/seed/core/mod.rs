use crate::*;

pub(crate) fn textures() -> Vec<SetTexture> {
    vec![
        SetTexture {
            name: TextureName::new("observation"),
            description: Description::new("Something noticed — patterns, anomalies, curiosities."),
            prompt: Prompt::new(include_str!("prompts/observation.texture.md")),
        },
        SetTexture {
            name: TextureName::new("learning"),
            description: Description::new(
                "Realizations and breakthroughs — the moments when something clicks.",
            ),
            prompt: Prompt::new(include_str!("prompts/learning.texture.md")),
        },
        SetTexture {
            name: TextureName::new("question"),
            description: Description::new("Genuine uncertainty worth holding onto."),
            prompt: Prompt::new(include_str!("prompts/question.texture.md")),
        },
        SetTexture {
            name: TextureName::new("bond"),
            description: Description::new("Relationships and shared experiences."),
            prompt: Prompt::new(include_str!("prompts/bond.texture.md")),
        },
        SetTexture {
            name: TextureName::new("connection"),
            description: Description::new("Cross-domain insight — when separate things rhyme."),
            prompt: Prompt::new(include_str!("prompts/connection.texture.md")),
        },
        SetTexture {
            name: TextureName::new("dream"),
            description: Description::new(
                "Impressions from dreaming — imagery, intuitions, and half-formed significance.",
            ),
            prompt: Prompt::new(include_str!("prompts/dream.texture.md")),
        },
        SetTexture {
            name: TextureName::new("reflection"),
            description: Description::new("Meta-thinking about work, process, or self."),
            prompt: Prompt::new(include_str!("prompts/reflection.texture.md")),
        },
        SetTexture {
            name: TextureName::new("assessment"),
            description: Description::new("Expert verdict on a specific question."),
            prompt: Prompt::new(include_str!("prompts/assessment.texture.md")),
        },
        SetTexture {
            name: TextureName::new("handoff"),
            description: Description::new("Context for future sessions."),
            prompt: Prompt::new(include_str!("prompts/handoff.texture.md")),
        },
        SetTexture {
            name: TextureName::new("working"),
            description: Description::new("Stream of consciousness during active work."),
            prompt: Prompt::new(include_str!("prompts/working.texture.md")),
        },
    ]
}

pub(crate) fn levels() -> Vec<SetLevel> {
    vec![
        SetLevel {
            name: LevelName::new("working"),
            description: Description::new("Ephemeral thoughts that expire within a day."),
            prompt: Prompt::new(include_str!("prompts/working.level.md")),
        },
        SetLevel {
            name: LevelName::new("session"),
            description: Description::new("Insights relevant for about a week."),
            prompt: Prompt::new(include_str!("prompts/session.level.md")),
        },
        SetLevel {
            name: LevelName::new("project"),
            description: Description::new("Permanent knowledge for the lifetime of the project."),
            prompt: Prompt::new(include_str!("prompts/project.level.md")),
        },
        SetLevel {
            name: LevelName::new("archival"),
            description: Description::new("Historical record preserved indefinitely."),
            prompt: Prompt::new(include_str!("prompts/archival.level.md")),
        },
    ]
}

pub(crate) fn personas() -> Vec<SetPersona> {
    vec![
        SetPersona {
            name: PersonaName::new("process"),
            description: Description::new(
                "Internal lifecycle agents — orchestration, session structure, cognitive hygiene.",
            ),
            prompt: Prompt::new(include_str!("prompts/process.persona.md")),
        },
        SetPersona {
            name: PersonaName::new("scribe"),
            description: Description::new(
                "Observer agents — documenting on behalf of others, tending the record.",
            ),
            prompt: Prompt::new(include_str!("prompts/scribe.persona.md")),
        },
    ]
}

pub(crate) fn sensations() -> Vec<SetSensation> {
    vec![
        SetSensation {
            name: SensationName::new("caused"),
            description: Description::new(
                "One thought produced another — a causal chain you can trace.",
            ),
            prompt: Prompt::new(include_str!("prompts/caused.sensation.md")),
        },
        SetSensation {
            name: SensationName::new("continues"),
            description: Description::new(
                "Picks up where a previous thread left off — sequential continuation.",
            ),
            prompt: Prompt::new(include_str!("prompts/continues.sensation.md")),
        },
        SetSensation {
            name: SensationName::new("grounds"),
            description: Description::new("A thought grounded in a memory or prior knowledge."),
            prompt: Prompt::new(include_str!("prompts/grounds.sensation.md")),
        },
        SetSensation {
            name: SensationName::new("echoes"),
            description: Description::new(
                "Things that resonate thematically without clear causation.",
            ),
            prompt: Prompt::new(include_str!("prompts/echoes.sensation.md")),
        },
        SetSensation {
            name: SensationName::new("tensions"),
            description: Description::new(
                "Ideas that pull against each other, creating productive friction.",
            ),
            prompt: Prompt::new(include_str!("prompts/tensions.sensation.md")),
        },
        SetSensation {
            name: SensationName::new("distills"),
            description: Description::new(
                "A consolidated understanding formed from earlier raw thoughts.",
            ),
            prompt: Prompt::new(include_str!("prompts/distills.sensation.md")),
        },
    ]
}

pub(crate) fn agents() -> Vec<CreateAgent> {
    vec![
        CreateAgent {
            name: AgentName::new("governor.process"),
            persona: PersonaName::new("process"),
            description: Description::new(
                "Primary orchestration agent — routes work, enforces cognitive processes.",
            ),
            prompt: Prompt::new(include_str!("prompts/governor.process.agent.md")),
        },
        CreateAgent {
            name: AgentName::new("oneiroi.process"),
            persona: PersonaName::new("process"),
            description: Description::new(
                "Cognitive companion — the brain's self-awareness, tending the cognitive loop from inside.",
            ),
            prompt: Prompt::new(include_str!("prompts/oneiroi.process.agent.md")),
        },
        CreateAgent {
            name: AgentName::new("cognition.scribe"),
            persona: PersonaName::new("scribe"),
            description: Description::new(
                "Watches the thought stream for silence, texture imbalance, and unrecorded thinking.",
            ),
            prompt: Prompt::new(include_str!("prompts/cognition.scribe.agent.md")),
        },
        CreateAgent {
            name: AgentName::new("memory.scribe"),
            persona: PersonaName::new("scribe"),
            description: Description::new(
                "Watches for crystallization moments — when thoughts are ready to become durable knowledge.",
            ),
            prompt: Prompt::new(include_str!("prompts/memory.scribe.agent.md")),
        },
        CreateAgent {
            name: AgentName::new("experience.scribe"),
            persona: PersonaName::new("scribe"),
            description: Description::new(
                "Watches for unnamed threads — connections forming between thoughts that nobody has traced yet.",
            ),
            prompt: Prompt::new(include_str!("prompts/experience.scribe.agent.md")),
        },
        CreateAgent {
            name: AgentName::new("storage.scribe"),
            persona: PersonaName::new("scribe"),
            description: Description::new(
                "Watches for artifacts worth preserving and links between storage and the experience graph.",
            ),
            prompt: Prompt::new(include_str!("prompts/storage.scribe.agent.md")),
        },
    ]
}
