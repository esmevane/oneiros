use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::*;

/// A bookmark entry — a reducer pipeline paired with its chronicle.
#[derive(Clone)]
pub struct BookmarkEntry {
    pub pipeline: ReducerPipeline<BrainCanon>,
    pub chronicle: Chronicle,
}

impl Default for BookmarkEntry {
    fn default() -> Self {
        Self {
            pipeline: ReducerPipeline::brain(),
            chronicle: Chronicle::new(),
        }
    }
}

/// The runtime branching state for a single brain.
///
/// Tracks which bookmarks exist (each with its own reducer pipeline
/// and chronicle) and which is currently active.
#[derive(Clone)]
pub struct Shelf {
    pub active: BookmarkName,
    pub branches: HashMap<BookmarkName, BookmarkEntry>,
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
    /// The active bookmark entry for this brain.
    pub fn active_entry(&self) -> BookmarkEntry {
        self.branches.get(&self.active).cloned().unwrap_or_default()
    }
}

/// A shared index of bookmark state, keyed by brain name.
///
/// Shared across all request handlers via `ServerState`. Each brain
/// gets a `Shelf` that tracks bookmarks and the active branch.
#[derive(Clone, Default)]
pub struct CanonIndex {
    brains: Arc<RwLock<HashMap<BrainName, Shelf>>>,
}

impl CanonIndex {
    pub fn new() -> Self {
        Self {
            brains: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get or create the active bookmark entry for a brain.
    pub fn brain_entry(&self, name: &BrainName) -> Result<BookmarkEntry, EventError> {
        {
            let read = self
                .brains
                .read()
                .map_err(|e| EventError::Lock(e.to_string()))?;

            if let Some(shelf) = read.get(name) {
                return Ok(shelf.active_entry());
            }
        }

        let mut write = self
            .brains
            .write()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        Ok(write.entry(name.clone()).or_default().active_entry())
    }

    /// Get the active bookmark's chronicle for a brain.
    pub fn chronicle(&self, brain: &BrainName) -> Result<Chronicle, EventError> {
        Ok(self.brain_entry(brain)?.chronicle)
    }

    /// Get a specific bookmark's chronicle for a brain.
    pub fn bookmark_chronicle(
        &self,
        brain: &BrainName,
        bookmark: &BookmarkName,
    ) -> Result<Chronicle, EventError> {
        let read = self
            .brains
            .read()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        Ok(read
            .get(brain)
            .and_then(|shelf| shelf.branches.get(bookmark))
            .map(|entry| entry.chronicle.clone())
            .unwrap_or_default())
    }

    /// Get the active bookmark name for a brain.
    pub fn active_bookmark(&self, brain: &BrainName) -> Result<BookmarkName, EventError> {
        let read = self
            .brains
            .read()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        Ok(read
            .get(brain)
            .map(|bc| bc.active.clone())
            .unwrap_or_else(BookmarkName::main))
    }

    /// Fork the active bookmark into a new bookmark.
    pub fn fork_brain(&self, brain: &BrainName, bookmark: &BookmarkName) -> Result<(), EventError> {
        let mut write = self
            .brains
            .write()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        let shelf = write.entry(brain.clone()).or_default();
        let active = shelf.active_entry();

        let forked = BookmarkEntry {
            pipeline: ReducerPipeline::brain_with_state(active.pipeline.state()?)?,
            chronicle: active.chronicle.fork()?,
        };
        shelf.branches.insert(bookmark.clone(), forked);
        shelf.active = bookmark.clone();

        Ok(())
    }

    /// Switch the active bookmark for a brain.
    pub fn switch_brain(
        &self,
        brain: &BrainName,
        bookmark: &BookmarkName,
    ) -> Result<(), EventError> {
        let mut write = self
            .brains
            .write()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        if let Some(shelf) = write.get_mut(brain) {
            shelf.active = bookmark.clone();
        }

        Ok(())
    }

    /// Merge source bookmark's chronicle into target bookmark.
    pub fn merge_brain(
        &self,
        brain: &BrainName,
        source: &BookmarkName,
        target: &BookmarkName,
    ) -> Result<(), EventError> {
        let read = self
            .brains
            .read()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        if let Some(shelf) = read.get(brain)
            && let (Some(source_entry), Some(target_entry)) =
                (shelf.branches.get(source), shelf.branches.get(target))
        {
            // Chronicle objects live in the system DB.
            let config = Config {
                brain: brain.clone(),
                ..Default::default()
            };

            if let Ok(db) = config.system_db() {
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

    /// Hydrate a brain's reducer pipeline and chronicle from its event log.
    ///
    /// Opens events.db standalone for the event log, then the bookmark
    /// connection for projection migrations.
    pub fn hydrate_brain(&self, config: &Config, name: &BrainName) -> Result<(), EventError> {
        let mut brain_config = config.clone();
        brain_config.brain = name.clone();

        // Events DB — standalone (no ATTACH).
        let events_path = brain_config.events_db_path();
        if !events_path.exists() {
            return Ok(());
        }
        let events_db = rusqlite::Connection::open(&events_path)?;
        events_db.pragma_update(None, "journal_mode", "wal")?;
        let log = EventLog::new(&events_db);

        // Ensure projection schema exists in the bookmark DB.
        let bookmark_db = brain_config.bookmark_conn()?;
        Projections::<BrainCanon>::project().migrate(&bookmark_db)?;

        let events = log.load_all()?;

        let entry = self.brain_entry(name)?;

        for event in &events {
            if let Event::Known(data) = &event.data {
                entry.pipeline.apply(data)?;
            }
        }

        // Rebuild the chronicle in the system DB.
        let system_db = config.system_db()?;
        let store = ChronicleStore::new(&system_db);
        store.migrate()?;
        for event in &events {
            entry
                .chronicle
                .record(event, &store.resolver(), &store.writer())?;
        }

        Ok(())
    }
}
