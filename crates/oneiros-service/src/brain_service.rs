use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use oneiros_db::Database;
use oneiros_model::*;
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use tokio::sync::broadcast;

use crate::handlers::dream::collector::DreamCollector;
use crate::{Error, projections};

/// Domain service for brain-scoped operations.
///
/// Owns the validate → construct → persist → broadcast pipeline.
/// Handlers delegate here; they own only HTTP parsing and response formatting.
pub(crate) struct BrainService<'a> {
    db: &'a Database,
    event_tx: &'a broadcast::Sender<Events>,
}

impl<'a> BrainService<'a> {
    pub fn new(db: &'a Database, event_tx: &'a broadcast::Sender<Events>) -> Self {
        Self { db, event_tx }
    }

    /// Persist a state-changing event (runs BRAIN projections) then broadcast.
    fn log_and_broadcast(&self, event: &Events) -> Result<(), Error> {
        self.db.log_event(event, projections::BRAIN)?;
        let _ = self.event_tx.send(event.clone());
        Ok(())
    }

    /// Persist an observational marker event (no projections) then broadcast.
    fn log_marker(&self, event: &Events) -> Result<(), Error> {
        self.db.log_event(event, &[])?;
        let _ = self.event_tx.send(event.clone());
        Ok(())
    }

    // ── Agent operations ──────────────────────────────────────────────

    pub fn create_agent(&self, request: CreateAgentRequest) -> Result<Agent, Error> {
        self.db
            .get_persona(&request.persona)?
            .ok_or(NotFound::Persona(request.persona.clone()))?;

        if self.db.agent_name_exists(&request.name)? {
            return Err(Conflicts::Agent(request.name).into());
        }

        let agent = Agent::init(
            request.description,
            request.prompt,
            request.name,
            request.persona,
        );

        let event = Events::Agent(AgentEvents::AgentCreated(agent.clone()));
        self.log_and_broadcast(&event)?;

        Ok(agent)
    }

    pub fn list_agents(&self) -> Result<Vec<Agent>, Error> {
        Ok(self.db.list_agents()?)
    }

    pub fn get_agent(&self, name: &AgentName) -> Result<Agent, Error> {
        self.db
            .get_agent(name)?
            .ok_or_else(|| NotFound::Agent(name.clone()).into())
    }

    pub fn update_agent(
        &self,
        name: &AgentName,
        request: UpdateAgentRequest,
    ) -> Result<Agent, Error> {
        let existing = self
            .db
            .get_agent(name)?
            .ok_or(NotFound::Agent(name.clone()))?;

        self.db
            .get_persona(&request.persona)?
            .ok_or(NotFound::Persona(request.persona.clone()))?;

        let agent = Agent::construct(
            existing.id,
            request.description,
            request.prompt,
            existing.name.clone(),
            request.persona,
        );

        let event = Events::Agent(AgentEvents::AgentUpdated(agent.clone()));
        self.log_and_broadcast(&event)?;

        Ok(agent)
    }

    pub fn remove_agent(&self, name: AgentName) -> Result<(), Error> {
        let event = Events::Agent(AgentEvents::AgentRemoved { name });
        self.log_and_broadcast(&event)?;

        Ok(())
    }

    // ── Cognition operations ──────────────────────────────────────────

    pub fn add_cognition(&self, request: AddCognitionRequest) -> Result<Cognition, Error> {
        let agent = self
            .db
            .get_agent(&request.agent)?
            .ok_or(NotFound::Agent(request.agent.clone()))?;

        self.db
            .get_texture(&request.texture)?
            .ok_or(NotFound::Texture(request.texture.clone()))?;

        let cognition = Cognition::create(agent.id, request.texture, request.content);

        let event = Events::Cognition(CognitionEvents::CognitionAdded(cognition.clone()));
        self.log_and_broadcast(&event)?;

        Ok(cognition)
    }

    pub fn get_cognition(&self, id: &CognitionId) -> Result<Cognition, Error> {
        self.db
            .get_cognition(id.to_string())?
            .ok_or_else(|| NotFound::Cognition(*id).into())
    }

