//! Scope — typestate ladder for capability via graduation.
//!
//! Scope is pure shape. Reaching a tier IS the capability —
//! `Scope<AtBookmark>` exists because the bookmark tier was actually
//! reached, not because a constructor declared it.
//!
//! Scope's transitions take pre-built data and verify trivial
//! invariants (e.g. that a Pick name lives in the registry it was
//! handed). Scope does NOT fetch from the filesystem or query the
//! system DB. That work belongs to [`ComposeScope`], the factory
//! paired with scope at every callsite that wields it.
//!
//! Scope and ComposeScope are wielded together — same module, same
//! callsite. Scope is the typed shape; ComposeScope is the plumbing
//! that fills it.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::*;

// ─────────────────────────────────────────────────────────────────────
// Wrapper
// ─────────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct Scope<T> {
    inner: T,
}

impl<T> Scope<T> {
    fn wrap(inner: T) -> Self {
        Self { inner }
    }
}

// ─────────────────────────────────────────────────────────────────────
// Tiers — five structs, one per stage. No phantom markers.
// ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Default)]
pub struct Empty;

#[derive(Clone)]
pub struct Uninitialized {
    config: Config,
}

#[derive(Clone)]
pub struct AtHost {
    config: Config,
    host: Arc<HostInfra>,
}

#[derive(Clone)]
pub struct AtProject {
    config: Config,
    host: Arc<HostInfra>,
    project: Arc<ProjectInfra>,
}

#[derive(Clone)]
pub struct AtBookmark {
    config: Config,
    host: Arc<HostInfra>,
    project: Arc<ProjectInfra>,
    bookmark: Arc<BookmarkInfra>,
}

// ─────────────────────────────────────────────────────────────────────
// Resource bundles
//
// Hold paths and registry data. Never connections — those are per-call
// work at the operation layer.
// ─────────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct HostInfra {
    pub data_dir: PathBuf,
    pub system_db_path: PathBuf,
    pub host_key_path: PathBuf,
    pub projects: HashMap<BrainName, Arc<ProjectInfra>>,
}

#[derive(Clone)]
pub struct ProjectInfra {
    pub name: BrainName,
    pub brain_dir: PathBuf,
    pub events_db_path: PathBuf,
    pub bookmarks_dir: PathBuf,
    pub bookmarks: HashMap<BookmarkName, Arc<BookmarkInfra>>,
}

#[derive(Clone)]
pub struct BookmarkInfra {
    pub name: BookmarkName,
    pub bookmark_db_path: PathBuf,
}

// ─────────────────────────────────────────────────────────────────────
// Errors
// ─────────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum ScopeError {
    #[error("project not found in registry: {0}")]
    ProjectNotFound(BrainName),

    #[error("bookmark not found in registry: {0}")]
    BookmarkNotFound(BookmarkName),
}

#[derive(Debug, thiserror::Error)]
pub enum ComposeError {
    #[error("host hydration failed: {0}")]
    HostHydrationFailed(String),

    #[error("could not enumerate projects: {0}")]
    ProjectEnumerationFailed(String),

    #[error("could not enumerate bookmarks: {0}")]
    BookmarkEnumerationFailed(String),

    #[error("no projects in registry")]
    NoProjects,

    #[error("no bookmarks in registry for brain: {0}")]
    NoBookmarks(BrainName),

