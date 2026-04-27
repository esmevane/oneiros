//! Pressure repo — async read queries over the pressure projection.

use rusqlite::params;

use crate::*;

pub struct PressureRepo<'a> {
    context: &'a ProjectContext,
}

impl<'a> PressureRepo<'a> {
    pub fn new(context: &'a ProjectContext) -> Self {
        Self { context }
    }

    pub async fn get(&self, agent_name: &AgentName) -> Result<Vec<Pressure>, EventError> {
        let db = self.context.db()?;

        // Look up agent by name to get the ID
        let agent = match AgentStore::new(&db).get(agent_name)? {
            Some(a) => a,
            None => return Ok(vec![]),
        };

        let mut stmt = db.prepare(
            "SELECT id, agent_id, urge, data, updated_at FROM pressures WHERE agent_id = ?1 ORDER BY urge",
        )?;

        let pressures = stmt
            .query_map(params![agent.id().to_string()], |row| {
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

    pub async fn list(&self) -> Result<Vec<Pressure>, EventError> {
        let db = self.context.db()?;

        let mut stmt = db.prepare(
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
}