    pub fn list_cognitions(
        &self,
        agent: Option<AgentName>,
        texture: Option<TextureName>,
    ) -> Result<Vec<Cognition>, Error> {
        let cognitions = match (agent, texture) {
            (Some(agent_name), Some(texture)) => {
                let agent = self
                    .db
                    .get_agent(&agent_name)?
                    .ok_or(NotFound::Agent(agent_name))?;

                self.db
                    .get_texture(&texture)?
                    .ok_or(NotFound::Texture(texture.clone()))?;

                self.db
                    .list_cognitions_by_agent_and_texture(agent.id.to_string(), &texture)?
            }
            (Some(agent_name), None) => {
                let agent = self
                    .db
                    .get_agent(&agent_name)?
                    .ok_or(NotFound::Agent(agent_name))?;

                self.db.list_cognitions_by_agent(agent.id.to_string())?
            }
            (None, Some(texture)) => {
                self.db
                    .get_texture(&texture)?
                    .ok_or(NotFound::Texture(texture.clone()))?;

                self.db.list_cognitions_by_texture(&texture)?
            }
            (None, None) => self.db.list_cognitions()?,
        };

        Ok(cognitions)
    }

    // ── Level operations ────────────────────────────────────────────

    pub fn set_level(&self, level: Level) -> Result<Level, Error> {
        let event = Events::Level(LevelEvents::LevelSet(level.clone()));
        self.log_and_broadcast(&event)?;
        Ok(level)
    }

    pub fn list_levels(&self) -> Result<Vec<Level>, Error> {
        Ok(self.db.list_levels()?)
    }

    pub fn get_level(&self, name: &LevelName) -> Result<Level, Error> {
        self.db
            .get_level(name)?
            .ok_or_else(|| NotFound::Level(name.clone()).into())
    }

    pub fn remove_level(&self, name: LevelName) -> Result<(), Error> {
        let event = Events::Level(LevelEvents::LevelRemoved { name });
        self.log_and_broadcast(&event)?;
        Ok(())
    }

    // ── Nature operations ───────────────────────────────────────────

    pub fn set_nature(&self, nature: Nature) -> Result<Nature, Error> {
        let event = Events::Nature(NatureEvents::NatureSet(nature.clone()));
        self.log_and_broadcast(&event)?;
        Ok(nature)
    }

    pub fn list_natures(&self) -> Result<Vec<Nature>, Error> {
        Ok(self.db.list_natures()?)
    }

    pub fn get_nature(&self, name: &NatureName) -> Result<Nature, Error> {
        self.db
            .get_nature(name)?
            .ok_or_else(|| NotFound::Nature(name.clone()).into())
    }

    pub fn remove_nature(&self, name: NatureName) -> Result<(), Error> {
        let event = Events::Nature(NatureEvents::NatureRemoved { name });
        self.log_and_broadcast(&event)?;
        Ok(())
    }

    // ── Persona operations ──────────────────────────────────────────

    pub fn set_persona(&self, persona: Persona) -> Result<Persona, Error> {
        let event = Events::Persona(PersonaEvents::PersonaSet(persona.clone()));
        self.log_and_broadcast(&event)?;
        Ok(persona)
    }

    pub fn list_personas(&self) -> Result<Vec<Persona>, Error> {
        Ok(self.db.list_personas()?)
    }

    pub fn get_persona(&self, name: &PersonaName) -> Result<Persona, Error> {
        self.db
            .get_persona(name)?
            .ok_or_else(|| NotFound::Persona(name.clone()).into())
    }

    pub fn remove_persona(&self, name: PersonaName) -> Result<(), Error> {
        let event = Events::Persona(PersonaEvents::PersonaRemoved { name });
        self.log_and_broadcast(&event)?;
        Ok(())
    }

    // ── Sensation operations ────────────────────────────────────────

    pub fn set_sensation(&self, sensation: Sensation) -> Result<Sensation, Error> {
        let event = Events::Sensation(SensationEvents::SensationSet(sensation.clone()));
        self.log_and_broadcast(&event)?;
        Ok(sensation)
    }

    pub fn list_sensations(&self) -> Result<Vec<Sensation>, Error> {
        Ok(self.db.list_sensations()?)
    }

    pub fn get_sensation(&self, name: &SensationName) -> Result<Sensation, Error> {
        self.db
            .get_sensation(name)?
            .ok_or_else(|| NotFound::Sensation(name.clone()).into())
    }

