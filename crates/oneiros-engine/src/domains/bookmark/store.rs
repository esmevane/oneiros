use crate::*;

fn is_missing_table(e: &rusqlite::Error) -> bool {
    matches!(
        e,
        rusqlite::Error::SqliteFailure(_, Some(msg))
            if msg.starts_with("no such table")
    )
}

pub(crate) struct BookmarkStore<'a> {
    db: &'a DbHandle<'a>,
}

impl<'a> BookmarkStore<'a> {
    pub(crate) fn new(db: &'a DbHandle<'a>) -> Self {
        Self { db }
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.db.execute_batch(
            "CREATE TABLE IF NOT EXISTS bookmarks (
                id TEXT PRIMARY KEY,
                project TEXT NOT NULL,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                UNIQUE(project, name)
            )",
        )?;
        Ok(())
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Bookmark(bookmark_event)) = &event.data
            && let Some(bookmark) = bookmark_event.maybe_bookmark()
        {
            self.write_bookmark(&bookmark)?;
        }
        Ok(())
    }

    /// Insert or update a bookmark row directly. Used by service-layer
    /// code that needs to write bookmark projections outside the event
    /// replay path (e.g. creating bookmarks during peer collect/submit).
    pub(crate) fn upsert(&self, bookmark: &Bookmark) -> Result<(), EventError> {
        self.write_bookmark(bookmark)
    }

    fn write_bookmark(&self, bookmark: &Bookmark) -> Result<(), EventError> {
        self.db.execute(
            "INSERT OR REPLACE INTO bookmarks (id, project, name, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                bookmark.id.to_string(),
                bookmark.project.to_string(),
                bookmark.name.to_string(),
                bookmark.created_at.to_string(),
            ],
        )?;
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.db.execute_batch("DELETE FROM bookmarks")?;
        Ok(())
    }

    /// List bookmark names for a specific project. Returns an empty
    /// list if the projection has not been migrated yet (cold start).
    pub(crate) fn list_for_project(
        &self,
        project: &ProjectName,
    ) -> Result<Vec<BookmarkName>, DbError> {
        let result = self.db.query_map(
            "SELECT name FROM bookmarks WHERE project = ?1",
            [project.to_string()],
            |row| row.get::<_, String>(0),
        );
        match result {
            Ok(names) => Ok(names.into_iter().map(BookmarkName::from).collect()),
            Err(DbError::Rusqlite(e)) if is_missing_table(&e) => Ok(Vec::new()),
            Err(e) => Err(e),
        }
    }
}
