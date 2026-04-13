use rusqlite::params;

use crate::*;

/// Actor read model — queries, projection handling, and lifecycle.
pub(crate) struct ActorRepo<'a> {
    context: &'a SystemContext,
}

impl<'a> ActorRepo<'a> {
    pub(crate) fn new(context: &'a SystemContext) -> Self {
        Self { context }
    }

    pub(crate) async fn get(&self, id: ActorId) -> Result<Option<Actor>, EventError> {
        let db = self.context.db()?;
        let mut statement =
            db.prepare("select id, tenant_id, name, created_at from actors where id = ?1")?;

        let raw = statement.query_row(params![id.to_string()], |row| {
            let id: String = row.get(0)?;
            let tenant_id: String = row.get(1)?;
            let name: String = row.get(2)?;
            let created_at: String = row.get(3)?;

            Ok((id, tenant_id, name, created_at))
        });

        match raw {
            Ok((id, tenant_id, name, created_at)) => Ok(Some(
                Actor::builder()
                    .id(id.parse()?)
                    .tenant_id(tenant_id.parse()?)
                    .name(name)
                    .created_at(Timestamp::parse_str(created_at)?)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub(crate) async fn list(&self, filters: &SearchFilters) -> Result<Listed<Actor>, EventError> {
        let db = self.context.db()?;

        let total = {
            let mut stmt = db.prepare("SELECT COUNT(*) FROM actors")?;
            stmt.query_row([], |row| row.get::<_, usize>(0))?
        };

        let select_sql =
            "SELECT id, tenant_id, name, created_at FROM actors ORDER BY name LIMIT ?1 OFFSET ?2";
        let mut statement = db.prepare(select_sql)?;

        let raw: Vec<(String, String, String, String)> = statement
            .query_map(rusqlite::params![filters.limit, filters.offset], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut actors = vec![];

        for (id, tenant_id, name, created_at) in raw {
            actors.push(
                Actor::builder()
                    .id(id.parse()?)
                    .tenant_id(tenant_id.parse()?)
                    .name(name)
                    .created_at(Timestamp::parse_str(created_at)?)
                    .build(),
            );
        }

        Ok(Listed::new(actors, total))
    }
}
