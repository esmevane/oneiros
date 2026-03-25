use rusqlite::params;

use crate::*;

/// Tenant read model — queries, projection handling, and lifecycle.
pub struct TenantRepo<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> TenantRepo<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Tenant(TenantEvents::TenantCreated(tenant)) = &event.data {
            self.conn.execute(
                "insert or replace into tenants (id, name, created_at) values (?1, ?2, ?3)",
                params![
                    tenant.id.to_string(),
                    tenant.name.to_string(),
                    tenant.created_at.as_string()
                ],
            )?;
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("delete from tenants", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "create table if not exists tenants (
                id text primary key,
                name text not null unique,
                created_at text not null default ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, id: &TenantId) -> Result<Option<Tenant>, TenantError> {
        let mut stmt = self
            .conn
            .prepare("select id, name, created_at from tenants where id = ?1")?;

        let raw = stmt.query_row(params![id.to_string()], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let created_at: String = row.get(2)?;
            Ok((id, name, created_at))
        });

        match raw {
            Ok((id, name, created_at)) => Ok(Some(Tenant {
                id: id.parse()?,
                name: TenantName::new(name),
                created_at: Timestamp::parse_str(created_at)?,
            })),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Tenant>, TenantError> {
        let mut stmt = self
            .conn
            .prepare("select id, name, created_at from tenants order by name")?;

        let raw: Vec<(String, String, String)> = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut tenants = vec![];

        for (id, name, created_at) in raw {
            tenants.push(Tenant {
                id: id.parse()?,
                name: TenantName::new(name),
                created_at: Timestamp::parse_str(created_at)?,
            });
        }

        Ok(tenants)
    }
}
