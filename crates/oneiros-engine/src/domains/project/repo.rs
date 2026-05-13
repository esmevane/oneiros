use rusqlite::params;

use crate::*;

/// Project read model — async queries against the host context.
pub(crate) struct ProjectRepo<'a> {
    scope: &'a Scope<AtHost>,
}

impl<'a> ProjectRepo<'a> {
    pub(crate) fn new(scope: &'a Scope<AtHost>) -> Self {
        Self { scope }
    }

    /// Eventually-consistent variant of [`get`]. Polls until the
    /// project appears or the configured patience window expires.
    ///
    /// [`get`]: ProjectRepo::get
    pub(crate) async fn fetch(&self, name: &ProjectName) -> Result<Option<Project>, EventError> {
        self.scope.config().fetch.eventual(|| self.get(name)).await
    }

    /// Eventually-consistent variant of [`get_by_id`]. Polls until the
    /// project appears or the configured patience window expires.
    ///
    /// [`get_by_id`]: ProjectRepo::get_by_id
    pub(crate) async fn fetch_by_id(&self, id: &ProjectId) -> Result<Option<Project>, EventError> {
        self.scope
            .config()
            .fetch
            .eventual(|| self.get_by_id(id))
            .await
    }

    pub(crate) async fn get(&self, name: &ProjectName) -> Result<Option<Project>, EventError> {
        let db = HostDb::open(self.scope).await?;
        let mut stmt = db.prepare("select id, name, created_at from projects where name = ?1")?;

        let raw = stmt.query_row(params![name.to_string()], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let created_at: String = row.get(2)?;
            Ok((id, name, created_at))
        });

        match raw {
            Ok((id, name, created_at)) => Ok(Some(
                Project::builder()
                    .id(id.parse()?)
                    .name(name)
                    .created_at(Timestamp::parse_str(created_at)?)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn get_by_id(&self, id: &ProjectId) -> Result<Option<Project>, EventError> {
        let db = HostDb::open(self.scope).await?;
        let mut stmt = db.prepare("select id, name, created_at from projects where id = ?1")?;

        let raw = stmt.query_row(params![id.to_string()], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let created_at: String = row.get(2)?;
            Ok((id, name, created_at))
        });

        match raw {
            Ok((id, name, created_at)) => Ok(Some(
                Project::builder()
                    .id(id.parse()?)
                    .name(name)
                    .created_at(Timestamp::parse_str(created_at)?)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn list(
        &self,
        filters: &SearchFilters,
    ) -> Result<Listed<Project>, EventError> {
        let db = HostDb::open(self.scope).await?;

        let total = {
            let mut stmt = db.prepare("SELECT COUNT(*) FROM projects")?;
            stmt.query_row([], |row| row.get::<_, usize>(0))?
        };

        let mut stmt = db.prepare(
            "SELECT id, name, created_at FROM projects ORDER BY name LIMIT ?1 OFFSET ?2",
        )?;

        let raw: Vec<(String, String, String)> = stmt
            .query_map(rusqlite::params![filters.limit, filters.offset], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut projects = vec![];

        for (id, name, created_at) in raw {
            projects.push(
                Project::builder()
                    .id(id.parse()?)
                    .name(name)
                    .created_at(Timestamp::parse_str(created_at)?)
                    .build(),
            );
        }

        Ok(Listed::new(projects, total))
    }
}