    #[error(transparent)]
    Scope(#[from] ScopeError),

    #[error(transparent)]
    Database(#[from] rusqlite::Error),
}

// ─────────────────────────────────────────────────────────────────────
// Scope transitions — pure structural advances. No fetching.
// ─────────────────────────────────────────────────────────────────────

impl Scope<Empty> {
    pub fn empty() -> Self {
        Self::wrap(Empty)
    }

    pub fn load(self, config: Config) -> Scope<Uninitialized> {
        Scope::wrap(Uninitialized { config })
    }
}

impl Scope<Uninitialized> {
    /// Advance to host tier with caller-built HostInfra. Caller is
    /// responsible for validation and registry assembly — see
    /// [`ComposeScope::host`].
    pub fn to_host(self, host: Arc<HostInfra>) -> Scope<AtHost> {
        let Uninitialized { config } = self.inner;
        Scope::wrap(AtHost { config, host })
    }
}

impl Scope<AtHost> {
    /// Advance to a specific project, verifying its name is in the
    /// host's registry. Caller assembled the registry; scope just
    /// guarantees the named project is among them.
    pub fn to_project(self, brain: BrainName) -> Result<Scope<AtProject>, ScopeError> {
        let AtHost { config, host } = self.inner;
        let project = host
            .projects
            .get(&brain)
            .cloned()
            .ok_or(ScopeError::ProjectNotFound(brain))?;
        Ok(Scope::wrap(AtProject {
            config,
            host,
            project,
        }))
    }
}

impl Scope<AtProject> {
    /// Advance to a specific bookmark, verifying its name is in the
    /// project's registry.
    pub fn to_bookmark(self, name: BookmarkName) -> Result<Scope<AtBookmark>, ScopeError> {
        let AtProject {
            config,
            host,
            project,
        } = self.inner;
        let bookmark = project
            .bookmarks
            .get(&name)
            .cloned()
            .ok_or(ScopeError::BookmarkNotFound(name))?;
        Ok(Scope::wrap(AtBookmark {
            config,
            host,
            project,
            bookmark,
        }))
    }
}

// ─────────────────────────────────────────────────────────────────────
// Operations — substrate ops at each tier. Open connections per-call;
// no held resources.
// ─────────────────────────────────────────────────────────────────────

impl Scope<AtHost> {
    /// Open the system database. Each tier names the DB it touches —
    /// `host_db` opens system.db, never the bookmark DB.
    pub async fn host_db(&self) -> Result<HostDb, HostDbError> {
        HostDb::open(&self.inner.config.platform()).await
    }

    /// Strangler bridge — produce a legacy `HostLog`. Shrinks as
    /// consumers move to use Scope ops directly.
    pub fn host_log(&self) -> HostLog {
        HostLog::new(self.inner.config.clone())
    }

    pub fn config(&self) -> &Config {
        &self.inner.config
    }
    pub fn host(&self) -> &HostInfra {
        &self.inner.host
    }
}

impl Scope<AtProject> {
    /// Open this project's events database.
    pub async fn events_db(&self) -> Result<EventsDb, EventsDbError> {
        EventsDb::open(&self.inner.config.platform(), &self.inner.project.name).await
    }

    pub fn config(&self) -> &Config {
        &self.inner.config
    }
    pub fn host(&self) -> &HostInfra {
        &self.inner.host
    }
    pub fn project(&self) -> &ProjectInfra {
        &self.inner.project
    }
}

impl Scope<AtBookmark> {
    /// Open the bookmark DB with the events DB ATTACHed.
    pub async fn bookmark_db(&self) -> Result<BookmarkDb, BookmarkDbError> {
        BookmarkDb::open(
            &self.inner.config.platform(),
            &self.inner.project.name,
            &self.inner.bookmark.name,
        )
        .await
    }

    /// Open the system DB from the bookmark tier (shared host DB).
    pub async fn host_db(&self) -> Result<HostDb, HostDbError> {
        HostDb::open(&self.inner.config.platform()).await
    }

    /// Strangler bridge — produce a legacy `ProjectLog` with fresh
    /// chronicle/broadcast (CLI-shape). HTTP construction handles its
    /// own behavior state (with_entry, with_broadcast) until the
    /// extractor migrates.
    pub fn project_log(&self) -> ProjectLog {
        ProjectLog::new(self.inner.config.clone())
    }

    /// Strangler bridge — produce a legacy `HostLog`.
    pub fn host_log(&self) -> HostLog {
        HostLog::new(self.inner.config.clone())
    }

    pub fn config(&self) -> &Config {
        &self.inner.config
    }
    pub fn host(&self) -> &HostInfra {
        &self.inner.host
    }
    pub fn project(&self) -> &ProjectInfra {
        &self.inner.project
    }
    pub fn bookmark(&self) -> &BookmarkInfra {
        &self.inner.bookmark
    }
}

// ─────────────────────────────────────────────────────────────────────
// ComposeScope — the factory paired with Scope.
//
// Knows how to read filesystem, build Infra structs, and walk the
// scope ladder. Lives wherever Scope is wielded. Today: filesystem
// enumeration. Follow-up: intersect with system-DB projection tables
// (BrainStore::list, BookmarkStore::list_for_brain) for authoritative
// capability.
// ─────────────────────────────────────────────────────────────────────

pub struct ComposeScope {
    config: Config,
}

impl ComposeScope {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Build a host-tier scope: validate `data_dir`, enumerate brain
    /// directories, assemble HostInfra with each brain's resolved
    /// paths and (empty) bookmark map.
    pub fn host(&self) -> Result<Scope<AtHost>, ComposeError> {
        let host = self.build_host_infra()?;
        Ok(Scope::empty()
            .load(self.config.clone())
            .to_host(Arc::new(host)))
    }

    /// Build a project-tier scope for a specific brain. Climbs to
    /// host, verifies the brain exists, enumerates its bookmarks,
    /// and attaches the populated ProjectInfra.
    pub fn project(&self, brain: BrainName) -> Result<Scope<AtProject>, ComposeError> {
        let mut host = self.build_host_infra()?;
        let project = host
            .projects
            .remove(&brain)
            .ok_or_else(|| ComposeError::Scope(ScopeError::ProjectNotFound(brain.clone())))?;
        let project = self.populate_bookmarks(&project)?;
        host.projects.insert(brain.clone(), Arc::new(project));

        let host_arc = Arc::new(host);
        let host_scope = Scope::empty().load(self.config.clone()).to_host(host_arc);
        Ok(host_scope.to_project(brain)?)
    }

    /// Build a bookmark-tier scope. Climbs to project, verifies the
    /// bookmark exists, attaches.
    pub fn bookmark(
        &self,
        brain: BrainName,
        name: BookmarkName,
    ) -> Result<Scope<AtBookmark>, ComposeError> {
        let project_scope = self.project(brain)?;
        Ok(project_scope.to_bookmark(name)?)
    }

    fn build_host_infra(&self) -> Result<HostInfra, ComposeError> {
        let platform = self.config.platform();
        if !platform.data_dir().is_dir() {
            return Err(ComposeError::HostHydrationFailed(format!(
                "data_dir does not exist: {}",
                platform.data_dir().display()
            )));
        }

        // Authoritative source: the `brains` projection in system DB.
        // System recognizes a brain when an event made it real; the
        // filesystem is the underlying medium. Intersection means
        // both must agree.
        let conn = self.config.system_db()?;
        let projection_names = BrainStore::new(&conn).list()?;

        let mut projects = HashMap::new();
        for name in projection_names {
            // System says the brain exists; verify it's actually
            // reachable on disk. Mismatch = orphan, exclude.
            if !platform.events_db_path(&name).exists() {
                continue;
            }
            let project = ProjectInfra {
                name: name.clone(),
                brain_dir: platform.brain_dir(&name),
                events_db_path: platform.events_db_path(&name),
                bookmarks_dir: platform.bookmarks_dir(&name),
                bookmarks: HashMap::new(),
            };
            projects.insert(name, Arc::new(project));
        }

        Ok(HostInfra {
            data_dir: platform.data_dir().to_path_buf(),
            system_db_path: platform.system_db_path(),
            host_key_path: platform.host_key_path(),
            projects,
        })
    }

    fn populate_bookmarks(&self, project: &ProjectInfra) -> Result<ProjectInfra, ComposeError> {
        // Authoritative source: `bookmarks` projection scoped to
        // brain. Filesystem must agree.
        let platform = self.config.platform();
        let conn = self.config.system_db()?;
        let projection_names = BookmarkStore::new(&conn).list_for_brain(&project.name)?;

        let mut bookmarks = HashMap::new();
        for name in projection_names {
            let bookmark_db_path = platform.bookmark_db_path(&project.name, &name);
            if !bookmark_db_path.exists() {
                // Orphan: projection knows the bookmark but no DB on
                // disk. Exclude.
                continue;
            }
            bookmarks.insert(
                name.clone(),
                Arc::new(BookmarkInfra {
                    name,
                    bookmark_db_path,
                }),
            );
        }

        Ok(ProjectInfra {
            bookmarks,
            ..project.clone()
        })
    }
}

// ─────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_config(dir: &TempDir) -> Config {
        Config::builder().data_dir(dir.path().to_path_buf()).build()
    }

