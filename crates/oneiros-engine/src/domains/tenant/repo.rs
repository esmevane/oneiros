use rusqlite::{Connection, params};

use crate::*;

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
        if let Events::Tenant(TenantEvents::TenantCreated(tenant)) = &event.data {
            self.create_record(tenant)?;
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

        let raw = stmt.query_row(params![id], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let created_at: String = row.get(2)?;
            Ok((id, name, created_at))
        });

        match raw {
            Ok((id, name, created_at)) => Ok(Some(Tenant {
                id: id.parse()?,
                name: TenantName::new(name),
                created_at,
            })),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Tenant>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, created_at FROM tenants ORDER BY name")?;

        let raw: Vec<(String, String, String)> = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut tenants = vec![];

        for (id, name, created_at) in raw {
            tenants.push(Tenant {
                id: id.parse()?,
                name: TenantName::new(name),
                created_at,
            });
        }

        Ok(tenants)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn create_record(&self, tenant: &Tenant) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO tenants (id, name, created_at) VALUES (?1, ?2, ?3)",
            params![
                tenant.id.to_string(),
                tenant.name.to_string(),
                tenant.created_at
            ],
        )?;
        Ok(())
    }
}
