use axum::{Json, extract::Path};
use oneiros_model::*;

use crate::*;

pub(crate) async fn handler(
    ticket: ActorContext,
    Path(agent_name): Path<AgentName>,
) -> Result<Json<DreamContext>, Error> {
    let (id, name, persona_name, desc, prompt) = ticket
        .db
        .get_agent(&agent_name)?
        .ok_or(NotFound::Agent(agent_name.clone()))?;

    let agent = Agent {
        id: id.parse().unwrap_or_default(),
        name: AgentName::new(name),
        persona: PersonaName::new(&persona_name),
        description: Description::new(desc),
        prompt: Prompt::new(prompt),
    };

    let begun = Events::Dreaming(DreamingEvents::DreamBegun {
        agent: agent.name.clone(),
    });
    ticket.db.log_event(&begun, &[])?;

    let (pname, pdesc, pprompt) = ticket
        .db
        .get_persona(PersonaName::new(&persona_name))?
        .ok_or(NotFound::Persona(PersonaName::new(persona_name)))?;

    let persona = Persona {
        name: PersonaName::new(pname),
        description: Description::new(pdesc),
        prompt: Prompt::new(pprompt),
    };

    let memories = ticket
        .db
        .list_memories_by_agent(agent.id.to_string())?
        .into_iter()
        .map(|(mid, agent_id, level, content, created_at)| Memory {
            id: mid.parse().unwrap_or_default(),
            agent_id: agent_id.parse().unwrap_or_default(),
            level: LevelName::new(level),
            content: Content::new(content),
            created_at: created_at.parse().unwrap_or_default(),
        })
        .collect();

    let cognitions = ticket
        .db
        .list_cognitions_by_agent(agent.id.to_string())?
        .into_iter()
        .map(|(cid, agent_id, texture, content, created_at)| Cognition {
            id: cid.parse().unwrap_or_default(),
            agent_id: agent_id.parse().unwrap_or_default(),
            texture: TextureName::new(texture),
            content: Content::new(content),
            created_at: created_at.parse().unwrap_or_default(),
        })
        .collect();

    let textures = ticket
        .db
        .list_textures()?
        .into_iter()
        .map(|(name, desc, prompt)| Texture {
            name: TextureName::new(name),
            description: Description::new(desc),
            prompt: Prompt::new(prompt),
        })
        .collect();

    let levels = ticket
        .db
        .list_levels()?
        .into_iter()
        .map(|(name, desc, prompt)| Level {
            name: LevelName::new(name),
            description: Description::new(desc),
            prompt: Prompt::new(prompt),
        })
        .collect();

    let sensations = ticket
        .db
        .list_sensations()?
        .into_iter()
        .map(|(name, desc, prompt)| Sensation {
            name: SensationName::new(name),
            description: Description::new(desc),
            prompt: Prompt::new(prompt),
        })
        .collect();

    let experiences: Vec<Experience> = ticket
        .db
        .list_experiences_by_agent(agent.id.to_string())?
        .into_iter()
        .map(|(eid, agent_id, sensation, description, created_at)| {
            let refs = ticket
                .db
                .list_experience_refs(&eid)
                .unwrap_or_default()
                .into_iter()
                .map(
                    |(_experience_id, record_id, record_kind, role, _created_at)| RecordRef {
                        id: record_id.parse().unwrap_or_default(),
                        kind: record_kind.parse().unwrap_or(RecordKind::Cognition),
                        role: role.map(Label::new),
                    },
                )
                .collect();

            Experience {
                id: eid.parse().unwrap_or_default(),
                agent_id: agent_id.parse().unwrap_or_default(),
                sensation: SensationName::new(sensation),
                description: Content::new(description),
                refs,
                created_at: created_at.parse().unwrap_or_default(),
            }
        })
        .collect();

    let context = DreamContext {
        agent,
        persona,
        memories,
        cognitions,
        experiences,
        textures,
        levels,
        sensations,
    };

    let complete = Events::Dreaming(DreamingEvents::DreamComplete(Box::new(context.clone())));
    ticket.db.log_event(&complete, &[])?;

    Ok(Json(context))
}
