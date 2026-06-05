use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::*;

/// A bookmark entry — a reducer pipeline paired with its chronicle.
#[derive(Clone)]
pub(crate) struct BookmarkEntry {
    pub(crate) pipeline: ReducerPipeline<ProjectCanon>,
    pub(crate) chronicle: Chronicle,
}

impl Default for BookmarkEntry {
    fn default() -> Self {
        Self {
            pipeline: ReducerPipeline::project(),
            chronicle: Chronicle::new(),
        }
    }
}

/// The runtime branching state for a single project.
///
/// Tracks which bookmarks exist (each with its own reducer pipeline
/// and chronicle) and which is currently active.
#[derive(Clone)]
pub(crate) struct Shelf {
    pub(crate) active: BookmarkName,
    pub(crate) branches: HashMap<BookmarkName, BookmarkEntry>,
}

impl Default for Shelf {
    fn default() -> Self {
        let mut branches = HashMap::new();
        branches.insert(BookmarkName::main(), BookmarkEntry::default());

        Self {
            active: BookmarkName::main(),
            branches,
        }
    }
}

impl Shelf {
    /// The active bookmark entry for this project.
    pub(crate) fn active_entry(&self) -> BookmarkEntry {
        self.branches.get(&self.active).cloned().unwrap_or_default()
    }
}

/// A shared index of bookmark state, keyed by project name.
///
/// Shared across all request handlers via `ServerState`. Each project
/// gets a `Shelf` that tracks bookmarks and the active branch.
#[derive(Clone, Default)]
pub(crate) struct CanonIndex {
    projects: Arc<RwLock<HashMap<ProjectName, Shelf>>>,
}

impl CanonIndex {
    pub(crate) fn new() -> Self {
        Self {
            projects: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get or create the active bookmark entry for a project.
    pub(crate) fn project_entry(&self, name: &ProjectName) -> Result<BookmarkEntry, EventError> {
        {
            let read = self
                .projects
                .read()
                .map_err(|e| EventError::Lock(e.to_string()))?;

            if let Some(shelf) = read.get(name) {
                return Ok(shelf.active_entry());
            }
        }

        let mut write = self
            .projects
            .write()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        Ok(write.entry(name.clone()).or_default().active_entry())
    }

    /// Get the active bookmark's chronicle for a project.
    pub(crate) fn chronicle(&self, project: &ProjectName) -> Result<Chronicle, EventError> {
        Ok(self.project_entry(project)?.chronicle)
    }

    /// Get a specific bookmark's chronicle for a project.
    pub(crate) fn bookmark_chronicle(
        &self,
        project: &ProjectName,
        bookmark: &BookmarkName,
    ) -> Result<Chronicle, EventError> {
        let read = self
            .projects
            .read()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        Ok(read
            .get(project)
            .and_then(|shelf| shelf.branches.get(bookmark))
            .map(|entry| entry.chronicle.clone())
            .unwrap_or_default())
    }

    /// Get the active bookmark name for a project.
    pub(crate) fn active_bookmark(
        &self,
        project: &ProjectName,
    ) -> Result<BookmarkName, EventError> {
        let read = self
            .projects
            .read()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        Ok(read
            .get(project)
            .map(|bc| bc.active.clone())
            .unwrap_or_else(BookmarkName::main))
    }

    /// Check whether a bookmark exists for a project.
    pub(crate) fn has_bookmark(
        &self,
        project: &ProjectName,
        bookmark: &BookmarkName,
    ) -> Result<bool, EventError> {
        let read = self
            .projects
            .read()
            .map_err(|e| EventError::Lock(e.to_string()))?;
        Ok(read
            .get(project)
            .map(|shelf| shelf.branches.contains_key(bookmark))
            .unwrap_or(false))
    }

    /// Get all bookmark names for a project.
    pub(crate) fn bookmark_names(
        &self,
        project: &ProjectName,
    ) -> Result<Vec<BookmarkName>, EventError> {
        let read = self
            .projects
            .read()
            .map_err(|e| EventError::Lock(e.to_string()))?;
        Ok(read
            .get(project)
            .map(|shelf| shelf.branches.keys().cloned().collect())
            .unwrap_or_default())
    }

    /// Fork the active bookmark into a new bookmark.
    pub(crate) fn fork_project(
        &self,
        project: &ProjectName,
        bookmark: &BookmarkName,
    ) -> Result<(), EventError> {
        let mut write = self
            .projects
            .write()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        let shelf = write.entry(project.clone()).or_default();
        let active = shelf.active_entry();

        let forked = BookmarkEntry {
            pipeline: ReducerPipeline::project_with_state(active.pipeline.state()?)?,
            chronicle: active.chronicle.fork()?,
        };
        shelf.branches.insert(bookmark.clone(), forked);
        shelf.active = bookmark.clone();

        Ok(())
    }

    /// Switch the active bookmark for a project.
    pub(crate) fn switch_project(
        &self,
        project: &ProjectName,
        bookmark: &BookmarkName,
    ) -> Result<(), EventError> {
        let mut write = self
            .projects
            .write()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        if let Some(shelf) = write.get_mut(project) {
            shelf.active = bookmark.clone();
        }

        Ok(())
    }

    /// Merge source bookmark's chronicle into target bookmark.
    pub(crate) fn merge_project(
        &self,
        project: &ProjectName,
        source: &BookmarkName,
        target: &BookmarkName,
    ) -> Result<(), EventError> {
        let read = self
            .projects
            .read()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        if let Some(shelf) = read.get(project)
            && let (Some(source_entry), Some(target_entry)) =
                (shelf.branches.get(source), shelf.branches.get(target))
        {
            // Chronicle objects live in the host DB.
            let config = Config {
                project: project.clone(),
                ..Default::default()
            };

            if let Ok(db) = config.host_db() {
                let store = ChronicleStore::new(&db);
                let _ = store.migrate();
                let _ = target_entry.chronicle.merge(
                    &source_entry.chronicle,
                    &store.resolver(),
                    &store.writer(),
                );
            }
        }

        Ok(())
    }

    /// Hydrate a project's reducer pipeline and chronicle from its event log.
    ///
    /// Opens events.db standalone for the event log, then the bookmark
    /// connection for projection migrations.
    pub(crate) fn hydrate_project(
        &self,
        config: &Config,
        name: &ProjectName,
    ) -> Result<(), EventError> {
        let mut project_config = config.clone();
        project_config.project = name.clone();

        // Events DB — standalone (no ATTACH).
        let events_path = project_config.events_db_path();
        if !events_path.exists() {
            return Ok(());
        }
        let events_db = rusqlite::Connection::open(&events_path)?;
        events_db.pragma_update(None, "journal_mode", "wal")?;
        let log = EventLog::new(&events_db);

        // Ensure projection schema exists in the bookmark DB.
        let bookmark_db = project_config.bookmark_conn()?;
        Projections::<ProjectCanon>::project().migrate(&bookmark_db)?;

        let events = log.load_all()?;

        let entry = self.project_entry(name)?;

        for event in &events {
            if let Event::Known(data) = &event.data {
                entry.pipeline.apply(data)?;
            }
        }

        // Rebuild the chronicle in the host DB.
        let host_db = config.host_db()?;
        let store = ChronicleStore::new(&host_db);
        store.migrate()?;
        for event in &events {
            entry
                .chronicle
                .record(event, &store.resolver(), &store.writer())?;
        }

        Ok(())
    }
}
