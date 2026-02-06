use rusqlite::functions::FunctionFlags;
use rusqlite::{Connection, params};
use serde_json::Value;
use std::path::Path;
use uuid::Uuid;

use crate::*;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(connection_string: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        let conn = Connection::open(connection_string.as_ref())?;

        Ok(Self { conn })
    }

    pub fn create(path: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        let conn = Connection::open(path)?;

        conn.create_scalar_function(
            "uuid",
            0,
            FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
            |_| Ok(Uuid::new_v4().to_string()),
        )?;

        conn.execute_batch(migrations::SYSTEM)?;

        Ok(Self { conn })
    }

    pub fn event_count(&self) -> Result<usize, DatabaseError> {
        let count: i64 = self
            .conn
            .query_row("select count(*) from events", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    pub fn tenant_exists(&self) -> Result<bool, DatabaseError> {
        let count: i64 = self
            .conn
            .query_row("select count(*) from tenant", [], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn reset_tenants(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from tenant")?;
        Ok(())
    }

    pub fn reset_actors(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from actor")?;
        Ok(())
    }

    pub fn create_tenant(&self, tenant_id: &str, name: &str) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into tenant (id, name) values (?1, ?2)",
            params![tenant_id, name],
        )?;
        Ok(())
    }

    pub fn create_actor(
        &self,
        actor_id: &str,
        tenant_id: &str,
        name: &str,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into actor (id, tenant_id, name) values (?1, ?2, ?3)",
            params![actor_id, tenant_id, name],
        )?;
        Ok(())
    }

    pub fn log_event(
        &self,
        data: impl serde::Serialize,
        projections: &[Projection],
    ) -> Result<(), DatabaseError> {
        let event = serde_json::to_value(data)?;

        self.create_event(&event)?;
        self.run_projections(projections, &event)
    }

    fn create_event(&self, data: &Value) -> Result<(), DatabaseError> {
        let event_type = data["type"].as_str().unwrap_or("__unmarked");
        let meta = serde_json::json!({ "type": event_type });

        self.conn.execute(
            "insert into events (data, meta) values (?1, ?2)",
            params![data.to_string(), meta.to_string()],
        )?;

        Ok(())
    }

    fn run_projections(
        &self,
        projections: &[Projection],
        event: &Value,
    ) -> Result<(), DatabaseError> {
        let Some(event_type) = event["type"].as_str() else {
            return Ok(());
        };

        let data = event["data"].clone();

        projections::project(self, projections, event_type, &data)
    }
}
