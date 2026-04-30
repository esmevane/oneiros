use crate::*;

fn is_missing_table(e: &rusqlite::Error) -> bool {
    matches!(
        e,
        rusqlite::Error::SqliteFailure(_, Some(msg))
            if msg.starts_with("no such table")
    )
}

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
        if let Event::Known(Events::Bookmark(bookmark_event)) = &event.data
            && let Some(bookmark) = bookmark_event.maybe_bookmark()
        {
            self.write_bookmark(&bookmark)?;
        }
        Ok(())
    }

    fn write_bookmark(&self, bookmark: &Bookmark) -> Result<(), EventError> {
        self.db.execute(
            "INSERT OR REPLACE INTO bookmarks (id, brain, name, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                bookmark.id.to_string(),
                bookmark.brain.to_string(),
                bookmark.name.to_string(),
                bookmark.created_at.to_string(),
            ],
        )?;
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.db.execute_batch("DELETE FROM bookmarks")?;
        Ok(())
    }

    /// List bookmark names for a specific brain. Returns an empty
    /// list if the projection has not been migrated yet (cold start).
    pub fn list_for_brain(&self, brain: &BrainName) -> Result<Vec<BookmarkName>, rusqlite::Error> {
        let mut stmt = match self
            .db
            .prepare("SELECT name FROM bookmarks WHERE brain = ?1")
        {
            Ok(stmt) => stmt,
            Err(e) if is_missing_table(&e) => return Ok(Vec::new()),
            Err(e) => return Err(e),
        };
        let rows = stmt
            .query_map([brain.to_string()], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows.into_iter().map(BookmarkName::from).collect())
    }
}
