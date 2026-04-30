use rusqlite::params;

use crate::*;

/// Tenant read model — async queries against the host-tier scope.
pub struct TenantRepo<'a> {
    scope: &'a Scope<AtHost>,
}

impl<'a> TenantRepo<'a> {
    pub fn new(scope: &'a Scope<AtHost>) -> Self {
        Self { scope }
    }

    pub async fn get(&self, id: &TenantId) -> Result<Option<Tenant>, TenantError> {
        let db = self.scope.host_db()?;
        let mut stmt = db.prepare("select id, name, created_at from tenants where id = ?1")?;

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

    pub async fn list(&self, filters: &SearchFilters) -> Result<Listed<Tenant>, TenantError> {
        let db = self.scope.host_db()?;

        let count_sql = "SELECT COUNT(*) FROM tenants";
        let total = {
            let mut stmt = db.prepare(count_sql)?;
            stmt.query_row([], |row| row.get::<_, usize>(0))?
        };

        let mut stmt = db
            .prepare("SELECT id, name, created_at FROM tenants ORDER BY name LIMIT ?1 OFFSET ?2")?;

        let raw: Vec<(String, String, String)> = stmt
            .query_map(rusqlite::params![filters.limit, filters.offset], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

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
