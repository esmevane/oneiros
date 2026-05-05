use rusqlite::params;

use crate::*;

/// Follow read model — async queries against the system context.
pub struct FollowRepo<'a> {
    scope: &'a Scope<AtHost>,
}

impl<'a> FollowRepo<'a> {
    pub fn new(scope: &'a Scope<AtHost>) -> Self {
        Self { scope }
    }

    /// Eventually-consistent variant of [`get`]. Polls until the
    /// follow appears or the configured patience window expires.
    ///
    /// [`get`]: FollowRepo::get
    pub async fn fetch(&self, id: FollowId) -> Result<Option<Follow>, EventError> {
        self.scope.config().fetch.eventual(|| self.get(id)).await
    }

    pub async fn get(&self, id: FollowId) -> Result<Option<Follow>, EventError> {
        let db = HostDb::open(self.scope).await?;
        let mut stmt = db.prepare(
            "select id, brain, bookmark, source, checkpoint, created_at \
             from follows where id = ?1",
        )?;

        let raw = stmt.query_row(params![id.to_string()], read_row);

        match raw {
            Ok(row) => Ok(Some(follow_from_row(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub async fn list(&self, filters: &SearchFilters) -> Result<Listed<Follow>, EventError> {
        let db = HostDb::open(self.scope).await?;

        let total = {
            let mut stmt = db.prepare("SELECT COUNT(*) FROM follows")?;
            stmt.query_row([], |row| row.get::<_, usize>(0))?
        };

        let mut stmt = db.prepare(
            "SELECT id, brain, bookmark, source, checkpoint, created_at \
             FROM follows ORDER BY created_at DESC LIMIT ?1 OFFSET ?2",
        )?;

        let raw: Vec<FollowRow> = stmt
            .query_map(rusqlite::params![filters.limit, filters.offset], read_row)?
            .collect::<Result<Vec<_>, _>>()?;

        let mut follows = Vec::with_capacity(raw.len());
        for row in raw {
            follows.push(follow_from_row(row)?);
        }

        Ok(Listed::new(follows, total))
    }

    pub async fn for_bookmark(
        &self,
        brain: &BrainName,
        bookmark: &BookmarkName,
    ) -> Result<Option<Follow>, EventError> {
        let db = HostDb::open(self.scope).await?;
        let mut stmt = db.prepare(
            "select id, brain, bookmark, source, checkpoint, created_at \
             from follows where brain = ?1 and bookmark = ?2",
        )?;

        let raw = stmt.query_row(params![brain.to_string(), bookmark.to_string()], read_row);

        match raw {
            Ok(row) => Ok(Some(follow_from_row(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }
}

type FollowRow = (String, String, String, String, String, String);

fn read_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<FollowRow> {
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
    (id, brain, bookmark, source, checkpoint, created_at): FollowRow,
) -> Result<Follow, EventError> {
    let source: FollowSource = serde_json::from_str(&source)?;
    let checkpoint: Checkpoint = serde_json::from_str(&checkpoint)?;

    Ok(Follow::builder()
        .id(id.parse::<FollowId>()?)
        .brain(BrainName::new(brain))
        .bookmark(BookmarkName::new(bookmark))
        .source(source)
        .checkpoint(checkpoint)
        .created_at(Timestamp::parse_str(created_at)?)
        .build())
}
