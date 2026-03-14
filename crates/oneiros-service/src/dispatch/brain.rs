use oneiros_model::*;

use crate::*;

impl OneirosService {
    // ── Event operations ──────────────────────────────────────────────

    pub fn dispatch_event(&self, request: EventRequests) -> Result<EventResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            EventRequests::ListEvents(request) => {
                Ok(EventResponses::Listed(db.read_events(request.after)?))
            }
            EventRequests::GetEvent(request) => {
                let event = db
                    .get_event(&request.id)?
                    .ok_or(NotFound::Event(request.id))?;
                Ok(EventResponses::Found(event))
            }
            EventRequests::ImportEvents(request) => {
                for event in &request.events {
                    let event = event.clone().with_source(self.source());
                    db.import_event(&event)?;
                }

                let replayed = db.replay(projections::BRAIN)?;

                Ok(EventResponses::Imported(ImportResponse {
                    imported: request.events.len(),
                    replayed,
                }))
            }
            EventRequests::ReplayEvents(_) => {
                let count = db.replay(projections::BRAIN)?;
                Ok(EventResponses::Replayed(ReplayResponse { replayed: count }))
            }
            EventRequests::ExportEvents(_) => Ok(EventResponses::Exported(db.read_events(None)?)),
        }
    }

    // ── Agent operations ──────────────────────────────────────────────

    pub fn dispatch_agent(&self, request: AgentRequests) -> Result<AgentResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            AgentRequests::CreateAgent(request) => {
                db.get_persona(&request.persona)?
                    .ok_or(NotFound::Persona(request.persona.clone()))?;

                if db.agent_name_exists(&request.name)? {
                    return Err(Conflicts::Agent(request.name).into());
                }

                let agent = Agent::init(
                    request.description,
                    request.prompt,
                    request.name,
                    request.persona,
                );

                let event = Events::Agent(AgentEvents::AgentCreated(agent.clone()));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;

                Ok(AgentResponses::AgentCreated(agent))
            }
            AgentRequests::ListAgents(_) => Ok(AgentResponses::AgentsListed(db.list_agents()?)),
            AgentRequests::GetAgent(request) => {
                let agent = db
                    .get_agent(&request.name)?
                    .ok_or(NotFound::Agent(request.name))?;
                Ok(AgentResponses::AgentFound(agent))
            }
            AgentRequests::UpdateAgent(request) => {
                let existing = db
                    .get_agent(&request.name)?
                    .ok_or(NotFound::Agent(request.name.clone()))?;

                db.get_persona(&request.persona)?
                    .ok_or(NotFound::Persona(request.persona.clone()))?;

                let agent = Agent::construct(
                    existing.id,
                    request.description,
                    request.prompt,
                    existing.name.clone(),
                    request.persona,
                );

                let event = Events::Agent(AgentEvents::AgentUpdated(agent.clone()));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;

                Ok(AgentResponses::AgentUpdated(agent))
            }
            AgentRequests::RemoveAgent(request) => {
                let event = Events::Agent(AgentEvents::AgentRemoved(SelectAgentByName {
                    name: request.name,
                }));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;

                Ok(AgentResponses::AgentRemoved)
            }
        }
    }

    // ── Cognition operations ──────────────────────────────────────────

    pub fn dispatch_cognition(
        &self,
        request: CognitionRequests,
    ) -> Result<CognitionResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            CognitionRequests::AddCognition(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                db.get_texture(&request.texture)?
                    .ok_or(NotFound::Texture(request.texture.clone()))?;

                let cognition = Cognition::create(agent.id, request.texture, request.content);

                let event = Events::Cognition(CognitionEvents::CognitionAdded(cognition.clone()));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;

                Ok(CognitionResponses::CognitionAdded(cognition))
            }
            CognitionRequests::GetCognition(request) => {
                let cognition = db
                    .get_cognition(request.id.to_string())?
                    .ok_or(NotFound::Cognition(request.id))?;
                Ok(CognitionResponses::CognitionFound(cognition))
            }
            CognitionRequests::ListCognitions(request) => {
                let cognitions = match (request.agent, request.texture) {
                    (Some(agent_name), Some(texture)) => {
                        let agent = db
                            .get_agent(&agent_name)?
                            .ok_or(NotFound::Agent(agent_name))?;

                        db.get_texture(&texture)?
                            .ok_or(NotFound::Texture(texture.clone()))?;

                        db.list_cognitions_by_agent_and_texture(agent.id.to_string(), &texture)?
                    }
                    (Some(agent_name), None) => {
                        let agent = db
                            .get_agent(&agent_name)?
                            .ok_or(NotFound::Agent(agent_name))?;

                        db.list_cognitions_by_agent(agent.id.to_string())?
                    }
                    (None, Some(texture)) => {
                        db.get_texture(&texture)?
                            .ok_or(NotFound::Texture(texture.clone()))?;

                        db.list_cognitions_by_texture(&texture)?
                    }
                    (None, None) => db.list_cognitions()?,
                };

                Ok(CognitionResponses::CognitionsListed(cognitions))
            }
        }
    }

    // ── Nature operations ─────────────────────────────────────────────

    pub fn dispatch_nature(&self, request: NatureRequests) -> Result<NatureResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            NatureRequests::SetNature(nature) => {
                let event = Events::Nature(NatureEvents::NatureSet(nature.clone()));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;
                Ok(NatureResponses::NatureSet(nature))
            }
            NatureRequests::ListNatures(_) => {
                Ok(NatureResponses::NaturesListed(db.list_natures()?))
            }
            NatureRequests::GetNature(request) => {
                let nature = db
                    .get_nature(&request.name)?
                    .ok_or(NotFound::Nature(request.name))?;
                Ok(NatureResponses::NatureFound(nature))
            }
            NatureRequests::RemoveNature(request) => {
                let event = Events::Nature(NatureEvents::NatureRemoved(SelectNatureByName {
                    name: request.name,
                }));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;
                Ok(NatureResponses::NatureRemoved)
            }
        }
    }

    // ── Persona operations ────────────────────────────────────────────

    pub fn dispatch_persona(&self, request: PersonaRequests) -> Result<PersonaResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            PersonaRequests::SetPersona(persona) => {
                let event = Events::Persona(PersonaEvents::PersonaSet(persona.clone()));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;
                Ok(PersonaResponses::PersonaSet(persona))
            }
            PersonaRequests::ListPersonas(_) => {
                Ok(PersonaResponses::PersonasListed(db.list_personas()?))
            }
            PersonaRequests::GetPersona(request) => {
                let persona = db
                    .get_persona(&request.name)?
                    .ok_or(NotFound::Persona(request.name))?;
                Ok(PersonaResponses::PersonaFound(persona))
            }
            PersonaRequests::RemovePersona(request) => {
                let event = Events::Persona(PersonaEvents::PersonaRemoved(SelectPersonaByName {
                    name: request.name,
                }));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;
                Ok(PersonaResponses::PersonaRemoved)
            }
        }
    }

    // ── Sensation operations ──────────────────────────────────────────

    pub fn dispatch_sensation(
        &self,
        request: SensationRequests,
    ) -> Result<SensationResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            SensationRequests::SetSensation(sensation) => {
                let event = Events::Sensation(SensationEvents::SensationSet(sensation.clone()));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;
                Ok(SensationResponses::SensationSet(sensation))
            }
            SensationRequests::ListSensations(_) => {
                Ok(SensationResponses::SensationsListed(db.list_sensations()?))
            }
            SensationRequests::GetSensation(request) => {
                let sensation = db
                    .get_sensation(&request.name)?
                    .ok_or(NotFound::Sensation(request.name))?;
                Ok(SensationResponses::SensationFound(sensation))
            }
            SensationRequests::RemoveSensation(request) => {
                let event =
                    Events::Sensation(SensationEvents::SensationRemoved(SelectSensationByName {
                        name: request.name,
                    }));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;
                Ok(SensationResponses::SensationRemoved)
            }
        }
    }

    // ── Texture operations ────────────────────────────────────────────

    pub fn dispatch_texture(&self, request: TextureRequests) -> Result<TextureResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            TextureRequests::SetTexture(texture) => {
                let event = Events::Texture(TextureEvents::TextureSet(texture.clone()));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;
                Ok(TextureResponses::TextureSet(texture))
            }
            TextureRequests::ListTextures(_) => {
                Ok(TextureResponses::TexturesListed(db.list_textures()?))
            }
            TextureRequests::GetTexture(request) => {
                let texture = db
                    .get_texture(&request.name)?
                    .ok_or(NotFound::Texture(request.name))?;
                Ok(TextureResponses::TextureFound(texture))
            }
            TextureRequests::RemoveTexture(request) => {
                let event = Events::Texture(TextureEvents::TextureRemoved(SelectTextureByName {
                    name: request.name,
                }));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;
                Ok(TextureResponses::TextureRemoved)
            }
        }
    }

    // ── Pressure operations ───────────────────────────────────────────

    pub fn dispatch_pressure(&self, request: PressureRequests) -> Result<PressureResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            PressureRequests::GetPressure(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let pressures = db.list_pressures_for_agent(&agent.id.to_string())?;

                Ok(PressureResponses::PressureFound(pressures))
            }
            PressureRequests::ListPressures(_) => {
                let agents = db.list_agents()?;
                let mut pressures = Vec::new();

                for agent in agents {
                    let mut agent_pressures = db.list_pressures_for_agent(&agent.id.to_string())?;

                    pressures.append(&mut agent_pressures);
                }

                Ok(PressureResponses::PressuresListed(pressures))
            }
        }
    }

    // ── Urge operations ───────────────────────────────────────────────

    pub fn dispatch_urge(&self, request: UrgeRequests) -> Result<UrgeResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            UrgeRequests::SetUrge(urge) => {
                let event = Events::Urge(UrgeEvents::UrgeSet(urge.clone()));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;
                Ok(UrgeResponses::UrgeSet(urge))
            }
            UrgeRequests::ListUrges(_) => Ok(UrgeResponses::UrgesListed(db.list_urges()?)),
            UrgeRequests::GetUrge(request) => {
                let urge = db
                    .get_urge(&request.name)?
                    .ok_or(NotFound::Urge(request.name))?;
                Ok(UrgeResponses::UrgeFound(urge))
            }
            UrgeRequests::RemoveUrge(request) => {
                let event = Events::Urge(UrgeEvents::UrgeRemoved(SelectUrgeByName {
                    name: request.name,
                }));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;
                Ok(UrgeResponses::UrgeRemoved)
            }
        }
    }

    // ── Memory operations ─────────────────────────────────────────────

    pub fn dispatch_memory(&self, request: MemoryRequests) -> Result<MemoryResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            MemoryRequests::AddMemory(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                db.get_level(&request.level)?
                    .ok_or(NotFound::Level(request.level.clone()))?;

                let memory = Memory::create(agent.id, request.level, request.content);

                let event = Events::Memory(MemoryEvents::MemoryAdded(memory.clone()));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;

                Ok(MemoryResponses::MemoryAdded(memory))
            }
            MemoryRequests::GetMemory(request) => {
                let memory = db
                    .get_memory(request.id.to_string())?
                    .ok_or(NotFound::Memory(request.id))?;
                Ok(MemoryResponses::MemoryFound(memory))
            }
            MemoryRequests::ListMemories(request) => {
                let memories = match (request.agent, request.level) {
                    (Some(agent_name), Some(level)) => {
                        let agent = db
                            .get_agent(&agent_name)?
                            .ok_or(NotFound::Agent(agent_name))?;

                        db.get_level(&level)?
                            .ok_or(NotFound::Level(level.clone()))?;

                        db.list_memories_by_agent_and_level(agent.id.to_string(), &level)?
                    }
                    (Some(agent_name), None) => {
                        let agent = db
                            .get_agent(&agent_name)?
                            .ok_or(NotFound::Agent(agent_name))?;

                        db.list_memories_by_agent(agent.id.to_string())?
                    }
                    (None, Some(level)) => {
                        db.get_level(&level)?
                            .ok_or(NotFound::Level(level.clone()))?;

                        db.list_memories_by_level(&level)?
                    }
                    (None, None) => db.list_memories()?,
                };

                Ok(MemoryResponses::MemoriesListed(memories))
            }
        }
    }

    // ── Experience operations ─────────────────────────────────────────

    pub fn dispatch_experience(
        &self,
        request: ExperienceRequests,
    ) -> Result<ExperienceResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            ExperienceRequests::CreateExperience(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                db.get_sensation(&request.sensation)?
                    .ok_or(NotFound::Sensation(request.sensation.clone()))?;

                let experience =
                    Experience::create(agent.id, request.sensation, request.description);

                let event =
                    Events::Experience(ExperienceEvents::ExperienceCreated(experience.clone()));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;

                Ok(ExperienceResponses::ExperienceCreated(experience))
            }
            ExperienceRequests::GetExperience(request) => {
                let experience = db
                    .get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;
                Ok(ExperienceResponses::ExperienceFound(experience))
            }
            ExperienceRequests::ListExperiences(request) => {
                let experiences = match (request.agent, request.sensation) {
                    (Some(agent_name), Some(sensation)) => {
                        let agent = db
                            .get_agent(&agent_name)?
                            .ok_or(NotFound::Agent(agent_name))?;

                        db.get_sensation(&sensation)?
                            .ok_or(NotFound::Sensation(sensation.clone()))?;

                        db.list_experiences_by_agent(agent.id.to_string())?
                            .into_iter()
                            .filter(|exp| exp.sensation == sensation)
                            .collect()
                    }
                    (Some(agent_name), None) => {
                        let agent = db
                            .get_agent(&agent_name)?
                            .ok_or(NotFound::Agent(agent_name))?;

                        db.list_experiences_by_agent(agent.id.to_string())?
                    }
                    (None, Some(sensation)) => {
                        db.get_sensation(&sensation)?
                            .ok_or(NotFound::Sensation(sensation.clone()))?;

                        db.list_experiences_by_sensation(&sensation)?
                    }
                    (None, None) => db.list_experiences()?,
                };

                Ok(ExperienceResponses::ExperiencesListed(experiences))
            }
            ExperienceRequests::UpdateExperienceDescription(request) => {
                db.get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;

                let event = Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated(
                    ExperienceDescriptionUpdate {
                        experience_id: request.id,
                        description: request.description,
                    },
                ));

                self.log_and_broadcast(&db, &event, projections::BRAIN)?;

                let experience = db
                    .get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;
                Ok(ExperienceResponses::ExperienceUpdated(experience))
            }
            ExperienceRequests::UpdateExperienceSensation(request) => {
                db.get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;

                let event = Events::Experience(ExperienceEvents::ExperienceSensationUpdated(
                    ExperienceSensationUpdate {
                        experience_id: request.id,
                        sensation: request.sensation,
                    },
                ));

                self.log_and_broadcast(&db, &event, projections::BRAIN)?;

                let experience = db
                    .get_experience(request.id.to_string())?
                    .ok_or(NotFound::Experience(request.id))?;
                Ok(ExperienceResponses::ExperienceUpdated(experience))
            }
        }
    }

    // ── Connection operations ─────────────────────────────────────────

    pub fn dispatch_connection(
        &self,
        request: ConnectionRequests,
    ) -> Result<ConnectionResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            ConnectionRequests::CreateConnection(request) => {
                db.get_nature(&request.nature)?
                    .ok_or(NotFound::Nature(request.nature.clone()))?;

                let connection =
                    Connection::create(request.nature, request.from_ref, request.to_ref);

                let event =
                    Events::Connection(ConnectionEvents::ConnectionCreated(connection.clone()));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;

                Ok(ConnectionResponses::ConnectionCreated(connection))
            }
            ConnectionRequests::GetConnection(request) => {
                let connection = db
                    .get_connection(request.id.to_string())?
                    .ok_or(NotFound::Connection(request.id))?;
                Ok(ConnectionResponses::ConnectionFound(connection))
            }
            ConnectionRequests::ListConnections(request) => {
                let connections = match (request.nature, request.entity_ref) {
                    (Some(nature), Some(ref_token)) => {
                        db.get_nature(&nature)?
                            .ok_or(NotFound::Nature(nature.clone()))?;

                        db.list_connections_by_ref(ref_token.inner())?
                            .into_iter()
                            .filter(|c| c.nature == nature)
                            .collect()
                    }
                    (Some(nature), None) => {
                        db.get_nature(&nature)?
                            .ok_or(NotFound::Nature(nature.clone()))?;

                        db.list_connections_by_nature(&nature)?
                    }
                    (None, Some(ref_token)) => db.list_connections_by_ref(ref_token.inner())?,
                    (None, None) => db.list_connections()?,
                };

                Ok(ConnectionResponses::ConnectionsListed(connections))
            }
            ConnectionRequests::RemoveConnection(request) => {
                db.get_connection(request.id.to_string())?
                    .ok_or(NotFound::Connection(request.id))?;

                let event =
                    Events::Connection(ConnectionEvents::ConnectionRemoved(SelectConnectionById {
                        id: request.id,
                    }));
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;

                Ok(ConnectionResponses::ConnectionRemoved)
            }
        }
    }

    // ── Storage dispatch ──────────────────────────────────────────────

    pub fn dispatch_storage(&self, request: StorageRequests) -> Result<StorageResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            StorageRequests::ListStorage(_) => {
                Ok(StorageResponses::StorageListed(db.list_storage()?))
            }
            StorageRequests::GetStorage(request) => {
                let entry = db
                    .get_storage(&request.key)?
                    .ok_or(NotFound::Storage(request.key))?;
                Ok(StorageResponses::StorageFound(entry))
            }
            StorageRequests::GetStorageContent(request) => {
                let entry = db
                    .get_storage(&request.key)?
                    .ok_or(NotFound::Storage(request.key.clone()))?;

                let blob = db
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
                self.log_and_broadcast(&db, &event, projections::BRAIN)?;
                Ok(StorageResponses::StorageRemoved)
            }
        }
    }

    // ── Search operations ─────────────────────────────────────────────

    pub fn dispatch_search(&self, request: SearchRequests) -> Result<SearchResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            SearchRequests::Search(request) => {
                let mut results = db.search_expressions(&request.query)?;

                if let Some(agent_name) = &request.agent {
                    let agent = db
                        .get_agent(agent_name)?
                        .ok_or(NotFound::Agent(agent_name.clone()))?;

                    let mut owned_refs: std::collections::HashSet<Ref> =
                        std::collections::HashSet::new();

                    owned_refs.insert(Ref::agent(agent.id));

                    for id in db.list_cognition_ids_by_agent(&agent.id)? {
                        owned_refs.insert(Ref::cognition(id));
                    }
                    for id in db.list_memory_ids_by_agent(&agent.id)? {
                        owned_refs.insert(Ref::memory(id));
                    }
                    for id in db.list_experience_ids_by_agent(&agent.id)? {
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

    // ── Sense operations ──────────────────────────────────────────────

    pub fn dispatch_sense(&self, request: SenseRequests) -> Result<SenseResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            SenseRequests::Sense(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let event = Events::Sense(SenseEvents::Sensed(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                self.log_marker(&db, &event)?;

                Ok(SenseResponses::Sensed(agent))
            }
        }
    }

    // ── Introspect operations ─────────────────────────────────────────

    pub fn dispatch_introspect(
        &self,
        request: IntrospectingRequests,
    ) -> Result<IntrospectingResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            IntrospectingRequests::Introspect(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let begun = Events::Introspecting(IntrospectingEvents::IntrospectionBegun(
                    SelectAgentByName {
                        name: agent.name.clone(),
                    },
                ));
                self.log_marker(&db, &begun)?;

                let complete = Events::Introspecting(IntrospectingEvents::IntrospectionComplete(
                    SelectAgentByName {
                        name: agent.name.clone(),
                    },
                ));
                self.log_marker(&db, &complete)?;

                Ok(IntrospectingResponses::IntrospectionComplete(agent))
            }
        }
    }

    // ── Reflect operations ────────────────────────────────────────────

    pub fn dispatch_reflect(
        &self,
        request: ReflectingRequests,
    ) -> Result<ReflectingResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            ReflectingRequests::Reflect(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let begun =
                    Events::Reflecting(ReflectingEvents::ReflectionBegun(SelectAgentByName {
                        name: agent.name.clone(),
                    }));
                self.log_marker(&db, &begun)?;

                let complete =
                    Events::Reflecting(ReflectingEvents::ReflectionComplete(SelectAgentByName {
                        name: agent.name.clone(),
                    }));
                self.log_marker(&db, &complete)?;

                Ok(ReflectingResponses::ReflectionComplete(agent))
            }
        }
    }

    // ── Dream operations ──────────────────────────────────────────────

    pub fn dispatch_dream(&self, request: DreamingRequests) -> Result<DreamingResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            DreamingRequests::Dream(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let begun = Events::Dreaming(DreamingEvents::DreamBegun(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                self.log_marker(&db, &begun)?;

                let context = DreamCollector::new(&db, request.config).collect(&agent)?;

                let complete =
                    Events::Dreaming(DreamingEvents::DreamComplete(DreamCompleteEvent {
                        agent: context.agent.clone(),
                    }));
                self.log_marker(&db, &complete)?;

                Ok(DreamingResponses::DreamComplete(Box::new(context)))
            }
        }
    }

    // ── Lifecycle operations ──────────────────────────────────────────

    pub fn dispatch_lifecycle(
        &self,
        request: LifecycleRequests,
    ) -> Result<LifecycleResponses, Error> {
        let db = self.lock_brain()?;
        match request {
            LifecycleRequests::Emerge(request) => {
                db.get_persona(&request.persona)?
                    .ok_or(NotFound::Persona(request.persona.clone()))?;

                if db.agent_name_exists(&request.name)? {
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
                self.log_marker(&db, &emerged)?;

                let created = Events::Agent(AgentEvents::AgentCreated(agent.clone()));
                self.log_and_broadcast(&db, &created, projections::BRAIN)?;

                Ok(LifecycleResponses::Emerged(agent))
            }
            LifecycleRequests::Wake(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent))?;

                let woke = Events::Lifecycle(LifecycleEvents::Woke(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                self.log_marker(&db, &woke)?;

                let begun = Events::Dreaming(DreamingEvents::DreamBegun(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                self.log_marker(&db, &begun)?;

                let context = DreamCollector::new(&db, DreamConfig::default()).collect(&agent)?;

                let complete =
                    Events::Dreaming(DreamingEvents::DreamComplete(DreamCompleteEvent {
                        agent: context.agent.clone(),
                    }));
                self.log_marker(&db, &complete)?;

                Ok(LifecycleResponses::Woke(Box::new(context)))
            }
            LifecycleRequests::Sleep(request) => {
                let agent = db
                    .get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent))?;

                let slept = Events::Lifecycle(LifecycleEvents::Slept(SelectAgentByName {
                    name: agent.name.clone(),
                }));
                self.log_marker(&db, &slept)?;

                let begun = Events::Introspecting(IntrospectingEvents::IntrospectionBegun(
                    SelectAgentByName {
                        name: agent.name.clone(),
                    },
                ));
                self.log_marker(&db, &begun)?;

                let complete = Events::Introspecting(IntrospectingEvents::IntrospectionComplete(
                    SelectAgentByName {
                        name: agent.name.clone(),
                    },
                ));
                self.log_marker(&db, &complete)?;

                Ok(LifecycleResponses::Slept(agent))
            }
            LifecycleRequests::Recede(request) => {
                db.get_agent(&request.agent)?
                    .ok_or(NotFound::Agent(request.agent.clone()))?;

                let receded = Events::Lifecycle(LifecycleEvents::Receded(SelectAgentByName {
                    name: request.agent.clone(),
                }));
                self.log_marker(&db, &receded)?;

                let removed = Events::Agent(AgentEvents::AgentRemoved(SelectAgentByName {
                    name: request.agent,
                }));
                self.log_and_broadcast(&db, &removed, projections::BRAIN)?;

                Ok(LifecycleResponses::Receded)
            }
        }
    }
}
