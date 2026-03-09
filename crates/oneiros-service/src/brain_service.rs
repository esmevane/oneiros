use oneiros_db::Database;
use oneiros_model::*;
use tokio::sync::broadcast;

use crate::dream_collector::DreamCollector;
use crate::{BadRequests, Error, projections};

/// Domain service for brain-scoped operations.
///
/// Owns the validate → construct → persist → broadcast pipeline.
/// Handlers delegate here; they own only HTTP parsing and response formatting.
pub struct BrainService<'a> {
    db: &'a Database,
    event_tx: &'a broadcast::Sender<Event>,
    source: Source,
}

impl<'a> BrainService<'a> {
    pub fn new(db: &'a Database, event_tx: &'a broadcast::Sender<Event>, source: Source) -> Self {
        Self {
            db,
            event_tx,
            source,
        }
    }

    /// Persist a state-changing event (runs BRAIN projections) then broadcast.
    fn log_and_broadcast(&self, event: &Events) -> Result<Event, Error> {
        let new_event = NewEvent::new(event.clone(), self.source);
        let persisted = self.db.log_event(&new_event, projections::BRAIN)?;
        let _ = self.event_tx.send(persisted.clone());
        Ok(persisted)
    }

    /// Unified dispatch: routes any protocol request to the appropriate domain dispatcher.
    ///
    /// Accepts anything that converts to `Requests`, so callers can pass either
    /// domain-specific request enums (e.g. `AgentRequests`) or the full `Requests`
    /// super-enum directly.
    pub fn dispatch(&self, request: impl Into<Requests>) -> Result<Responses, Error> {
        match request.into() {
            Requests::Agent(r) => Ok(self.dispatch_agent(r)?.into()),
            Requests::Cognition(r) => Ok(self.dispatch_cognition(r)?.into()),
            Requests::Connection(r) => Ok(self.dispatch_connection(r)?.into()),
            Requests::Dreaming(r) => Ok(self.dispatch_dream(r)?.into()),
            Requests::Event(r) => Ok(self.dispatch_event(r)?.into()),
            Requests::Experience(r) => Ok(self.dispatch_experience(r)?.into()),
            Requests::Introspecting(r) => Ok(self.dispatch_introspect(r)?.into()),
            Requests::Level(r) => Ok(self.dispatch_level(r)?.into()),
            Requests::Lifecycle(r) => Ok(self.dispatch_lifecycle(r)?.into()),
            Requests::Memory(r) => Ok(self.dispatch_memory(r)?.into()),
            Requests::Nature(r) => Ok(self.dispatch_nature(r)?.into()),
            Requests::Persona(r) => Ok(self.dispatch_persona(r)?.into()),
            Requests::Reflecting(r) => Ok(self.dispatch_reflect(r)?.into()),
            Requests::Search(r) => Ok(self.dispatch_search(r)?.into()),
            Requests::Sensation(r) => Ok(self.dispatch_sensation(r)?.into()),
            Requests::Sense(r) => Ok(self.dispatch_sense(r)?.into()),
            Requests::Storage(r) => Ok(self.dispatch_storage(r)?.into()),
            Requests::Texture(r) => Ok(self.dispatch_texture(r)?.into()),
            Requests::Brain(_) => {
                Err(BadRequests::NotHandled("brain operations require system service").into())
            }
        }
    }

    /// Persist an observational marker event (no projections) then broadcast.
    fn log_marker(&self, event: &Events) -> Result<Event, Error> {
        let new_event = NewEvent::new(event.clone(), self.source);
        let persisted = self.db.log_event(&new_event, &[])?;
        let _ = self.event_tx.send(persisted.clone());
        Ok(persisted)
    }

    // ── Event operations ──────────────────────────────────────────────

