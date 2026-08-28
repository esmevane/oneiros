use rusqlite::params;

use crate::*;

/// Tenant read model — async queries against the host-tier scope.
pub(crate) struct TenantRepo<'a> {
    scope: &'a Scope<AtHost>,
}

impl<'a> TenantRepo<'a> {
    pub(crate) fn new(scope: &'a Scope<AtHost>) -> Self {
        Self { scope }
    }

    /// Eventually-consistent variant of [`get`]. Polls until the
    /// tenant appears or the configured patience window expires.
    ///
    /// [`get`]: TenantRepo::get
    pub(crate) async fn fetch(&self, id: &TenantId) -> Result<Option<Tenant>, TenantError> {
        self.scope.config().fetch.eventual(|| self.get(id)).await
    }

    pub(crate) async fn get(&self, id: &TenantId) -> Result<Option<Tenant>, TenantError> {
        let db = self.scope.host_db().await?;

        let raw = db.query_row(
            "select id, name, created_at from tenants where id = ?1",
            params![id.to_string()],
            |row| {
                let id: String = row.get(0)?;
                let name: String = row.get(1)?;
                let created_at: String = row.get(2)?;
                Ok((id, name, created_at))
            },
        );

        match raw {
            Ok((id, name, created_at)) => Ok(Some(Tenant {
                id: id.parse()?,
                name: TenantName::new(name),
                created_at: Timestamp::parse_str(created_at)?,
            })),
            Err(DbError::NotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn list(
        &self,
        filters: &SearchFilters,
    ) -> Result<Listed<Tenant>, TenantError> {
        let db = self.scope.host_db().await?;

        let count_sql = "SELECT COUNT(*) FROM tenants";
        let total = db.query_row(count_sql, [], |row| row.get::<_, usize>(0))?;

        let raw: Vec<(String, String, String)> = db.query_map(
            "SELECT id, name, created_at FROM tenants ORDER BY name LIMIT ?1 OFFSET ?2",
            rusqlite::params![filters.limit, filters.offset],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;

        let mut tenants = vec![];

        for (id, name, created_at) in raw {
            tenants.push(Tenant {
                id: id.parse()?,
                name: TenantName::new(name),
                created_at: Timestamp::parse_str(created_at)?,
            });
        }

        Ok(Listed::new(tenants, total))
    }
}
