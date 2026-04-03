use rusqlite::params;

use crate::*;

/// Brain read model — async queries against the system context.
pub struct BrainRepo<'a> {
    context: &'a SystemContext,
}

impl<'a> BrainRepo<'a> {
    pub fn new(context: &'a SystemContext) -> Self {
        Self { context }
    }

    pub async fn get(&self, name: &BrainName) -> Result<Option<Brain>, EventError> {
        let db = self.context.db()?;
        let mut stmt = db.prepare("select id, name, created_at from brains where name = ?1")?;

        let raw = stmt.query_row(params![name.to_string()], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let created_at: String = row.get(2)?;
            Ok((id, name, created_at))
        });

        match raw {
            Ok((id, name, created_at)) => Ok(Some(
                Brain::builder()
                    .id(id.parse()?)
                    .name(name)
                    .created_at(Timestamp::parse_str(created_at)?)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn get_by_id(&self, id: &BrainId) -> Result<Option<Brain>, EventError> {
        let db = self.context.db()?;
        let mut stmt = db.prepare("select id, name, created_at from brains where id = ?1")?;

        let raw = stmt.query_row(params![id.to_string()], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let created_at: String = row.get(2)?;
            Ok((id, name, created_at))
        });

        match raw {
            Ok((id, name, created_at)) => Ok(Some(
                Brain::builder()
                    .id(id.parse()?)
                    .name(name)
                    .created_at(Timestamp::parse_str(created_at)?)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn list(&self, filters: &SearchFilters) -> Result<Listed<Brain>, EventError> {
        let db = self.context.db()?;

        let total = {
            let mut stmt = db.prepare("SELECT COUNT(*) FROM brains")?;
            stmt.query_row([], |row| row.get::<_, usize>(0))?
        };

        let mut stmt =
            db.prepare("SELECT id, name, created_at FROM brains ORDER BY name LIMIT ?1 OFFSET ?2")?;

        let raw: Vec<(String, String, String)> = stmt
            .query_map(rusqlite::params![filters.limit, filters.offset], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut brains = vec![];

        for (id, name, created_at) in raw {
            brains.push(
                Brain::builder()
                    .id(id.parse()?)
                    .name(name)
                    .created_at(Timestamp::parse_str(created_at)?)
                    .build(),
            );
        }

        Ok(Listed::new(brains, total))
    }

    pub async fn name_exists(&self, name: &BrainName) -> Result<bool, EventError> {
        let db = self.context.db()?;
        let count: i64 = db.query_row(
            "select count(*) from brains where name = ?1",
            params![name.to_string()],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }
}
