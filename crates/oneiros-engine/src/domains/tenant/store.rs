use rusqlite::params;

use crate::*;

pub(crate) struct TenantStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> TenantStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Tenant(TenantEvents::TenantCreated(creation))) = &event.data {
            let tenant = creation.current()?.tenant;
            self.write_tenant(&tenant)?;
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("delete from tenants", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "create table if not exists tenants (
                id text primary key,
                name text not null unique,
                created_at text not null default ''
            )",
        )?;
        Ok(())
    }

    fn write_tenant(&self, tenant: &Tenant) -> Result<(), EventError> {
        self.conn.execute(
            "insert or replace into tenants (id, name, created_at) values (?1, ?2, ?3)",
            params![
                tenant.id.to_string(),
                tenant.name.to_string(),
                tenant.created_at.as_string()
            ],
        )?;
        Ok(())
    }
}
