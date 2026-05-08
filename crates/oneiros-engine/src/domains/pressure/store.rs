//! Pressure store — projection lifecycle, compute logic, and write operations.
//!
//! Pressure is computed from queries across cognition, memory, and events.
//! The projection gathers inputs and computes urgency via the gauge system.

use rusqlite::params;

use crate::*;

pub(crate) struct PressureStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> PressureStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("delete from pressures", [])?;

        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "create table if not exists pressures (
                id         text primary key,
                agent_id   text not null,
                urge       text not null,
                data       text not null,
                updated_at text not null,
                unique(agent_id, urge)
            )",
        )?;
        Ok(())
    }

    pub(crate) fn get(&self, agent_name: &AgentName) -> Result<Vec<Pressure>, EventError> {
        // Look up agent by name to get the ID
        let agent = match AgentStore::new(self.conn).get(agent_name)? {
            Some(a) => a,
            None => return Ok(vec![]),
        };

        let mut stmt = self.conn.prepare(
            "select id, agent_id, urge, data, updated_at from pressures where agent_id = ?1 order by urge",
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

    pub(crate) fn upsert(
        &self,
        agent_id: &AgentId,
        urge: &UrgeName,
        gauge: &Gauge,
        updated_at: &Timestamp,
    ) -> Result<(), EventError> {
        let id = PressureId::new();
        let data_json = serde_json::to_string(gauge)?;

        self.conn.execute(
            "insert into pressures (id, agent_id, urge, data, updated_at) values (?1, ?2, ?3, ?4, ?5)
             on conflict(agent_id, urge) do update set data = ?4, updated_at = ?5",
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
}
