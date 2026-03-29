//! Pressure repo — cross-domain derived state.
//!
//! Pressure is computed from queries across cognition, memory, and events.
//! The projection gathers inputs and computes urgency via the gauge system.

use rusqlite::params;

use crate::*;

pub struct PressureRepo<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> PressureRepo<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    /// Recompute pressure for the agent associated with this event.
    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        let agent_name_str = match self.resolve_agent_name(event) {
            Some(name) => name,
            None => return Ok(()),
        };

        let agent_name = AgentName::new(&agent_name_str);

        // Look up the agent by name
        let agent = match AgentRepo::new(self.conn).get(&agent_name)? {
            Some(a) => a,
            None => return Ok(()),
        };

        // Get all urges
        let urges = UrgeRepo::new(self.conn).list()?;
        let now = Timestamp::now();

        for urge in &urges {
            let gauge = self.compute_gauge(&urge.name, &agent.id, &agent.name)?;
            self.upsert(&agent.id, &urge.name, &gauge, &now)?;
        }

        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM pressures", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS pressures (
                id TEXT PRIMARY KEY,
                agent_id TEXT NOT NULL,
                urge TEXT NOT NULL,
                data TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                UNIQUE(agent_id, urge)
            )",
        )?;
        Ok(())
    }

    pub fn get(&self, agent_name: &AgentName) -> Result<Vec<Pressure>, EventError> {
        // Look up agent by name to get the ID
        let agent = match AgentRepo::new(self.conn).get(agent_name)? {
            Some(a) => a,
            None => return Ok(vec![]),
        };

        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, urge, data, updated_at FROM pressures WHERE agent_id = ?1 ORDER BY urge",
        )?;

        let pressures = stmt
            .query_map(params![agent.id.to_string()], |row| {
                let data_str: String = row.get(3)?;
                let updated_str: String = row.get(4)?;
                Ok(Pressure {
                    id: row
                        .get::<_, String>(0)?
                        .parse()
                        .unwrap_or_else(|_| PressureId::new()),
                    agent_id: row
                        .get::<_, String>(1)?
                        .parse()
                        .unwrap_or_else(|_| AgentId::new()),
                    urge: UrgeName::new(row.get::<_, String>(2)?),
                    data: serde_json::from_str(&data_str).unwrap_or(Gauge::Introspect(
                        IntrospectGauge::from_inputs(IntrospectInputs {
                            hours_since_last_introspect: 0.0,
                            total_cognitions: 0,
                            working_cognitions: 0,
                            cognitions_since_introspect: 0,
                            memories_since_introspect: 0,
                            session_cognition_count: 0,
                        }),
                    )),
                    updated_at: Timestamp::parse_str(&updated_str)
                        .unwrap_or_else(|_| Timestamp::now()),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(pressures)
    }

    pub fn list(&self) -> Result<Vec<Pressure>, EventError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, urge, data, updated_at FROM pressures ORDER BY agent_id, urge",
        )?;

        let pressures = stmt
            .query_map([], |row| {
                let data_str: String = row.get(3)?;
                let updated_str: String = row.get(4)?;
                Ok(Pressure {
                    id: row
                        .get::<_, String>(0)?
                        .parse()
                        .unwrap_or_else(|_| PressureId::new()),
                    agent_id: row
                        .get::<_, String>(1)?
                        .parse()
                        .unwrap_or_else(|_| AgentId::new()),
                    urge: UrgeName::new(row.get::<_, String>(2)?),
                    data: serde_json::from_str(&data_str).unwrap_or(Gauge::Introspect(
                        IntrospectGauge::from_inputs(IntrospectInputs {
                            hours_since_last_introspect: 0.0,
                            total_cognitions: 0,
                            working_cognitions: 0,
                            cognitions_since_introspect: 0,
                            memories_since_introspect: 0,
                            session_cognition_count: 0,
                        }),
                    )),
                    updated_at: Timestamp::parse_str(&updated_str)
                        .unwrap_or_else(|_| Timestamp::now()),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(pressures)
    }

    // ── Private helpers ──────────────────────────────────────────────

    fn resolve_agent_name(&self, event: &StoredEvent) -> Option<String> {
        match &event.data {
            Events::Cognition(CognitionEvents::CognitionAdded(c)) => {
                // Look up agent name by agent_id (UUID) from the agents table
                self.conn
                    .query_row(
                        "SELECT name FROM agents WHERE id = ?1",
                        params![c.agent_id.to_string()],
                        |row| row.get::<_, String>(0),
                    )
                    .ok()
            }
            Events::Memory(MemoryEvents::MemoryAdded(m)) => self
                .conn
                .query_row(
                    "SELECT name FROM agents WHERE id = ?1",
                    params![m.agent_id.to_string()],
                    |row| row.get::<_, String>(0),
                )
                .ok(),
            Events::Continuity(ContinuityEvents::Introspected(a)) => Some(a.agent.to_string()),
            Events::Continuity(ContinuityEvents::Reflected(a)) => Some(a.agent.to_string()),
            Events::Continuity(ContinuityEvents::Dreamed(a)) => Some(a.agent.to_string()),
            _ => None,
        }
    }

    fn compute_gauge(
        &self,
        urge: &UrgeName,
        agent_id: &AgentId,
        agent_name: &AgentName,
    ) -> Result<Gauge, EventError> {
        let urge_str = urge.to_string();
        match urge_str.as_str() {
            "introspect" => Ok(Gauge::Introspect(
                self.compute_introspect(agent_id, agent_name)?,
            )),
            "catharsis" => Ok(Gauge::Catharsis(
                self.compute_catharsis(agent_id, agent_name)?,
            )),
            "recollect" => Ok(Gauge::Recollect(
                self.compute_recollect(agent_id, agent_name)?,
            )),
            "retrospect" => Ok(Gauge::Retrospect(
                self.compute_retrospect(agent_id, agent_name)?,
            )),
            _ => Ok(Gauge::Introspect(IntrospectGauge::from_inputs(
                IntrospectInputs {
                    hours_since_last_introspect: 0.0,
                    total_cognitions: 0,
                    working_cognitions: 0,
                    cognitions_since_introspect: 0,
                    memories_since_introspect: 0,
                    session_cognition_count: 0,
                },
            ))),
        }
    }

    fn compute_introspect(
        &self,
        agent_id: &AgentId,
        agent_name: &AgentName,
    ) -> Result<IntrospectGauge, EventError> {
        let agent_id_str = agent_id.to_string();
        let agent_name_str = agent_name.to_string();

        // Hours since last introspection event
        let hours_since =
            self.hours_since_lifecycle_event(&agent_name_str, "introspection-complete");

        // Total cognitions for this agent
        let total_cognitions: u64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM cognitions WHERE agent_id = ?1",
                params![agent_id_str],
                |row| row.get(0),
            )
            .unwrap_or(0);

        // Working-texture cognitions
        let working_cognitions: u64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM cognitions WHERE agent_id = ?1 AND texture = 'working'",
                params![agent_id_str],
                |row| row.get(0),
            )
            .unwrap_or(0);

        // Cognitions since last introspection
        let last_introspect_time =
            self.last_lifecycle_time(&agent_name_str, "introspection-complete");
        let cognitions_since: u64 = match &last_introspect_time {
            Some(t) => self
                .conn
                .query_row(
                    "SELECT COUNT(*) FROM cognitions WHERE agent_id = ?1 AND created_at > ?2",
                    params![agent_id_str, t],
                    |row| row.get(0),
                )
                .unwrap_or(total_cognitions),
            None => total_cognitions,
        };

        // Memories since last introspection
        let memories_since: u64 = match &last_introspect_time {
            Some(t) => self
                .conn
                .query_row(
                    "SELECT COUNT(*) FROM memories WHERE agent_id = ?1 AND created_at > ?2",
                    params![agent_id_str, t],
                    |row| row.get(0),
                )
                .unwrap_or(0),
            None => self
                .conn
                .query_row(
                    "SELECT COUNT(*) FROM memories WHERE agent_id = ?1",
                    params![agent_id_str],
                    |row| row.get(0),
                )
                .unwrap_or(0),
        };

        // Session cognitions (since last wake)
        let last_wake_time = self.last_lifecycle_time(&agent_name_str, "woke");
        let session_cognition_count: u64 = match &last_wake_time {
            Some(t) => self
                .conn
                .query_row(
                    "SELECT COUNT(*) FROM cognitions WHERE agent_id = ?1 AND created_at > ?2",
                    params![agent_id_str, t],
                    |row| row.get(0),
                )
                .unwrap_or(total_cognitions),
            None => total_cognitions,
        };

        let inputs = IntrospectInputs {
            hours_since_last_introspect: hours_since,
            total_cognitions,
            working_cognitions,
            cognitions_since_introspect: cognitions_since,
            memories_since_introspect: memories_since,
            session_cognition_count,
        };

        Ok(IntrospectGauge::from_inputs(inputs))
    }

    fn compute_catharsis(
        &self,
        agent_id: &AgentId,
        agent_name: &AgentName,
    ) -> Result<CatharsisGauge, EventError> {
        let agent_id_str = agent_id.to_string();
        let agent_name_str = agent_name.to_string();

        // Count tensions experiences
        let tensions_count: u64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM experiences WHERE agent_id = ?1 AND sensation = 'tensions'",
                params![agent_id_str],
                |row| row.get(0),
            )
            .unwrap_or(0);

        // Working cognitions as fraction of total (stale working)
        let total_cognitions: u64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM cognitions WHERE agent_id = ?1",
                params![agent_id_str],
                |row| row.get(0),
            )
            .unwrap_or(0);

        let working_cognitions: u64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM cognitions WHERE agent_id = ?1 AND texture = 'working'",
                params![agent_id_str],
                |row| row.get(0),
            )
            .unwrap_or(0);

        let _stale_working = if total_cognitions > 0 {
            working_cognitions as f64 / total_cognitions as f64
        } else {
            0.0
        };

        // Hours since last reflection
        let hours_since_reflect =
            self.hours_since_lifecycle_event(&agent_name_str, "reflection-complete");

        // Orphaned cognitions (not referenced by any experience)
        let orphaned: u64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM cognitions c WHERE c.agent_id = ?1 AND NOT EXISTS (SELECT 1 FROM connections WHERE source_ref LIKE '%' || c.id || '%' OR target_ref LIKE '%' || c.id || '%')",
                params![agent_id_str],
                |row| row.get(0),
            )
            .unwrap_or(total_cognitions);

        let _orphaned_fraction = if total_cognitions > 0 {
            orphaned as f64 / total_cognitions as f64
        } else {
            0.0
        };

        let inputs = CatharsisInputs {
            tensions_experience_count: tensions_count,
            total_cognitions,
            working_cognitions,
            hours_since_last_reflect: hours_since_reflect,
            orphaned_cognitions: orphaned,
        };

        Ok(CatharsisGauge::from_inputs(inputs))
    }

    fn compute_recollect(
        &self,
        agent_id: &AgentId,
        agent_name: &AgentName,
    ) -> Result<RecollectGauge, EventError> {
        let agent_id_str = agent_id.to_string();
        let _agent_name_str = agent_name.to_string();

        // Session-level memories
        let session_memories: u64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM memories WHERE agent_id = ?1 AND level = 'session'",
                params![agent_id_str],
                |row| row.get(0),
            )
            .unwrap_or(0);

        // Unconnected experiences
        let total_experiences: u64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM experiences WHERE agent_id = ?1",
                params![agent_id_str],
                |row| row.get(0),
            )
            .unwrap_or(0);

        let connected_experiences: u64 = self
            .conn
            .query_row(
                "SELECT COUNT(DISTINCT e.id) FROM experiences e INNER JOIN connections c ON c.source_ref LIKE '%' || e.id || '%' OR c.target_ref LIKE '%' || e.id || '%' WHERE e.agent_id = ?1",
                params![agent_id_str],
                |row| row.get(0),
            )
            .unwrap_or(0);

        let _unconnected = if total_experiences > 0 {
            (total_experiences - connected_experiences.min(total_experiences)) as f64
                / total_experiences as f64
        } else {
            0.0
        };

        // Hours since last memory added
        let hours_since_memory = self.hours_since_event_type("memory-added", &agent_id_str);

        // Working-level memories
        let working_memories: u64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM memories WHERE agent_id = ?1 AND level = 'working'",
                params![agent_id_str],
                |row| row.get(0),
            )
            .unwrap_or(0);

        let inputs = RecollectInputs {
            session_memory_count: session_memories,
            total_experiences,
            unconnected_experiences: total_experiences
                - connected_experiences.min(total_experiences),
            hours_since_last_memory: hours_since_memory,
            working_memory_count: working_memories,
        };

        Ok(RecollectGauge::from_inputs(inputs))
    }

    fn compute_retrospect(
        &self,
        agent_id: &AgentId,
        _agent_name: &AgentName,
    ) -> Result<RetrospectGauge, EventError> {
        let agent_id_str = agent_id.to_string();

        // Hours since last archival memory (default to 168h = 1 week if never)
        let hours_since_archival = self
            .conn
            .query_row(
                "SELECT created_at FROM memories WHERE agent_id = ?1 AND level = 'archival' ORDER BY created_at DESC LIMIT 1",
                params![agent_id_str],
                |row| row.get::<_, String>(0),
            )
            .ok()
            .map(|t| self.hours_since_time(&t))
            .unwrap_or(168.0);

        // Hours since last project memory
        let hours_since_project = self
            .conn
            .query_row(
                "SELECT created_at FROM memories WHERE agent_id = ?1 AND level = 'project' ORDER BY created_at DESC LIMIT 1",
                params![agent_id_str],
                |row| row.get::<_, String>(0),
            )
            .ok()
            .map(|t| self.hours_since_time(&t))
            .unwrap_or(48.0);

        // Sessions (wake events) since last archival memory
        // Simplified: count all wake events as a proxy
        let total_experiences: u64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM experiences WHERE agent_id = ?1",
                params![agent_id_str],
                |row| row.get(0),
            )
            .unwrap_or(0);

        let inputs = RetrospectInputs {
            hours_since_last_archival: hours_since_archival,
            hours_since_last_project_memory: hours_since_project,
            sessions_since_retrospect: 0,
            total_experience_count: total_experiences,
        };

        Ok(RetrospectGauge::from_inputs(inputs))
    }

    fn upsert(
        &self,
        agent_id: &AgentId,
        urge: &UrgeName,
        gauge: &Gauge,
        updated_at: &Timestamp,
    ) -> Result<(), EventError> {
        let id = PressureId::new();
        let data_json = serde_json::to_string(gauge)?;

        self.conn.execute(
            "INSERT INTO pressures (id, agent_id, urge, data, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(agent_id, urge) DO UPDATE SET data = ?4, updated_at = ?5",
            params![
                id.to_string(),
                agent_id.to_string(),
                urge.to_string(),
                data_json,
                updated_at.as_string()
            ],
        )?;

        Ok(())
    }

    fn hours_since_lifecycle_event(&self, agent_name: &str, event_type: &str) -> f64 {
        self.conn
            .query_row(
                "SELECT created_at FROM events WHERE event_type = ?1 AND data LIKE ?2 ORDER BY rowid DESC LIMIT 1",
                params![event_type, format!("%{}%", agent_name)],
                |row| row.get::<_, String>(0),
            )
            .ok()
            .map(|t| self.hours_since_time(&t))
            .unwrap_or(24.0) // Default: 24h if never happened
    }

    fn last_lifecycle_time(&self, agent_name: &str, event_type: &str) -> Option<String> {
        self.conn
            .query_row(
                "SELECT created_at FROM events WHERE event_type = ?1 AND data LIKE ?2 ORDER BY rowid DESC LIMIT 1",
                params![event_type, format!("%{}%", agent_name)],
                |row| row.get::<_, String>(0),
            )
            .ok()
    }

    fn hours_since_event_type(&self, event_type: &str, agent_id: &str) -> f64 {
        self.conn
            .query_row(
                "SELECT created_at FROM events WHERE event_type = ?1 AND data LIKE ?2 ORDER BY rowid DESC LIMIT 1",
                params![event_type, format!("%{}%", agent_id)],
                |row| row.get::<_, String>(0),
            )
            .ok()
            .map(|t| self.hours_since_time(&t))
            .unwrap_or(24.0)
    }

    fn hours_since_time(&self, timestamp: &str) -> f64 {
        chrono::DateTime::parse_from_rfc3339(timestamp)
            .map(|t| {
                let elapsed = chrono::Utc::now().signed_duration_since(t);
                elapsed.num_seconds() as f64 / 3600.0
            })
            .unwrap_or(24.0)
    }
}
