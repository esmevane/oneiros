use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use oneiros_db::Database;
use oneiros_model::*;
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use tokio::sync::broadcast;

use crate::dream_collector::DreamCollector;
use crate::{Error, projections};

/// Domain service for brain-scoped operations.
///
/// Owns the validate → construct → persist → broadcast pipeline.
/// Handlers delegate here; they own only HTTP parsing and response formatting.
pub struct BrainService<'a> {
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

    // ── Event operations ──────────────────────────────────────────────

    pub fn read_events(&self) -> Result<Vec<Event>, Error> {
        Ok(self.db.read_events()?)
    }

    pub fn get_event(&self, id: &EventId) -> Result<Event, Error> {
        self.db.get_event(id)?.ok_or(NotFound::Event(*id).into())
    }

    pub fn import_events(&self, events: &[ImportEvent]) -> Result<ImportResponse, Error> {
        for event in events {
            self.db.import_event(&event.timestamp, &event.data)?;
        }

        let replayed = self.db.replay(projections::BRAIN)?;

        Ok(ImportResponse {
            imported: events.len(),
            replayed,
        })
    }

    pub fn replay(&self) -> Result<ReplayResponse, Error> {
        let count = self.db.replay(projections::BRAIN)?;
        Ok(ReplayResponse { replayed: count })
    }

    pub fn event_count(&self) -> Result<usize, Error> {
        Ok(self.db.event_count()?)
    }

    // ── Agent operations ──────────────────────────────────────────────

    pub fn create_agent(&self, request: CreateAgentRequest) -> Result<AgentResponses, Error> {
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

        Ok(AgentResponses::AgentCreated(agent))
    }

    pub fn list_agents(&self) -> Result<AgentResponses, Error> {
        Ok(AgentResponses::AgentsListed(self.db.list_agents()?))
    }

    pub fn get_agent(&self, name: &AgentName) -> Result<AgentResponses, Error> {
        let agent = self
            .db
            .get_agent(name)?
            .ok_or(NotFound::Agent(name.clone()))?;
        Ok(AgentResponses::AgentFound(agent))
    }

    pub fn update_agent(
        &self,
        name: &AgentName,
        request: UpdateAgentRequest,
    ) -> Result<AgentResponses, Error> {
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

        Ok(AgentResponses::AgentUpdated(agent))
    }

    pub fn remove_agent(&self, name: AgentName) -> Result<AgentResponses, Error> {
        let event = Events::Agent(AgentEvents::AgentRemoved(SelectAgentByName { name }));
        self.log_and_broadcast(&event)?;

        Ok(AgentResponses::AgentRemoved)
    }

    // ── Cognition operations ──────────────────────────────────────────

    pub fn add_cognition(&self, request: AddCognitionRequest) -> Result<CognitionResponses, Error> {
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

        Ok(CognitionResponses::CognitionAdded(cognition))
    }

    pub fn get_cognition(&self, id: &CognitionId) -> Result<CognitionResponses, Error> {
        let cognition = self
            .db
            .get_cognition(id.to_string())?
            .ok_or(NotFound::Cognition(*id))?;
        Ok(CognitionResponses::CognitionFound(cognition))
    }

    pub fn list_cognitions(
        &self,
        agent: Option<AgentName>,
        texture: Option<TextureName>,
    ) -> Result<CognitionResponses, Error> {
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

        Ok(CognitionResponses::CognitionsListed(cognitions))
    }

    // ── Level operations ────────────────────────────────────────────

    pub fn set_level(&self, level: Level) -> Result<LevelResponses, Error> {
        let event = Events::Level(LevelEvents::LevelSet(level.clone()));
        self.log_and_broadcast(&event)?;
        Ok(LevelResponses::LevelSet(level))
    }

    pub fn list_levels(&self) -> Result<LevelResponses, Error> {
        Ok(LevelResponses::LevelsListed(self.db.list_levels()?))
    }

    pub fn get_level(&self, name: &LevelName) -> Result<LevelResponses, Error> {
        let level = self
            .db
            .get_level(name)?
            .ok_or(NotFound::Level(name.clone()))?;
        Ok(LevelResponses::LevelFound(level))
    }

    pub fn remove_level(&self, name: LevelName) -> Result<LevelResponses, Error> {
        let event = Events::Level(LevelEvents::LevelRemoved(SelectLevelByName { name }));
        self.log_and_broadcast(&event)?;
        Ok(LevelResponses::LevelRemoved)
    }

    // ── Nature operations ───────────────────────────────────────────

    pub fn set_nature(&self, nature: Nature) -> Result<NatureResponses, Error> {
        let event = Events::Nature(NatureEvents::NatureSet(nature.clone()));
        self.log_and_broadcast(&event)?;
        Ok(NatureResponses::NatureSet(nature))
    }

    pub fn list_natures(&self) -> Result<NatureResponses, Error> {
        Ok(NatureResponses::NaturesListed(self.db.list_natures()?))
    }

    pub fn get_nature(&self, name: &NatureName) -> Result<NatureResponses, Error> {
        let nature = self
            .db
            .get_nature(name)?
            .ok_or(NotFound::Nature(name.clone()))?;
        Ok(NatureResponses::NatureFound(nature))
    }

    pub fn remove_nature(&self, name: NatureName) -> Result<NatureResponses, Error> {
        let event = Events::Nature(NatureEvents::NatureRemoved(SelectNatureByName { name }));
        self.log_and_broadcast(&event)?;
        Ok(NatureResponses::NatureRemoved)
    }

    // ── Persona operations ──────────────────────────────────────────

    pub fn set_persona(&self, persona: Persona) -> Result<PersonaResponses, Error> {
        let event = Events::Persona(PersonaEvents::PersonaSet(persona.clone()));
        self.log_and_broadcast(&event)?;
        Ok(PersonaResponses::PersonaSet(persona))
    }

    pub fn list_personas(&self) -> Result<PersonaResponses, Error> {
        Ok(PersonaResponses::PersonasListed(self.db.list_personas()?))
    }

    pub fn get_persona(&self, name: &PersonaName) -> Result<PersonaResponses, Error> {
        let persona = self
            .db
            .get_persona(name)?
            .ok_or(NotFound::Persona(name.clone()))?;
        Ok(PersonaResponses::PersonaFound(persona))
    }

    pub fn remove_persona(&self, name: PersonaName) -> Result<PersonaResponses, Error> {
        let event = Events::Persona(PersonaEvents::PersonaRemoved(SelectPersonaByName { name }));
        self.log_and_broadcast(&event)?;
        Ok(PersonaResponses::PersonaRemoved)
    }

    // ── Sensation operations ────────────────────────────────────────

    pub fn set_sensation(&self, sensation: Sensation) -> Result<SensationResponses, Error> {
        let event = Events::Sensation(SensationEvents::SensationSet(sensation.clone()));
        self.log_and_broadcast(&event)?;
        Ok(SensationResponses::SensationSet(sensation))
    }

    pub fn list_sensations(&self) -> Result<SensationResponses, Error> {
        Ok(SensationResponses::SensationsListed(
            self.db.list_sensations()?,
        ))
    }

    pub fn get_sensation(&self, name: &SensationName) -> Result<SensationResponses, Error> {
        let sensation = self
            .db
            .get_sensation(name)?
            .ok_or(NotFound::Sensation(name.clone()))?;
        Ok(SensationResponses::SensationFound(sensation))
    }

    pub fn remove_sensation(&self, name: SensationName) -> Result<SensationResponses, Error> {
        let event = Events::Sensation(SensationEvents::SensationRemoved(SelectSensationByName {
            name,
        }));
        self.log_and_broadcast(&event)?;
        Ok(SensationResponses::SensationRemoved)
    }

    // ── Texture operations ──────────────────────────────────────────

    pub fn set_texture(&self, texture: Texture) -> Result<TextureResponses, Error> {
        let event = Events::Texture(TextureEvents::TextureSet(texture.clone()));
        self.log_and_broadcast(&event)?;
        Ok(TextureResponses::TextureSet(texture))
    }

    pub fn list_textures(&self) -> Result<TextureResponses, Error> {
        Ok(TextureResponses::TexturesListed(self.db.list_textures()?))
    }

    pub fn get_texture(&self, name: &TextureName) -> Result<TextureResponses, Error> {
        let texture = self
            .db
            .get_texture(name)?
            .ok_or(NotFound::Texture(name.clone()))?;
        Ok(TextureResponses::TextureFound(texture))
    }

    pub fn remove_texture(&self, name: TextureName) -> Result<TextureResponses, Error> {
        let event = Events::Texture(TextureEvents::TextureRemoved(SelectTextureByName { name }));
        self.log_and_broadcast(&event)?;
        Ok(TextureResponses::TextureRemoved)
    }

    // ── Memory operations ───────────────────────────────────────────

    pub fn add_memory(&self, request: AddMemoryRequest) -> Result<MemoryResponses, Error> {
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

        Ok(MemoryResponses::MemoryAdded(memory))
    }

    pub fn get_memory(&self, id: &MemoryId) -> Result<MemoryResponses, Error> {
        let memory = self
            .db
            .get_memory(id.to_string())?
            .ok_or(NotFound::Memory(*id))?;
        Ok(MemoryResponses::MemoryFound(memory))
    }

    pub fn list_memories(
        &self,
        agent: Option<AgentName>,
        level: Option<LevelName>,
    ) -> Result<MemoryResponses, Error> {
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

        Ok(MemoryResponses::MemoriesListed(memories))
    }

    // ── Experience operations ───────────────────────────────────────

    pub fn create_experience(
        &self,
        request: CreateExperienceRequest,
    ) -> Result<ExperienceResponses, Error> {
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

        Ok(ExperienceResponses::ExperienceCreated(experience))
    }

    pub fn get_experience(&self, id: &ExperienceId) -> Result<ExperienceResponses, Error> {
        let experience = self
            .db
            .get_experience(id.to_string())?
            .ok_or(NotFound::Experience(*id))?;
        Ok(ExperienceResponses::ExperienceFound(experience))
    }

    pub fn list_experiences(
        &self,
        agent: Option<AgentName>,
        sensation: Option<SensationName>,
    ) -> Result<ExperienceResponses, Error> {
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

        Ok(ExperienceResponses::ExperiencesListed(experiences))
    }

    pub fn update_experience_description(
        &self,
        id: &ExperienceId,
        request: UpdateExperienceDescriptionRequest,
    ) -> Result<ExperienceResponses, Error> {
        self.db
            .get_experience(id.to_string())?
            .ok_or(NotFound::Experience(*id))?;

        let event = Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated(
            ExperienceDescriptionUpdate {
                experience_id: *id,
                description: request.description,
            },
        ));

        self.log_and_broadcast(&event)?;

        // Re-fetch to include projection updates.
        let experience = self
            .db
            .get_experience(id.to_string())?
            .ok_or(NotFound::Experience(*id))?;
        Ok(ExperienceResponses::ExperienceUpdated(experience))
    }

    pub fn update_experience_sensation(
        &self,
        id: &ExperienceId,
        request: UpdateExperienceSensationRequest,
    ) -> Result<ExperienceResponses, Error> {
        self.db
            .get_experience(id.to_string())?
            .ok_or(NotFound::Experience(*id))?;

        let event = Events::Experience(ExperienceEvents::ExperienceSensationUpdated(
            ExperienceSensationUpdate {
                experience_id: *id,
                sensation: request.sensation,
            },
        ));

        self.log_and_broadcast(&event)?;

        // Re-fetch to include projection updates.
        let experience = self
            .db
            .get_experience(id.to_string())?
            .ok_or(NotFound::Experience(*id))?;
        Ok(ExperienceResponses::ExperienceUpdated(experience))
    }

    // ── Connection operations ───────────────────────────────────────

    pub fn create_connection(
        &self,
        request: CreateConnectionRequest,
    ) -> Result<ConnectionResponses, Error> {
        self.db
            .get_nature(&request.nature)?
            .ok_or(NotFound::Nature(request.nature.clone()))?;

        let connection = Connection::create(request.nature, request.from_ref, request.to_ref);

        let event = Events::Connection(ConnectionEvents::ConnectionCreated(connection.clone()));
        self.log_and_broadcast(&event)?;

        Ok(ConnectionResponses::ConnectionCreated(connection))
    }

    pub fn get_connection(&self, id: &ConnectionId) -> Result<ConnectionResponses, Error> {
        let connection = self
            .db
            .get_connection(id.to_string())?
            .ok_or(NotFound::Connection(*id))?;
        Ok(ConnectionResponses::ConnectionFound(connection))
    }

    pub fn list_connections(
        &self,
        nature: Option<NatureName>,
        entity_ref: Option<&Ref>,
    ) -> Result<ConnectionResponses, Error> {
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

        Ok(ConnectionResponses::ConnectionsListed(connections))
    }

    pub fn remove_connection(&self, id: &ConnectionId) -> Result<ConnectionResponses, Error> {
        self.db
            .get_connection(id.to_string())?
            .ok_or(NotFound::Connection(*id))?;

        let event = Events::Connection(ConnectionEvents::ConnectionRemoved(SelectConnectionById {
            id: *id,
        }));
        self.log_and_broadcast(&event)?;

        Ok(ConnectionResponses::ConnectionRemoved)
    }

    // ── Storage operations ──────────────────────────────────────────

    pub fn set_storage(
        &self,
        key: StorageKey,
        description: &str,
        data: &[u8],
    ) -> Result<StorageResponses, Error> {
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

        Ok(StorageResponses::StorageSet(entry))
    }

    pub fn list_storage(&self) -> Result<StorageResponses, Error> {
        Ok(StorageResponses::StorageListed(self.db.list_storage()?))
    }

    pub fn get_storage(&self, key: &StorageKey) -> Result<StorageResponses, Error> {
        let entry = self
            .db
            .get_storage(key)?
            .ok_or(NotFound::Storage(key.clone()))?;
        Ok(StorageResponses::StorageFound(entry))
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

    pub fn remove_storage(&self, key: StorageKey) -> Result<StorageResponses, Error> {
        let event = Events::Storage(StorageEvents::StorageRemoved(SelectStorageByKey { key }));
        self.log_and_broadcast(&event)?;
        Ok(StorageResponses::StorageRemoved)
    }

    // ── Search operations ───────────────────────────────────────────

    pub fn search(&self, query: &str, agent: Option<&AgentName>) -> Result<SearchResponses, Error> {
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

        Ok(SearchResponses::SearchComplete(SearchResults {
            query: query.to_owned(),
            results,
        }))
    }

    // ── Sense operations ────────────────────────────────────────────

    pub fn sense(&self, agent_name: &AgentName) -> Result<SenseResponses, Error> {
        let agent = self
            .db
            .get_agent(agent_name)?
            .ok_or(NotFound::Agent(agent_name.clone()))?;

        let event = Events::Sense(SenseEvents::Sensed(SelectAgentByName {
            name: agent.name.clone(),
        }));
        self.log_marker(&event)?;

        Ok(SenseResponses::Sensed(agent))
    }

    // ── Introspect operations ───────────────────────────────────────

    pub fn introspect(&self, agent_name: &AgentName) -> Result<IntrospectingResponses, Error> {
        let agent = self
            .db
            .get_agent(agent_name)?
            .ok_or(NotFound::Agent(agent_name.clone()))?;

        let begun =
            Events::Introspecting(IntrospectingEvents::IntrospectionBegun(SelectAgentByName {
                name: agent.name.clone(),
            }));
        self.log_marker(&begun)?;

        let complete = Events::Introspecting(IntrospectingEvents::IntrospectionComplete(
            SelectAgentByName {
                name: agent.name.clone(),
            },
        ));
        self.log_marker(&complete)?;

        Ok(IntrospectingResponses::IntrospectionComplete(agent))
    }

    // ── Reflect operations ──────────────────────────────────────────

    pub fn reflect(&self, agent_name: &AgentName) -> Result<ReflectingResponses, Error> {
        let agent = self
            .db
            .get_agent(agent_name)?
            .ok_or(NotFound::Agent(agent_name.clone()))?;

        let begun = Events::Reflecting(ReflectingEvents::ReflectionBegun(SelectAgentByName {
            name: agent.name.clone(),
        }));
        self.log_marker(&begun)?;

        let complete =
            Events::Reflecting(ReflectingEvents::ReflectionComplete(SelectAgentByName {
                name: agent.name.clone(),
            }));
        self.log_marker(&complete)?;

        Ok(ReflectingResponses::ReflectionComplete(agent))
    }

    // ── Dream operations ────────────────────────────────────────────

    pub fn dream(
        &self,
        agent_name: &AgentName,
        config: DreamConfig,
    ) -> Result<DreamingResponses, Error> {
        let agent = self
            .db
            .get_agent(agent_name)?
            .ok_or(NotFound::Agent(agent_name.clone()))?;

        let begun = Events::Dreaming(DreamingEvents::DreamBegun(SelectAgentByName {
            name: agent.name.clone(),
        }));
        self.log_marker(&begun)?;

        let context = DreamCollector::new(self.db, config).collect(&agent)?;

        let complete = Events::Dreaming(DreamingEvents::DreamComplete(DreamCompleteEvent {
            agent: context.agent.clone(),
        }));
        self.log_marker(&complete)?;

        Ok(DreamingResponses::DreamComplete(Box::new(context)))
    }

    // ── Lifecycle operations ────────────────────────────────────────

    pub fn emerge(&self, request: CreateAgentRequest) -> Result<LifecycleResponses, Error> {
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

        let emerged = Events::Lifecycle(LifecycleEvents::Emerged(SelectAgentByName {
            name: agent_name,
        }));
        self.log_marker(&emerged)?;

        let created = Events::Agent(AgentEvents::AgentCreated(agent.clone()));
        self.log_and_broadcast(&created)?;

        Ok(LifecycleResponses::Emerged(agent))
    }

    pub fn wake(&self, agent_name: &AgentName) -> Result<LifecycleResponses, Error> {
        let agent = self
            .db
            .get_agent(agent_name)?
            .ok_or(NotFound::Agent(agent_name.clone()))?;

        let woke = Events::Lifecycle(LifecycleEvents::Woke(SelectAgentByName {
            name: agent.name.clone(),
        }));
        self.log_marker(&woke)?;

        let begun = Events::Dreaming(DreamingEvents::DreamBegun(SelectAgentByName {
            name: agent.name.clone(),
        }));
        self.log_marker(&begun)?;

        let context = DreamCollector::new(self.db, DreamConfig::default()).collect(&agent)?;

        let complete = Events::Dreaming(DreamingEvents::DreamComplete(DreamCompleteEvent {
            agent: context.agent.clone(),
        }));
        self.log_marker(&complete)?;

        Ok(LifecycleResponses::Woke(Box::new(context)))
    }

    pub fn sleep(&self, agent_name: &AgentName) -> Result<LifecycleResponses, Error> {
        let agent = self
            .db
            .get_agent(agent_name)?
            .ok_or(NotFound::Agent(agent_name.clone()))?;

        let slept = Events::Lifecycle(LifecycleEvents::Slept(SelectAgentByName {
            name: agent.name.clone(),
        }));
        self.log_marker(&slept)?;

        let begun =
            Events::Introspecting(IntrospectingEvents::IntrospectionBegun(SelectAgentByName {
                name: agent.name.clone(),
            }));
        self.log_marker(&begun)?;

        let complete = Events::Introspecting(IntrospectingEvents::IntrospectionComplete(
            SelectAgentByName {
                name: agent.name.clone(),
            },
        ));
        self.log_marker(&complete)?;

        Ok(LifecycleResponses::Slept(agent))
    }

    pub fn recede(&self, name: AgentName) -> Result<LifecycleResponses, Error> {
        self.db
            .get_agent(&name)?
            .ok_or(NotFound::Agent(name.clone()))?;

        let receded = Events::Lifecycle(LifecycleEvents::Receded(SelectAgentByName {
            name: name.clone(),
        }));
        self.log_marker(&receded)?;

        let removed = Events::Agent(AgentEvents::AgentRemoved(SelectAgentByName { name }));
        self.log_and_broadcast(&removed)?;

        Ok(LifecycleResponses::Receded)
    }
}
