use rusqlite::params;

use crate::*;

/// Projection store for the `follows` table. Listens to the follow
/// lifecycle events that live on `BookmarkEvents`: `BookmarkFollowed`,
/// `BookmarkCollected`, `BookmarkUnfollowed`. `BookmarkShared` doesn't
/// touch the follows table — it's only recorded for audit.
pub(crate) struct FollowStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> FollowStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        match &event.data {
            Events::Bookmark(BookmarkEvents::BookmarkFollowed(follow)) => {
                self.create_record(follow)?;
            }
            Events::Bookmark(BookmarkEvents::BookmarkCollected(collected)) => {
                self.advance_checkpoint(collected)?;
            }
            Events::Bookmark(BookmarkEvents::BookmarkUnfollowed(unfollowed)) => {
                self.delete_record(unfollowed.follow_id)?;
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("delete from follows", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "create table if not exists follows (
                id text primary key,
                brain text not null,
                bookmark text not null,
                source text not null,
                checkpoint text not null,
                created_at text not null default '',
                unique(brain, bookmark)
            )",
        )?;
        Ok(())
    }

    fn create_record(&self, follow: &Follow) -> Result<(), EventError> {
        let source_json = serde_json::to_string(&follow.source)?;
        let checkpoint_json = serde_json::to_string(&follow.checkpoint)?;

        self.conn.execute(
            "insert or replace into follows (
                id, brain, bookmark, source, checkpoint, created_at
             )
             values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                follow.id.to_string(),
                follow.brain.to_string(),
                follow.bookmark.to_string(),
                source_json,
                checkpoint_json,
                follow.created_at.as_string(),
            ],
        )?;
        Ok(())
    }

    fn advance_checkpoint(&self, collected: &BookmarkCollected) -> Result<(), EventError> {
        let checkpoint_json = serde_json::to_string(&collected.checkpoint)?;
        self.conn.execute(
            "update follows set checkpoint = ?1 where id = ?2",
            params![checkpoint_json, collected.follow_id.to_string()],
        )?;
        Ok(())
    }

    fn delete_record(&self, id: FollowId) -> Result<(), EventError> {
        self.conn
            .execute("delete from follows where id = ?1", params![id.to_string()])?;
        Ok(())
    }
}