    pub fn remove_sensation(&self, name: SensationName) -> Result<(), Error> {
        let event = Events::Sensation(SensationEvents::SensationRemoved { name });
        self.log_and_broadcast(&event)?;
        Ok(())
    }

    // ── Texture operations ──────────────────────────────────────────

    pub fn set_texture(&self, texture: Texture) -> Result<Texture, Error> {
        let event = Events::Texture(TextureEvents::TextureSet(texture.clone()));
        self.log_and_broadcast(&event)?;
        Ok(texture)
    }

    pub fn list_textures(&self) -> Result<Vec<Texture>, Error> {
        Ok(self.db.list_textures()?)
    }

    pub fn get_texture(&self, name: &TextureName) -> Result<Texture, Error> {
        self.db
            .get_texture(name)?
            .ok_or_else(|| NotFound::Texture(name.clone()).into())
    }

    pub fn remove_texture(&self, name: TextureName) -> Result<(), Error> {
        let event = Events::Texture(TextureEvents::TextureRemoved { name });
        self.log_and_broadcast(&event)?;
        Ok(())
    }

    // ── Memory operations ───────────────────────────────────────────

    pub fn add_memory(&self, request: AddMemoryRequest) -> Result<Memory, Error> {
        let agent = self
            .db
            .get_agent(&request.agent)?
            .ok_or(NotFound::Agent(request.agent.clone()))?;

        self.db
            .get_level(&request.level)?
            .ok_or(NotFound::Level(request.level.clone()))?;

        let memory = Memory::create(agent.id, request.level, request.content);

        let event = Events::Memory(MemoryEvents::MemoryAdded(memory.clone()));
        self.log_and_broadcast(&event)?;

        Ok(memory)
    }

    pub fn get_memory(&self, id: &MemoryId) -> Result<Memory, Error> {
        self.db
            .get_memory(id.to_string())?
            .ok_or_else(|| NotFound::Memory(*id).into())
    }

    pub fn list_memories(
        &self,
        agent: Option<AgentName>,
        level: Option<LevelName>,
    ) -> Result<Vec<Memory>, Error> {
        let memories = match (agent, level) {
            (Some(agent_name), Some(level)) => {
                let agent = self
                    .db
                    .get_agent(&agent_name)?
                    .ok_or(NotFound::Agent(agent_name))?;

                self.db
                    .get_level(&level)?
                    .ok_or(NotFound::Level(level.clone()))?;

                self.db
                    .list_memories_by_agent_and_level(agent.id.to_string(), &level)?
            }
            (Some(agent_name), None) => {
                let agent = self
                    .db
                    .get_agent(&agent_name)?
                    .ok_or(NotFound::Agent(agent_name))?;

                self.db.list_memories_by_agent(agent.id.to_string())?
            }
            (None, Some(level)) => {
                self.db
                    .get_level(&level)?
                    .ok_or(NotFound::Level(level.clone()))?;

                self.db.list_memories_by_level(&level)?
            }
            (None, None) => self.db.list_memories()?,
        };

        Ok(memories)
    }

    // ── Experience operations ───────────────────────────────────────

    pub fn create_experience(&self, request: CreateExperienceRequest) -> Result<Experience, Error> {
        let agent = self
            .db
            .get_agent(&request.agent)?
            .ok_or(NotFound::Agent(request.agent.clone()))?;

        self.db
            .get_sensation(&request.sensation)?
            .ok_or(NotFound::Sensation(request.sensation.clone()))?;

        let experience = Experience::create(agent.id, request.sensation, request.description);

        let event = Events::Experience(ExperienceEvents::ExperienceCreated(experience.clone()));
        self.log_and_broadcast(&event)?;

        Ok(experience)
    }

    pub fn get_experience(&self, id: &ExperienceId) -> Result<Experience, Error> {
        self.db
            .get_experience(id.to_string())?
            .ok_or_else(|| NotFound::Experience(*id).into())
    }

