use rusqlite::params;

use crate::*;

/// Follow read model — async queries against the host context.
pub(crate) struct FollowRepo<'a> {
    scope: &'a Scope<AtHost>,
}

impl<'a> FollowRepo<'a> {
    pub(crate) fn new(scope: &'a Scope<AtHost>) -> Self {
        Self { scope }
    }

    /// Eventually-consistent variant of [`get`]. Polls until the
    /// follow appears or the configured patience window expires.
    ///
    /// [`get`]: FollowRepo::get
    pub(crate) async fn fetch(&self, id: FollowId) -> Result<Option<Follow>, EventError> {
        self.scope.config().fetch.eventual(|| self.get(id)).await
    }

    pub(crate) async fn get(&self, id: FollowId) -> Result<Option<Follow>, EventError> {
        let db = self.scope.host_db().await?;

        let raw = db.query_row(
            "select id, project, bookmark, source, checkpoint, created_at \
             from follows where id = ?1",
            params![id.to_string()],
            read_row,
        );

        match raw {
            Ok(row) => Ok(Some(follow_from_row(row)?)),
            Err(DbError::Rusqlite(rusqlite::Error::QueryReturnedNoRows)) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub(crate) async fn list(&self, filters: &SearchFilters) -> Result<Listed<Follow>, EventError> {
        let db = self.scope.host_db().await?;

        let total = db.query_row("SELECT COUNT(*) FROM follows", [], |row| {
            row.get::<_, usize>(0)
        })?;

        let raw: Vec<FollowRow> = db.query_map(
            "SELECT id, project, bookmark, source, checkpoint, created_at \
             FROM follows ORDER BY created_at DESC LIMIT ?1 OFFSET ?2",
            rusqlite::params![filters.limit, filters.offset],
            read_row,
        )?;

        let mut follows = Vec::with_capacity(raw.len());
        for row in raw {
            follows.push(follow_from_row(row)?);
        }

        Ok(Listed::new(follows, total))
    }

    pub(crate) async fn for_bookmark(
        &self,
        project: &ProjectName,
        bookmark: &BookmarkName,
    ) -> Result<Option<Follow>, EventError> {
        let db = self.scope.host_db().await?;

        let raw = db.query_row(
            "select id, project, bookmark, source, checkpoint, created_at \
             from follows where project = ?1 and bookmark = ?2",
            params![project.to_string(), bookmark.to_string()],
            read_row,
        );

        match raw {
            Ok(row) => Ok(Some(follow_from_row(row)?)),
            Err(DbError::Rusqlite(rusqlite::Error::QueryReturnedNoRows)) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }
}

type FollowRow = (String, String, String, String, String, String);

fn read_row(row: &DbRow<'_>) -> Result<FollowRow, DbError> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
    ))
}

fn follow_from_row(
    (id, project, bookmark, source, checkpoint, created_at): FollowRow,
) -> Result<Follow, EventError> {
    let source: FollowSource = serde_json::from_str(&source)?;
    let checkpoint: Checkpoint = serde_json::from_str(&checkpoint)?;

    Ok(Follow::builder()
        .id(id.parse::<FollowId>()?)
        .project(ProjectName::new(project))
        .bookmark(BookmarkName::new(bookmark))
        .source(source)
        .checkpoint(checkpoint)
        .created_at(Timestamp::parse_str(created_at)?)
        .build())
}
