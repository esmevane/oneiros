use rusqlite::params;

use crate::*;

/// Actor read model — queries, projection handling, and lifecycle.
pub struct ActorRepo<'a> {
    context: &'a SystemContext,
}

impl<'a> ActorRepo<'a> {
    pub fn new(context: &'a SystemContext) -> Self {
        Self { context }
    }

    pub async fn get(&self, id: ActorId) -> Result<Option<Actor>, EventError> {
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

    pub async fn list(&self) -> Result<Vec<Actor>, EventError> {
        let db = self.context.db()?;
        let mut statement =
            db.prepare("select id, tenant_id, name, created_at from actors order by name")?;

        let raw: Vec<(String, String, String, String)> = statement
            .query_map([], |row| {
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

        Ok(actors)
    }
}