    pub fn list_experiences(
        &self,
        agent: Option<AgentName>,
        sensation: Option<SensationName>,
    ) -> Result<Vec<Experience>, Error> {
        let experiences = match (agent, sensation) {
            (Some(agent_name), Some(sensation)) => {
                let agent = self
                    .db
                    .get_agent(&agent_name)?
                    .ok_or(NotFound::Agent(agent_name))?;

                self.db
                    .get_sensation(&sensation)?
                    .ok_or(NotFound::Sensation(sensation.clone()))?;

                self.db
                    .list_experiences_by_agent(agent.id.to_string())?
                    .into_iter()
                    .filter(|exp| exp.sensation == sensation)
                    .collect()
            }
            (Some(agent_name), None) => {
                let agent = self
                    .db
                    .get_agent(&agent_name)?
                    .ok_or(NotFound::Agent(agent_name))?;

                self.db.list_experiences_by_agent(agent.id.to_string())?
            }
            (None, Some(sensation)) => {
                self.db
                    .get_sensation(&sensation)?
                    .ok_or(NotFound::Sensation(sensation.clone()))?;

                self.db.list_experiences_by_sensation(&sensation)?
            }
            (None, None) => self.db.list_experiences()?,
        };

        Ok(experiences)
    }

    pub fn update_experience_description(
        &self,
        id: &ExperienceId,
        request: UpdateExperienceDescriptionRequest,
    ) -> Result<Experience, Error> {
        self.db
            .get_experience(id.to_string())?
            .ok_or(NotFound::Experience(*id))?;

        let event = Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated {
            experience_id: *id,
            description: request.description,
        });

        self.log_and_broadcast(&event)?;

