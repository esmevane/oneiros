use crate::*;

pub struct BookmarkStore<'a> {
    db: &'a rusqlite::Connection,
}

impl<'a> BookmarkStore<'a> {
    pub fn new(db: &'a rusqlite::Connection) -> Self {
        Self { db }
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.db.execute_batch(
            "CREATE TABLE IF NOT EXISTS bookmarks (
                id TEXT PRIMARY KEY,
                brain TEXT NOT NULL,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                UNIQUE(brain, name)
            )",
        )?;
        Ok(())
    }

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Bookmark(bookmark_event) = &event.data {
            match bookmark_event {
                BookmarkEvents::BookmarkCreated(created) => {
                    self.insert(&created.brain, &created.name, &event.created_at)?;
                }
                BookmarkEvents::BookmarkForked(forked) => {
                    self.insert(&forked.brain, &forked.name, &event.created_at)?;
                }
                BookmarkEvents::BookmarkSwitched(_)
                | BookmarkEvents::BookmarkMerged(_)
                | BookmarkEvents::BookmarkShared(_)
                | BookmarkEvents::BookmarkFollowed(_)
                | BookmarkEvents::BookmarkCollected(_)
                | BookmarkEvents::BookmarkUnfollowed(_) => {}
            }
        }
        Ok(())
    }

    fn insert(
        &self,
        brain: &BrainName,
        name: &BookmarkName,
        created_at: &Timestamp,
    ) -> Result<(), EventError> {
        let id = BookmarkId::new();
        self.db.execute(
            "INSERT OR REPLACE INTO bookmarks (id, brain, name, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                id.to_string(),
                brain.to_string(),
                name.to_string(),
                created_at.to_string(),
            ],
        )?;
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.db.execute_batch("DELETE FROM bookmarks")?;
        Ok(())
    }
}
