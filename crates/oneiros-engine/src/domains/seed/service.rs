use crate::*;

pub(crate) struct SeedService;

impl SeedService {
    pub(crate) async fn core(client: &Client) -> Result<SeedResponse, SeedError> {
        let level = LevelClient::new(client);
        let texture = TextureClient::new(client);
        let sensation = SensationClient::new(client);
        let nature = NatureClient::new(client);
        let persona = PersonaClient::new(client);
        let urge = UrgeClient::new(client);

        for (name, description, prompt) in [
            (
                "working",
                "What you're actively processing — in-flight thoughts, scratchpad notes, things you haven't consolidated yet.",
                "",
            ),
            (
                "session",
                "Current session context. Learnings, observations, and decisions from what you're doing now.",
                "",
            ),
            (
                "project",
                "Durable knowledge for the lifetime of the project.",
                "",
            ),
            (
                "archival",
                "Deep history — milestone reflections, post-mortems, and historical context.",
                "",
            ),
            (
                "core",
                "Identity fundaments — the memories that define how you process everything else.",
                "",
            ),
        ] {
            level
                .set(&SetLevel::builder()
                    .name(LevelName::new(name))
                    .description(Description::from(description))
                    .prompt(Prompt::from(prompt))
                    .build())
                .await?;
        }

        for (name, description) in [
            (
                "observation",
                "When you notice something interesting about the code, architecture, or process.",
            ),
            ("learning", "Capture moments of genuine understanding."),
            ("question", "Record questions you cannot answer yet."),
            (
                "connection",
                "When you see a relationship between separate domains.",
            ),
            ("reflection", "Step back and think about how work is going."),
            (
                "assessment",
                "Provide a definitive perspective from your domain expertise.",
            ),
            ("handoff", "Write what the next session needs to know."),
            (
                "working",
                "Capture thoughts as they happen during implementation.",
            ),
            ("dream", "Impressions that surface during dreaming."),
            (
                "bond",
                "After a meaningful interaction, capture it as a bond.",
            ),
        ] {
            texture
                .set(&SetTexture::builder()
                    .name(TextureName::new(name))
                    .description(Description::from(description))
                    .build())
                .await?;
        }

        for (name, description) in [
            ("caused", "Directed. One thought produced another."),
            (
                "continues",
                "Directed. Picks up where a previous thread left off.",
            ),
            (
                "distills",
                "Directed. A consolidated understanding formed from earlier raw thoughts.",
            ),
            (
                "echoes",
                "Undirected. Things that resonate thematically without clear causation.",
            ),
            (
                "grounds",
                "Directed. A thought grounded in a memory or prior knowledge.",
            ),
            (
                "tensions",
                "Undirected. Ideas that pull against each other.",
            ),
        ] {
            sensation
                .set(&SetSensation::builder()
                    .name(name)
                    .description(description)
                    .build())
                .await?;
        }

        for (name, description) in [
            (
                "context",
                "Directed. One entity provides background or framing for another.",
            ),
            ("continuation", "Directed. One entity follows from another."),
            (
                "contrast",
                "Undirected. Two entities differ in an instructive way.",
            ),
            ("origin", "Directed. One entity gave rise to another."),
            (
                "reference",
                "Undirected. Two entities are related without implying causation.",
            ),
            (
                "revision",
                "Directed. One entity updates or supersedes another.",
            ),
        ] {
            nature
                .set(&SetNature::builder()
                    .name(name)
                    .description(description)
                    .build())
                .await?;
        }

        for (name, description) in [
            (
                "process",
                "Internal lifecycle agents — orchestration, session structure, cognitive hygiene.",
            ),
            (
                "expert",
                "Domain specialists — deep knowledge in a specific area.",
            ),
            ("scribe", "Record-keepers — maintain the cognitive record."),
        ] {
            persona
                .set(&SetPersona::builder()
                    .name(name)
                    .description(description)
                    .build())
                .await?;
        }

        for (name, description, prompt) in [
            (
                "introspect",
                "The pull to look inward.",
                "Pause to examine your cognitive state.",
            ),
            (
                "catharsis",
                "The need to release accumulated tension.",
                "Confront what's stuck and clear space.",
            ),
            (
                "recollect",
                "The drive to gather scattered pieces.",
                "Search for connections and consolidate.",
            ),
            (
                "retrospect",
                "The impulse to look back.",
                "Review the trajectory and capture learnings.",
            ),
        ] {
            urge.set(&SetUrge::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build())
                .await?;
        }

        Ok(SeedResponse::SeedComplete)
    }

    pub(crate) async fn agents(client: &Client) -> Result<SeedResponse, SeedError> {
        let persona_client = PersonaClient::new(client);
        let agent_client = AgentClient::new(client);

        let personas = persona_client
            .list(&ListPersonas::builder().build())
            .await?;

        let persona_names: Vec<&str> = match &personas {
            PersonaResponse::Personas(listed) => {
                listed.items.iter().map(|p| p.data.name.as_str()).collect()
            }
            _ => vec![],
        };

        if !persona_names.contains(&"process") || !persona_names.contains(&"scribe") {
            return Err(SeedError::MissingPersonas);
        }

        let agents = [
            (
                "governor",
                "process",
                "Primary orchestration agent — routes work, enforces cognitive processes.",
                include_str!("../../../templates/seed/agents/governor.md"),
            ),
            (
                "oneiroi",
                "process",
                "The brain's self-awareness — watches the cognitive loop, notices drift, tends the garden from inside.",
                include_str!("../../../templates/seed/agents/oneiroi.md"),
            ),
            (
                "activity",
                "scribe",
                "Watches for artifacts worth preserving — outputs, documents, and references that deserve a place in the brain's archive.",
                include_str!("../../../templates/seed/agents/activity.md"),
            ),
        ];

        for (name, persona, description, prompt) in agents {
            let agent_name = AgentName::new(name);
            let full_name = agent_name.normalize_with(&PersonaName::new(persona));

            let exists = match agent_client.get(&full_name).await {
                Ok(AgentResponse::AgentDetails(_)) => true,
                _ => false,
            };

            if exists {
                continue;
            }

            agent_client
                .create(&CreateAgent::builder()
                    .name(agent_name)
                    .persona(PersonaName::new(persona))
                    .description(Description::from(description))
                    .prompt(Prompt::from(prompt))
                    .build())
                .await?;
        }

        Ok(SeedResponse::AgentsSeedComplete)
    }
}