        // Re-fetch to include projection updates.
        self.db
            .get_experience(id.to_string())?
            .ok_or_else(|| NotFound::Experience(*id).into())
    }

    pub fn update_experience_sensation(
        &self,
        id: &ExperienceId,
        request: UpdateExperienceSensationRequest,
    ) -> Result<Experience, Error> {
        self.db
            .get_experience(id.to_string())?
            .ok_or(NotFound::Experience(*id))?;

        let event = Events::Experience(ExperienceEvents::ExperienceSensationUpdated {
            experience_id: *id,
            sensation: request.sensation,
        });

        self.log_and_broadcast(&event)?;

        // Re-fetch to include projection updates.
        self.db
            .get_experience(id.to_string())?
            .ok_or_else(|| NotFound::Experience(*id).into())
    }

    // ── Connection operations ───────────────────────────────────────

    pub fn create_connection(&self, request: CreateConnectionRequest) -> Result<Connection, Error> {
        self.db
            .get_nature(&request.nature)?
            .ok_or(NotFound::Nature(request.nature.clone()))?;

        let connection = Connection::create(request.nature, request.from_ref, request.to_ref);

        let event = Events::Connection(ConnectionEvents::ConnectionCreated(connection.clone()));
        self.log_and_broadcast(&event)?;

        Ok(connection)
    }

    pub fn get_connection(&self, id: &ConnectionId) -> Result<Connection, Error> {
        self.db
            .get_connection(id.to_string())?
            .ok_or_else(|| NotFound::Connection(*id).into())
    }

    pub fn list_connections(
        &self,
        nature: Option<NatureName>,
        entity_ref: Option<&Ref>,
    ) -> Result<Vec<Connection>, Error> {
        let connections = match (nature, entity_ref) {
            (Some(nature), Some(entity_ref)) => {
                self.db
                    .get_nature(&nature)?
                    .ok_or(NotFound::Nature(nature.clone()))?;

                self.db
                    .list_connections_by_ref(entity_ref)?
                    .into_iter()
                    .filter(|c| c.nature == nature)
                    .collect()
            }
            (Some(nature), None) => {
                self.db
                    .get_nature(&nature)?
                    .ok_or(NotFound::Nature(nature.clone()))?;

                self.db.list_connections_by_nature(&nature)?
            }
            (None, Some(entity_ref)) => self.db.list_connections_by_ref(entity_ref)?,
            (None, None) => self.db.list_connections()?,
        };

        Ok(connections)
    }

    pub fn remove_connection(&self, id: &ConnectionId) -> Result<(), Error> {
        self.db
            .get_connection(id.to_string())?
            .ok_or(NotFound::Connection(*id))?;

        let event = Events::Connection(ConnectionEvents::ConnectionRemoved { id: *id });
        self.log_and_broadcast(&event)?;

        Ok(())
    }

    // ── Storage operations ──────────────────────────────────────────

    pub fn set_storage(
        &self,
        key: StorageKey,
        description: &str,
        data: &[u8],
    ) -> Result<StorageEntry, Error> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash_bytes = hasher.finalize();
        let hash_hex = data_encoding::HEXLOWER.encode(&hash_bytes);

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)?;
        let compressed = encoder.finish()?;

        self.db.put_blob(&hash_hex, &compressed, data.len())?;

        let entry = StorageEntry::init(key, description, ContentHash::new(&hash_hex));

        let event = Events::Storage(StorageEvents::StorageSet(entry.clone()));
        self.log_and_broadcast(&event)?;

        Ok(entry)
    }

    pub fn list_storage(&self) -> Result<Vec<StorageEntry>, Error> {
        Ok(self.db.list_storage()?)
    }

    pub fn get_storage(&self, key: &StorageKey) -> Result<StorageEntry, Error> {
        self.db
            .get_storage(key)?
            .ok_or_else(|| NotFound::Storage(key.clone()).into())
    }

    pub fn get_storage_content(&self, key: &StorageKey) -> Result<Vec<u8>, Error> {
        let entry = self
            .db
            .get_storage(key)?
            .ok_or(NotFound::Storage(key.clone()))?;

        let (compressed, _original_size) = self
            .db
            .get_blob(&entry.hash)?
            .ok_or(DataIntegrity::BlobMissing(entry.hash.clone()))?;

        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;

        Ok(decompressed)
    }

    pub fn remove_storage(&self, key: StorageKey) -> Result<(), Error> {
        let event = Events::Storage(StorageEvents::StorageRemoved { key });
        self.log_and_broadcast(&event)?;
        Ok(())
    }

    // ── Search operations ───────────────────────────────────────────

    pub fn search(&self, query: &str, agent: Option<&AgentName>) -> Result<SearchResults, Error> {
        let mut results = self.db.search_expressions(query)?;

        if let Some(agent_name) = agent {
            let agent = self
                .db
                .get_agent(agent_name)?
                .ok_or(NotFound::Agent(agent_name.clone()))?;

            let mut owned_refs: std::collections::HashSet<Ref> = std::collections::HashSet::new();

            owned_refs.insert(Ref::agent(agent.id));

            for id in self.db.list_cognition_ids_by_agent(&agent.id)? {
                owned_refs.insert(Ref::cognition(id));
            }
            for id in self.db.list_memory_ids_by_agent(&agent.id)? {
                owned_refs.insert(Ref::memory(id));
            }
            for id in self.db.list_experience_ids_by_agent(&agent.id)? {
                owned_refs.insert(Ref::experience(id));
            }

            results.retain(|expr| {
                let label = expr.resource_ref.resource().label();
                matches!(
                    label,
                    "persona" | "texture" | "level" | "sensation" | "nature"
                ) || owned_refs.contains(&expr.resource_ref)
            });
        }

        Ok(SearchResults {
            query: query.to_owned(),
            results,
        })
    }

    // ── Sense operations ────────────────────────────────────────────

    pub fn sense(&self, agent_name: &AgentName) -> Result<Agent, Error> {
        let agent = self
            .db
            .get_agent(agent_name)?
            .ok_or(NotFound::Agent(agent_name.clone()))?;

        let event = Events::Sense(SenseEvents::Sensed {
            agent: agent.name.clone(),
        });
        self.log_marker(&event)?;

        Ok(agent)
    }

    // ── Introspect operations ───────────────────────────────────────

    pub fn introspect(&self, agent_name: &AgentName) -> Result<Agent, Error> {
        let agent = self
            .db
            .get_agent(agent_name)?
            .ok_or(NotFound::Agent(agent_name.clone()))?;

        let begun = Events::Introspecting(IntrospectingEvents::IntrospectionBegun {
            agent: agent.name.clone(),
        });
        self.log_marker(&begun)?;

        let complete = Events::Introspecting(IntrospectingEvents::IntrospectionComplete {
            agent: agent.name.clone(),
        });
        self.log_marker(&complete)?;

        Ok(agent)
    }

    // ── Reflect operations ──────────────────────────────────────────

    pub fn reflect(&self, agent_name: &AgentName) -> Result<Agent, Error> {
        let agent = self
            .db
            .get_agent(agent_name)?
            .ok_or(NotFound::Agent(agent_name.clone()))?;

        let begun = Events::Reflecting(ReflectingEvents::ReflectionBegun {
            agent: agent.name.clone(),
        });
        self.log_marker(&begun)?;

        let complete = Events::Reflecting(ReflectingEvents::ReflectionComplete {
            agent: agent.name.clone(),
        });
        self.log_marker(&complete)?;

        Ok(agent)
    }

    // ── Dream operations ────────────────────────────────────────────

    pub fn dream(
        &self,
        agent_name: &AgentName,
        config: DreamConfig,
    ) -> Result<DreamContext, Error> {
        let agent = self
            .db
            .get_agent(agent_name)?
            .ok_or(NotFound::Agent(agent_name.clone()))?;

        let begun = Events::Dreaming(DreamingEvents::DreamBegun {
            agent: agent.name.clone(),
        });
        self.log_marker(&begun)?;

        let context = DreamCollector::new(self.db, config).collect(&agent)?;

        let complete = Events::Dreaming(DreamingEvents::DreamComplete {
            agent: context.agent.clone(),
        });
        self.log_marker(&complete)?;

        Ok(context)
    }

    // ── Lifecycle operations ────────────────────────────────────────

    pub fn emerge(&self, request: CreateAgentRequest) -> Result<Agent, Error> {
        self.db
            .get_persona(&request.persona)?
            .ok_or(NotFound::Persona(request.persona.clone()))?;

        if self.db.agent_name_exists(&request.name)? {
            return Err(Conflicts::Agent(request.name).into());
        }

        let agent_name = request.name.clone();

        let agent = Agent::init(
            request.description,
            request.prompt,
            request.name,
            request.persona,
        );

        let emerged = Events::Lifecycle(LifecycleEvents::Emerged { name: agent_name });
        self.log_marker(&emerged)?;

        let created = Events::Agent(AgentEvents::AgentCreated(agent.clone()));
        self.log_and_broadcast(&created)?;

        Ok(agent)
    }

    pub fn wake(&self, agent_name: &AgentName) -> Result<DreamContext, Error> {
        let agent = self
            .db
            .get_agent(agent_name)?
            .ok_or(NotFound::Agent(agent_name.clone()))?;

        let woke = Events::Lifecycle(LifecycleEvents::Woke {
            name: agent.name.clone(),
        });
        self.log_marker(&woke)?;

        let begun = Events::Dreaming(DreamingEvents::DreamBegun {
            agent: agent.name.clone(),
        });
        self.log_marker(&begun)?;

        let context = DreamCollector::new(self.db, DreamConfig::default()).collect(&agent)?;

        let complete = Events::Dreaming(DreamingEvents::DreamComplete {
            agent: context.agent.clone(),
        });
        self.log_marker(&complete)?;

        Ok(context)
    }

    pub fn sleep(&self, agent_name: &AgentName) -> Result<Agent, Error> {
        let agent = self
            .db
            .get_agent(agent_name)?
            .ok_or(NotFound::Agent(agent_name.clone()))?;

        let slept = Events::Lifecycle(LifecycleEvents::Slept {
            name: agent.name.clone(),
        });
        self.log_marker(&slept)?;

        let begun = Events::Introspecting(IntrospectingEvents::IntrospectionBegun {
            agent: agent.name.clone(),
        });
        self.log_marker(&begun)?;

        let complete = Events::Introspecting(IntrospectingEvents::IntrospectionComplete {
            agent: agent.name.clone(),
        });
        self.log_marker(&complete)?;

        Ok(agent)
    }

    pub fn recede(&self, name: AgentName) -> Result<(), Error> {
        self.db
            .get_agent(&name)?
            .ok_or(NotFound::Agent(name.clone()))?;

        let receded = Events::Lifecycle(LifecycleEvents::Receded { name: name.clone() });
        self.log_marker(&receded)?;

        let removed = Events::Agent(AgentEvents::AgentRemoved { name });
        self.log_and_broadcast(&removed)?;

        Ok(())
    }
}