    pub fn dispatch_event(&self, request: EventRequests) -> Result<EventResponses, Error> {
        match request {
            EventRequests::ListEvents(request) => {
                Ok(EventResponses::Listed(self.db.read_events(request.after)?))
            }
            EventRequests::GetEvent(request) => {
                let event = self
                    .db
                    .get_event(&request.id)?
                    .ok_or(NotFound::Event(request.id))?;
                Ok(EventResponses::Found(event))
            }
            EventRequests::ImportEvents(request) => {
                for event in &request.events {
                    let event = event.clone().with_source(self.source);
                    self.db.import_event(&event)?;
                }

                let replayed = self.db.replay(projections::BRAIN)?;

                Ok(EventResponses::Imported(ImportResponse {
                    imported: request.events.len(),
                    replayed,
                }))
            }
            EventRequests::ReplayEvents(_) => {
                let count = self.db.replay(projections::BRAIN)?;
                Ok(EventResponses::Replayed(ReplayResponse { replayed: count }))
            }
            EventRequests::ExportEvents(_) => {
                Ok(EventResponses::Exported(self.db.read_events(None)?))
            }
        }
    }

    pub fn event_count(&self) -> Result<usize, Error> {
        Ok(self.db.event_count()?)
    }

    // ── Agent operations ──────────────────────────────────────────────

