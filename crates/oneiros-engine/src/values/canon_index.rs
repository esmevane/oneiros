use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::*;

/// A bookmark entry — a canon paired with its reducer pipeline and chronicle.
#[derive(Clone)]
pub struct BookmarkEntry {
    pub canon: Canon,
    pub pipeline: ReducerPipeline<BrainCanon>,
    pub chronicle: Chronicle,
}

impl Default for BookmarkEntry {
    fn default() -> Self {
        Self {
            canon: Canon::new(),
            pipeline: ReducerPipeline::brain(),
            chronicle: Chronicle::new(),
        }
    }
}

/// The runtime branching state for a single brain.
///
/// Tracks which bookmarks exist (each a separate Canon/LoroDoc
/// with its own reducer pipeline) and which is currently active.
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

    /// The active canon for this brain.
    pub fn active_canon(&self) -> Canon {
        self.active_entry().canon
    }
}

/// A shared index of CRDT canons, keyed by brain name.
///
/// Hydrated at server startup from event logs. Shared across
/// all request handlers via `ServerState`. Each brain gets a
/// `Shelf` that tracks bookmarks and the active branch.
#[derive(Clone, Default)]
pub struct CanonIndex {
    brains: Arc<RwLock<HashMap<BrainName, Shelf>>>,
    system: Canon,
}

impl CanonIndex {
    pub fn new() -> Self {
        Self {
            brains: Arc::new(RwLock::new(HashMap::new())),
            system: Canon::new(),
        }
    }

    /// The system-level canon.
    pub fn system(&self) -> &Canon {
        &self.system
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

    /// Get or create the active canon for a brain.
    pub fn brain(&self, name: &BrainName) -> Result<Canon, EventError> {
        Ok(self.brain_entry(name)?.canon)
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

    /// Fork the active canon for a brain into a new bookmark.
    pub fn fork_brain(&self, brain: &BrainName, bookmark: &BookmarkName) -> Result<(), EventError> {
        let mut write = self
            .brains
            .write()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        let shelf = write.entry(brain.clone()).or_default();
        let active = shelf.active_entry();

        let forked = BookmarkEntry {
            canon: active.canon.fork(),
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

    /// Merge source bookmark into target bookmark for a brain.
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
            target_entry.canon.merge_from(&source_entry.canon)?;
        }

        Ok(())
    }

    /// Hydrate the system canon from its event log.
    ///
    /// Runs migrations first to ensure the schema is current — this
    /// handles existing installs that predate newer projections (e.g.
    /// the bookmarks table).
    pub fn hydrate_system(&self, config: &Config) -> Result<(), EventError> {
        let db = config.system_db()?;

        Projections::<SystemCanon>::system().migrate(&db)?;

        let events = EventLog::new(&db).load_all()?;
        let pipeline = ReducerPipeline::system();

        for event in &events {
            pipeline.apply(&event.data)?;
        }

        self.system.reconcile(&pipeline.state()?)?;

        Ok(())
    }

    /// Hydrate a brain's canon from its event log.
    ///
    /// Runs migrations first to ensure the schema is current.
    pub fn hydrate_brain(&self, config: &Config, name: &BrainName) -> Result<(), EventError> {
        let mut brain_config = config.clone();
        brain_config.brain = name.clone();

        let db = brain_config.brain_db()?;

        Projections::<BrainCanon>::project().migrate(&db)?;

        let events = EventLog::new(&db).load_all()?;

        let entry = self.brain_entry(name)?;

        for event in &events {
            entry.pipeline.apply(&event.data)?;
        }

        entry.canon.reconcile(&entry.pipeline.state()?)?;

        Ok(())
    }
}
