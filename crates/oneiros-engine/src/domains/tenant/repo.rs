use rusqlite::{Connection, params};

use crate::store::{StoreError, StoredEvent};

use super::model::Tenant;

/// Tenant read model — queries, projection handling, and lifecycle.
pub struct TenantRepo<'a> {
    conn: &'a Connection,
}

impl<'a> TenantRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        if event.event_type == "tenant-created" {
            let tenant: Tenant = serde_json::from_value(event.data.clone())?;
            self.create_record(&tenant)?;
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM tenants", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS tenants (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, id: &str) -> Result<Option<Tenant>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, created_at FROM tenants WHERE id = ?1")?;

        let result = stmt.query_row(params![id], |row| {
            Ok(Tenant {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
            })
        });

        match result {
            Ok(tenant) => Ok(Some(tenant)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Tenant>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, created_at FROM tenants ORDER BY name")?;

        let tenants = stmt
            .query_map([], |row| {
                Ok(Tenant {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tenants)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn create_record(&self, tenant: &Tenant) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO tenants (id, name, created_at) VALUES (?1, ?2, ?3)",
            params![tenant.id, tenant.name, tenant.created_at],
        )?;
        Ok(())
    }
}
