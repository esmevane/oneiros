use rusqlite::params;

use crate::*;

/// Memory projection store — projection lifecycle, write operations, and sync read queries.
pub struct MemoryStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> MemoryStore<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Memory(memory_event)) = &event.data {
            match memory_event {
                MemoryEvents::MemoryAdded(memory) => self.insert(memory)?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM memories", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS memories (
                id TEXT PRIMARY KEY,
                agent_id TEXT NOT NULL,
                level TEXT NOT NULL DEFAULT '',
                content TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    pub fn get(&self, id: &MemoryId) -> Result<Option<Memory>, EventError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, level, content, created_at
             FROM memories WHERE id = ?1",
        )?;

        let result = stmt.query_row(params![id.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        });

        match result {
            Ok((id, agent_id, level, content, created_at)) => Ok(Some(
                Memory::builder()
                    .id(id.parse()?)
                    .agent_id(agent_id.parse()?)
                    .level(level)
                    .content(content)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self, agent: Option<&str>) -> Result<Vec<Memory>, EventError> {
        let mut stmt = match agent {
            Some(_) => self.conn.prepare(
                "SELECT id, agent_id, level, content, created_at
                 FROM memories WHERE agent_id = ?1 ORDER BY created_at",
            )?,
            None => self.conn.prepare(
                "SELECT id, agent_id, level, content, created_at
                 FROM memories ORDER BY created_at",
            )?,
        };

        let map_row = |row: &rusqlite::Row<'_>| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        };

        let raw = match agent {
            Some(a) => stmt.query_map(params![a], map_row),
            None => stmt.query_map([], map_row),
        }?
        .collect::<Result<Vec<_>, _>>()?;

        let mut memories = vec![];
        for (id, agent_id, level, content, created_at) in raw {
            memories.push(
                Memory::builder()
                    .id(id.parse()?)
                    .agent_id(agent_id.parse()?)
                    .level(level)
                    .content(content)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            );
        }

        Ok(memories)
    }

    fn insert(&self, memory: &Memory) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO memories (id, agent_id, level, content, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                memory.id.to_string(),
                memory.agent_id.to_string(),
                memory.level.to_string(),
                memory.content.to_string(),
                memory.created_at.as_string(),
            ],
        )?;
        Ok(())
    }
}