    pub fn dispatch_agent(&self, request: AgentRequests) -> Result<AgentResponses, Error> {
        match request {
            AgentRequests::CreateAgent(request) => {
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
            AgentRequests::ListAgents(_) => {
                Ok(AgentResponses::AgentsListed(self.db.list_agents()?))
            }
            AgentRequests::GetAgent(request) => {
                let agent = self
                    .db
                    .get_agent(&request.name)?
                    .ok_or(NotFound::Agent(request.name))?;
                Ok(AgentResponses::AgentFound(agent))
            }
            AgentRequests::UpdateAgent(request) => {
                let existing = self
                    .db
                    .get_agent(&request.name)?
                    .ok_or(NotFound::Agent(request.name.clone()))?;

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
            AgentRequests::RemoveAgent(request) => {
                let event = Events::Agent(AgentEvents::AgentRemoved(SelectAgentByName {
                    name: request.name,
                }));
                self.log_and_broadcast(&event)?;

                Ok(AgentResponses::AgentRemoved)
            }
        }
    }

    // ── Cognition operations ──────────────────────────────────────────

    pub fn dispatch_cognition(
        &self,
        request: CognitionRequests,
    ) -> Result<CognitionResponses, Error> {
        match request {
            CognitionRequests::AddCognition(request) => {
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
            CognitionRequests::GetCognition(request) => {
                let cognition = self
                    .db
                    .get_cognition(request.id.to_string())?
                    .ok_or(NotFound::Cognition(request.id))?;
                Ok(CognitionResponses::CognitionFound(cognition))
            }
            CognitionRequests::ListCognitions(request) => {
                let cognitions = match (request.agent, request.texture) {
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
        }
    }

    // ── Level operations ────────────────────────────────────────────

    pub fn dispatch_level(&self, request: LevelRequests) -> Result<LevelResponses, Error> {
        match request {
            LevelRequests::SetLevel(level) => {
                let event = Events::Level(LevelEvents::LevelSet(level.clone()));
                self.log_and_broadcast(&event)?;
                Ok(LevelResponses::LevelSet(level))
            }
            LevelRequests::ListLevels(_) => {
                Ok(LevelResponses::LevelsListed(self.db.list_levels()?))
            }
            LevelRequests::GetLevel(request) => {
                let level = self
                    .db
                    .get_level(&request.name)?
                    .ok_or(NotFound::Level(request.name))?;
                Ok(LevelResponses::LevelFound(level))
            }
            LevelRequests::RemoveLevel(request) => {
                let event = Events::Level(LevelEvents::LevelRemoved(SelectLevelByName {
                    name: request.name,
                }));
                self.log_and_broadcast(&event)?;
                Ok(LevelResponses::LevelRemoved)
            }
        }
    }

    // ── Nature operations ───────────────────────────────────────────

    pub fn dispatch_nature(&self, request: NatureRequests) -> Result<NatureResponses, Error> {
        match request {
            NatureRequests::SetNature(nature) => {
                let event = Events::Nature(NatureEvents::NatureSet(nature.clone()));
                self.log_and_broadcast(&event)?;
                Ok(NatureResponses::NatureSet(nature))
            }
            NatureRequests::ListNatures(_) => {
                Ok(NatureResponses::NaturesListed(self.db.list_natures()?))
            }
            NatureRequests::GetNature(request) => {
                let nature = self
                    .db
                    .get_nature(&request.name)?
                    .ok_or(NotFound::Nature(request.name))?;
                Ok(NatureResponses::NatureFound(nature))
            }
            NatureRequests::RemoveNature(request) => {
                let event = Events::Nature(NatureEvents::NatureRemoved(SelectNatureByName {
                    name: request.name,
                }));
                self.log_and_broadcast(&event)?;
                Ok(NatureResponses::NatureRemoved)
            }
        }
    }

    // ── Persona operations ──────────────────────────────────────────

    pub fn dispatch_persona(&self, request: PersonaRequests) -> Result<PersonaResponses, Error> {
        match request {
            PersonaRequests::SetPersona(persona) => {
                let event = Events::Persona(PersonaEvents::PersonaSet(persona.clone()));
                self.log_and_broadcast(&event)?;
                Ok(PersonaResponses::PersonaSet(persona))
            }
            PersonaRequests::ListPersonas(_) => {
                Ok(PersonaResponses::PersonasListed(self.db.list_personas()?))
            }
            PersonaRequests::GetPersona(request) => {
                let persona = self
                    .db
                    .get_persona(&request.name)?
                    .ok_or(NotFound::Persona(request.name))?;
                Ok(PersonaResponses::PersonaFound(persona))
            }
            PersonaRequests::RemovePersona(request) => {
                let event = Events::Persona(PersonaEvents::PersonaRemoved(SelectPersonaByName {
                    name: request.name,
                }));
                self.log_and_broadcast(&event)?;
                Ok(PersonaResponses::PersonaRemoved)
            }
        }
    }

    // ── Sensation operations ────────────────────────────────────────

    pub fn dispatch_sensation(
        &self,
        request: SensationRequests,
    ) -> Result<SensationResponses, Error> {
        match request {
            SensationRequests::SetSensation(sensation) => {
                let event = Events::Sensation(SensationEvents::SensationSet(sensation.clone()));
                self.log_and_broadcast(&event)?;
                Ok(SensationResponses::SensationSet(sensation))
            }
            SensationRequests::ListSensations(_) => Ok(SensationResponses::SensationsListed(
                self.db.list_sensations()?,
            )),
            SensationRequests::GetSensation(request) => {
                let sensation = self
                    .db
                    .get_sensation(&request.name)?
                    .ok_or(NotFound::Sensation(request.name))?;
                Ok(SensationResponses::SensationFound(sensation))
            }
            SensationRequests::RemoveSensation(request) => {
                let event =
                    Events::Sensation(SensationEvents::SensationRemoved(SelectSensationByName {
                        name: request.name,
                    }));
                self.log_and_broadcast(&event)?;
                Ok(SensationResponses::SensationRemoved)
            }
        }
    }

    // ── Texture operations ──────────────────────────────────────────

    pub fn dispatch_texture(&self, request: TextureRequests) -> Result<TextureResponses, Error> {
        match request {
            TextureRequests::SetTexture(texture) => {
                let event = Events::Texture(TextureEvents::TextureSet(texture.clone()));
                self.log_and_broadcast(&event)?;
                Ok(TextureResponses::TextureSet(texture))
            }
            TextureRequests::ListTextures(_) => {
                Ok(TextureResponses::TexturesListed(self.db.list_textures()?))
            }
            TextureRequests::GetTexture(request) => {
                let texture = self
                    .db
                    .get_texture(&request.name)?
                    .ok_or(NotFound::Texture(request.name))?;
                Ok(TextureResponses::TextureFound(texture))
            }
            TextureRequests::RemoveTexture(request) => {
                let event = Events::Texture(TextureEvents::TextureRemoved(SelectTextureByName {
                    name: request.name,
                }));
                self.log_and_broadcast(&event)?;
                Ok(TextureResponses::TextureRemoved)
            }
        }
    }

    // ── Memory operations ───────────────────────────────────────────

    pub fn dispatch_memory(&self, request: MemoryRequests) -> Result<MemoryResponses, Error> {
        match request {
            MemoryRequests::AddMemory(request) => {
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
            MemoryRequests::GetMemory(request) => {
                let memory = self
                    .db
                    .get_memory(request.id.to_string())?
                    .ok_or(NotFound::Memory(request.id))?;
                Ok(MemoryResponses::MemoryFound(memory))
            }
            MemoryRequests::ListMemories(request) => {
                let memories = match (request.agent, request.level) {
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
        }
    }

    // ── Experience operations ───────────────────────────────────────

    pub fn dispatch_experience(
        &self,
        request: ExperienceRequests,
    ) -> Result<ExperienceResponses, Error> {
        match request {
            ExperienceRequests::CreateExperience(request) => {
                let agent = self
                    .db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                self.db
                    .get_sensation(&request.sensation)?
                    .ok_or(NotFound::Sensation(request.sensation.clone()))?;

                let experience =
                    Experience::create(agent.id, request.sensation, request.description);

                let event =
                    Events::Experience(ExperienceEvents::ExperienceCreated(experience.clone()));
                self.log_and_broadcast(&event)?;

                Ok(ExperienceResponses::ExperienceCreated(experience))
            }
            ExperienceRequests::GetExperience(request) => {
                let experience = self
                    .db
                    .get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;
                Ok(ExperienceResponses::ExperienceFound(experience))
            }
            ExperienceRequests::ListExperiences(request) => {
                let experiences = match (request.agent, request.sensation) {
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
            ExperienceRequests::UpdateExperienceDescription(request) => {
                self.db
                    .get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;

                let event = Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated(
                    ExperienceDescriptionUpdate {
                        experience_id: request.id,
                        description: request.description,
                    },
                ));

                self.log_and_broadcast(&event)?;

                let experience = self
                    .db
                    .get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;
                Ok(ExperienceResponses::ExperienceUpdated(experience))
            }
            ExperienceRequests::UpdateExperienceSensation(request) => {
                self.db
                    .get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;

                let event = Events::Experience(ExperienceEvents::ExperienceSensationUpdated(
                    ExperienceSensationUpdate {
                        experience_id: request.id,
                        sensation: request.sensation,
                    },
                ));

                self.log_and_broadcast(&event)?;

                let experience = self
                    .db
                    .get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;
                Ok(ExperienceResponses::ExperienceUpdated(experience))
            }
        }
    }

    // ── Connection operations ───────────────────────────────────────

    pub fn dispatch_connection(
        &self,
        request: ConnectionRequests,
    ) -> Result<ConnectionResponses, Error> {
        match request {
            ConnectionRequests::CreateConnection(request) => {
                self.db
                    .get_nature(&request.nature)?
                    .ok_or(NotFound::Nature(request.nature.clone()))?;

                let connection =
                    Connection::create(request.nature, request.from_ref, request.to_ref);

                let event =
                    Events::Connection(ConnectionEvents::ConnectionCreated(connection.clone()));
                self.log_and_broadcast(&event)?;

                Ok(ConnectionResponses::ConnectionCreated(connection))
            }
            ConnectionRequests::GetConnection(request) => {
                let connection = self
                    .db
                    .get_connection(request.id.to_string())?
                    .ok_or(NotFound::Connection(request.id))?;
                Ok(ConnectionResponses::ConnectionFound(connection))
            }
            ConnectionRequests::ListConnections(request) => {
                let connections = match (request.nature, request.entity_ref) {
                    (Some(nature), Some(ref_token)) => {
                        self.db
                            .get_nature(&nature)?
                            .ok_or(NotFound::Nature(nature.clone()))?;

                        self.db
                            .list_connections_by_ref(ref_token.inner())?
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
                    (None, Some(ref_token)) => {
                        self.db.list_connections_by_ref(ref_token.inner())?
                    }
                    (None, None) => self.db.list_connections()?,
                };

                Ok(ConnectionResponses::ConnectionsListed(connections))
            }
            ConnectionRequests::RemoveConnection(request) => {
                self.db
                    .get_connection(request.id.to_string())?
                    .ok_or(NotFound::Connection(request.id))?;

                let event =
                    Events::Connection(ConnectionEvents::ConnectionRemoved(SelectConnectionById {
                        id: request.id,
                    }));
                self.log_and_broadcast(&event)?;

                Ok(ConnectionResponses::ConnectionRemoved)
            }
        }
    }

    // ── Storage operations ──────────────────────────────────────────

    pub fn set_storage(&self, request: SetStorageRequest) -> Result<StorageResponses, Error> {
        let blob_content = BlobContent::create(&request.data)?;

        self.db.put_blob(&blob_content)?;

        let entry = StorageEntry::init(request.key, &request.description, blob_content.hash);
        let event = Events::Storage(StorageEvents::StorageSet(entry.clone()));

        self.log_and_broadcast(&event)?;

        Ok(StorageResponses::StorageSet(entry))
    }

    pub fn dispatch_storage(&self, request: StorageRequests) -> Result<StorageResponses, Error> {
        match request {
            StorageRequests::ListStorage(_) => {
                Ok(StorageResponses::StorageListed(self.db.list_storage()?))
            }
            StorageRequests::GetStorage(request) => {
                let entry = self
                    .db
                    .get_storage(&request.key)?
                    .ok_or(NotFound::Storage(request.key))?;
                Ok(StorageResponses::StorageFound(entry))
            }
            StorageRequests::GetStorageContent(request) => {
                let entry = self
                    .db
                    .get_storage(&request.key)?
                    .ok_or(NotFound::Storage(request.key.clone()))?;

                let blob = self
                    .db
                    .get_blob(&entry.hash)?
                    .ok_or(DataIntegrity::BlobMissing(entry.hash.clone()))?;

                let _decompressed = blob.data.decompressed()?;

                // Content retrieval returns through StorageResponses for dispatch uniformity,
                // but the HTTP handler still returns raw bytes. This is the outlier.
                Ok(StorageResponses::StorageFound(entry))
            }
            StorageRequests::RemoveStorage(request) => {
                let event = Events::Storage(StorageEvents::StorageRemoved(SelectStorageByKey {
                    key: request.key,
                }));
                self.log_and_broadcast(&event)?;
                Ok(StorageResponses::StorageRemoved)
            }
        }
    }

    /// Get raw storage content bytes. This bypasses the dispatch enum because
    /// it returns raw bytes rather than a protocol response.
    pub fn get_storage_content(&self, key: &StorageKey) -> Result<Vec<u8>, Error> {
        let entry = self
            .db
            .get_storage(key)?
            .ok_or(NotFound::Storage(key.clone()))?;

        let blob = self
            .db
            .get_blob(&entry.hash)?
            .ok_or(DataIntegrity::BlobMissing(entry.hash.clone()))?;

        let decompressed = blob.data.decompressed()?;

        Ok(decompressed)
    }

    // ── Search operations ───────────────────────────────────────────

    pub fn dispatch_search(&self, request: SearchRequests) -> Result<SearchResponses, Error> {
        match request {
            SearchRequests::Search(request) => {
                let mut results = self.db.search_expressions(&request.query)?;

                if let Some(agent_name) = &request.agent {
                    let agent = self
                        .db
                        .get_agent(agent_name)?
                        .ok_or(NotFound::Agent(agent_name.clone()))?;

                    let mut owned_refs: std::collections::HashSet<Ref> =
                        std::collections::HashSet::new();

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
                    query: request.query,
                    results,
                }))
            }
        }
    }

    // ── Sense operations ────────────────────────────────────────────

    pub fn dispatch_sense(&self, request: SenseRequests) -> Result<SenseResponses, Error> {
        match request {
            SenseRequests::Sense(request) => {
                let agent = self
                    .db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let event = Events::Sense(SenseEvents::Sensed(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                self.log_marker(&event)?;

                Ok(SenseResponses::Sensed(agent))
            }
        }
    }

    // ── Introspect operations ───────────────────────────────────────

    pub fn dispatch_introspect(
        &self,
        request: IntrospectingRequests,
    ) -> Result<IntrospectingResponses, Error> {
        match request {
            IntrospectingRequests::Introspect(request) => {
                let agent = self
                    .db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let begun = Events::Introspecting(IntrospectingEvents::IntrospectionBegun(
                    SelectAgentByName {
                        name: agent.name.clone(),
                    },
                ));
                self.log_marker(&begun)?;

                let complete = Events::Introspecting(IntrospectingEvents::IntrospectionComplete(
                    SelectAgentByName {
                        name: agent.name.clone(),
                    },
                ));
                self.log_marker(&complete)?;

                Ok(IntrospectingResponses::IntrospectionComplete(agent))
            }
        }
    }

    // ── Reflect operations ──────────────────────────────────────────

    pub fn dispatch_reflect(
        &self,
        request: ReflectingRequests,
    ) -> Result<ReflectingResponses, Error> {
        match request {
            ReflectingRequests::Reflect(request) => {
                let agent = self
                    .db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let begun =
                    Events::Reflecting(ReflectingEvents::ReflectionBegun(SelectAgentByName {
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
        }
    }

    // ── Dream operations ────────────────────────────────────────────

    pub fn dispatch_dream(&self, request: DreamingRequests) -> Result<DreamingResponses, Error> {
        match request {
            DreamingRequests::Dream(request) => {
                let agent = self
                    .db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let begun = Events::Dreaming(DreamingEvents::DreamBegun(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                self.log_marker(&begun)?;

                let context = DreamCollector::new(self.db, request.config).collect(&agent)?;

                let complete =
                    Events::Dreaming(DreamingEvents::DreamComplete(DreamCompleteEvent {
                        agent: context.agent.clone(),
                    }));
                self.log_marker(&complete)?;

                Ok(DreamingResponses::DreamComplete(Box::new(context)))
            }
        }
    }

    // ── Lifecycle operations ────────────────────────────────────────

    pub fn dispatch_lifecycle(
        &self,
        request: LifecycleRequests,
    ) -> Result<LifecycleResponses, Error> {
        match request {
            LifecycleRequests::Emerge(request) => {
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
            LifecycleRequests::Wake(request) => {
                let agent = self
                    .db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent))?;

                let woke = Events::Lifecycle(LifecycleEvents::Woke(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                self.log_marker(&woke)?;

                let begun = Events::Dreaming(DreamingEvents::DreamBegun(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                self.log_marker(&begun)?;

                let context =
                    DreamCollector::new(self.db, DreamConfig::default()).collect(&agent)?;

                let complete =
                    Events::Dreaming(DreamingEvents::DreamComplete(DreamCompleteEvent {
                        agent: context.agent.clone(),
                    }));
                self.log_marker(&complete)?;

                Ok(LifecycleResponses::Woke(Box::new(context)))
            }
            LifecycleRequests::Sleep(request) => {
                let agent = self
                    .db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent))?;

                let slept = Events::Lifecycle(LifecycleEvents::Slept(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                self.log_marker(&slept)?;

                let begun = Events::Introspecting(IntrospectingEvents::IntrospectionBegun(
                    SelectAgentByName {
                        name: agent.name.clone(),
                    },
                ));
                self.log_marker(&begun)?;

                let complete = Events::Introspecting(IntrospectingEvents::IntrospectionComplete(
                    SelectAgentByName {
                        name: agent.name.clone(),
                    },
                ));
                self.log_marker(&complete)?;

                Ok(LifecycleResponses::Slept(agent))
            }
            LifecycleRequests::Recede(request) => {
                self.db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let receded = Events::Lifecycle(LifecycleEvents::Receded(SelectAgentByName {
                    name: request.agent.clone(),
                }));
                self.log_marker(&receded)?;

                let removed = Events::Agent(AgentEvents::AgentRemoved(SelectAgentByName {
                    name: request.agent,
                }));
                self.log_and_broadcast(&removed)?;

                Ok(LifecycleResponses::Receded)
            }
        }
    }
}