    /// Seed a brain on both sides of the intersection: filesystem
    /// (brain dir + events.db) AND projection (`brains` row).
    fn seed_brain(config: &Config, name: &str) -> PathBuf {
        let brain_dir = config.data_dir.join(name);
        std::fs::create_dir_all(&brain_dir).unwrap();
        std::fs::write(brain_dir.join("events.db"), b"").unwrap();

        let conn = config.system_db().unwrap();
        BrainStore::new(&conn).migrate().unwrap();
        conn.execute(
            "insert or replace into brains (id, name, created_at) values (?1, ?2, ?3)",
            rusqlite::params![format!("brain-{name}"), name, "2026-04-28T00:00:00"],
        )
        .unwrap();

        brain_dir
    }

    /// Seed a bookmark on both sides: filesystem (`bookmarks/{name}.db`)
    /// AND projection (`bookmarks` row scoped to brain).
    fn seed_bookmark(config: &Config, brain: &str, name: &str) {
        let bookmarks_dir = config.data_dir.join(brain).join("bookmarks");
        std::fs::create_dir_all(&bookmarks_dir).unwrap();
        std::fs::write(bookmarks_dir.join(format!("{name}.db")), b"").unwrap();

        let conn = config.system_db().unwrap();
        BookmarkStore::new(&conn).migrate().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO bookmarks (id, brain, name, created_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                format!("bookmark-{brain}-{name}"),
                brain,
                name,
                "2026-04-28T00:00:00"
            ],
        )
        .unwrap();
    }

    #[test]
    fn missing_data_dir_fails_at_host_compose() {
        let config = Config::builder()
            .data_dir(PathBuf::from("/this/path/does/not/exist"))
            .build();
        let result = ComposeScope::new(config).host();
        assert!(matches!(result, Err(ComposeError::HostHydrationFailed(_))));
    }

    #[test]
    fn host_compose_intersects_projection_and_filesystem() -> Result<(), ComposeError> {
        let dir = TempDir::new().unwrap();
        let config = test_config(&dir);
        seed_brain(&config, "real-brain");

        // Orphan in projection but no events.db on disk.
        let conn = config.system_db().unwrap();
        BrainStore::new(&conn).migrate().unwrap();
        conn.execute(
            "insert or replace into brains (id, name, created_at) values (?1, ?2, ?3)",
            rusqlite::params!["brain-orphan", "orphan", "2026-04-28T00:00:00"],
        )
        .unwrap();

        // Filesystem-only dir without a projection row.
        std::fs::create_dir_all(dir.path().join("ghost")).unwrap();
        std::fs::write(dir.path().join("ghost").join("events.db"), b"").unwrap();

        let scope = ComposeScope::new(config).host()?;
        let host = scope.host();
        assert!(host.projects.contains_key(&BrainName::from("real-brain")));
        assert!(!host.projects.contains_key(&BrainName::from("orphan")));
        assert!(!host.projects.contains_key(&BrainName::from("ghost")));
        Ok(())
    }

    #[test]
    fn host_compose_with_uninitialized_system_db_returns_empty() -> Result<(), ComposeError> {
        // data_dir exists but no projection migrated yet — cold start.
        let dir = TempDir::new().unwrap();
        let scope = ComposeScope::new(test_config(&dir)).host()?;
        assert!(scope.host().projects.is_empty());
        Ok(())
    }

    #[test]
    fn project_compose_unknown_brain_errors() {
        let dir = TempDir::new().unwrap();
        let result = ComposeScope::new(test_config(&dir)).project(BrainName::from("nope"));
        assert!(matches!(
            result,
            Err(ComposeError::Scope(ScopeError::ProjectNotFound(_)))
        ));
    }

    #[test]
    fn project_compose_attaches_known_brain() -> Result<(), ComposeError> {
        let dir = TempDir::new().unwrap();
        let config = test_config(&dir);
        seed_brain(&config, "alpha");

        let scope = ComposeScope::new(config).project(BrainName::from("alpha"))?;
        assert_eq!(scope.project().name, BrainName::from("alpha"));
        assert!(scope.project().events_db_path.starts_with(dir.path()));
        Ok(())
    }

    #[test]
    fn bookmark_compose_picks_existing_bookmark() -> Result<(), ComposeError> {
        let dir = TempDir::new().unwrap();
        let config = test_config(&dir);
        seed_brain(&config, "alpha");
        seed_bookmark(&config, "alpha", "main");

        let scope =
            ComposeScope::new(config).bookmark(BrainName::from("alpha"), BookmarkName::main())?;
        assert_eq!(scope.bookmark().name, BookmarkName::main());
        Ok(())
    }

    #[test]
    fn bookmark_compose_unknown_bookmark_errors() {
        let dir = TempDir::new().unwrap();
        let config = test_config(&dir);
        seed_brain(&config, "alpha");

        let result = ComposeScope::new(config)
            .bookmark(BrainName::from("alpha"), BookmarkName::from("nope"));
        assert!(matches!(
            result,
            Err(ComposeError::Scope(ScopeError::BookmarkNotFound(_)))
        ));
    }
}
