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

use std::{collections::HashMap, sync::Arc};

use crate::*;

#[derive(Clone)]
pub(crate) struct Scope<T> {
    inner: T,
}

impl<T> Scope<T> {
    fn wrap(inner: T) -> Self {
        Self { inner }
    }
}

#[derive(Clone, Default)]
pub(crate) struct Empty;

impl Scope<Empty> {
    pub(crate) fn empty() -> Self {
        Self::wrap(Empty)
    }

    pub(crate) fn with_config(self, config: Config) -> Scope<Configured> {
        Scope::wrap(Configured { config })
    }
}

#[derive(Clone)]
pub(crate) struct Configured {
    config: Config,
}

impl Scope<Configured> {
    /// Transition scope to its [`AtHost`] tier, which means it has
    /// access to everything it needs to manage a host instance, and
    /// we've verified that the host instance can run.
    ///
    pub(crate) fn verify_host(self, host: Arc<HostInfra>) -> Scope<AtHost> {
        let Configured { config } = self.inner;
        Scope::wrap(AtHost { config, host })
    }
}

#[derive(Clone)]
pub(crate) struct AtHost {
    config: Config,
    host: Arc<HostInfra>,
}

#[derive(Clone)]
pub(crate) struct AtProject {
    config: Config,
    project: Arc<ProjectInfra>,
}

#[derive(Clone)]
pub(crate) struct AtBookmark {
    config: Config,
    project: Arc<ProjectInfra>,
    bookmark: Arc<BookmarkInfra>,
}

// ─────────────────────────────────────────────────────────────────────
// Capability markers.
//
// Each trait says "this scope tier carries enough info to open the
// resources at <its> tier." DB types take `&impl HasHost` /
// `&impl HasProject` / `&impl HasBookmark` and ask scope for what
// they need — scope is shape, db is opening. The hierarchical bounds
// (`HasProject: HasHost`, `HasBookmark: HasProject`) mean lower tiers
// can open higher-tier resources for free.
// ─────────────────────────────────────────────────────────────────────

pub(crate) trait HasHost {
    fn config(&self) -> &Config;
}

pub(crate) trait HasProject: HasHost {
    fn project(&self) -> &ProjectInfra;
}

pub(crate) trait HasBookmark: HasProject {
    fn bookmark(&self) -> &BookmarkInfra;
}

// ─────────────────────────────────────────────────────────────────────
// Resource bundles
//
// Hold paths and registry data. Never connections — those are per-call
// work at the operation layer.
// ─────────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub(crate) struct HostInfra {
    pub(crate) projects: HashMap<BrainName, Arc<ProjectInfra>>,
}

#[derive(Clone)]
pub(crate) struct ProjectInfra {
    pub(crate) name: BrainName,
    pub(crate) bookmarks: HashMap<BookmarkName, Arc<BookmarkInfra>>,
}

#[derive(Clone)]
pub(crate) struct BookmarkInfra {
    pub(crate) name: BookmarkName,
}

// ─────────────────────────────────────────────────────────────────────
// Errors
// ─────────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub(crate) enum ScopeError {
    #[error("project not found in registry: {0}")]
    ProjectNotFound(BrainName),

    #[error("bookmark not found in registry: {0}")]
    BookmarkNotFound(BookmarkName),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ComposeError {
    #[error("host hydration failed: {0}")]
    HostHydrationFailed(String),

    #[error(transparent)]
    Scope(#[from] ScopeError),

    #[error(transparent)]
    Database(#[from] rusqlite::Error),
}

// ─────────────────────────────────────────────────────────────────────
// Scope transitions — pure structural advances. No fetching.
// ─────────────────────────────────────────────────────────────────────

impl Scope<AtHost> {
    /// Advance to a specific project, verifying its name is in the
    /// host's registry. Caller assembled the registry; scope just
    /// guarantees the named project is among them.
    pub(crate) fn verify_project(self, brain: BrainName) -> Result<Scope<AtProject>, ScopeError> {
        let AtHost { config, host } = self.inner;
        let project = host
            .projects
            .get(&brain)
            .cloned()
            .ok_or(ScopeError::ProjectNotFound(brain))?;
        Ok(Scope::wrap(AtProject { config, project }))
    }
}

impl Scope<AtProject> {
    /// Advance to a specific bookmark, verifying its name is in the
    /// project's registry.
    pub(crate) fn verify_bookmark(
        self,
        name: BookmarkName,
    ) -> Result<Scope<AtBookmark>, ScopeError> {
        let AtProject { config, project } = self.inner;
        let bookmark = project
            .bookmarks
            .get(&name)
            .cloned()
            .ok_or(ScopeError::BookmarkNotFound(name))?;
        Ok(Scope::wrap(AtBookmark {
            config,
            project,
            bookmark,
        }))
    }
}

// ─────────────────────────────────────────────────────────────────────
// Operations — substrate ops at each tier. Open connections per-call;
// no held resources.
// ─────────────────────────────────────────────────────────────────────

impl HasHost for Scope<AtHost> {
    fn config(&self) -> &Config {
        &self.inner.config
    }
}

impl HasHost for Scope<AtProject> {
    fn config(&self) -> &Config {
        &self.inner.config
    }
}

impl HasProject for Scope<AtProject> {
    fn project(&self) -> &ProjectInfra {
        &self.inner.project
    }
}

impl HasHost for Scope<AtBookmark> {
    fn config(&self) -> &Config {
        &self.inner.config
    }
}

impl HasProject for Scope<AtBookmark> {
    fn project(&self) -> &ProjectInfra {
        &self.inner.project
    }
}

impl HasBookmark for Scope<AtBookmark> {
    fn bookmark(&self) -> &BookmarkInfra {
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

pub(crate) struct ComposeScope {
    config: Config,
}

impl ComposeScope {
    pub(crate) fn new(config: Config) -> Self {
        Self { config }
    }

    /// Build a host-tier scope: validate `data_dir`, enumerate brain
    /// directories, assemble HostInfra with each brain's resolved
    /// paths and (empty) bookmark map.
    pub(crate) fn host(&self) -> Result<Scope<AtHost>, ComposeError> {
        let host = self.build_host_infra()?;
        Ok(Scope::empty()
            .with_config(self.config.clone())
            .verify_host(Arc::new(host)))
    }

    /// Build a project-tier scope for a specific brain. Climbs to
    /// host, verifies the brain exists, enumerates its bookmarks,
    /// and attaches the populated ProjectInfra.
    pub(crate) fn project(&self, brain: BrainName) -> Result<Scope<AtProject>, ComposeError> {
        let mut host = self.build_host_infra()?;
        let project = host
            .projects
            .remove(&brain)
            .ok_or_else(|| ComposeError::Scope(ScopeError::ProjectNotFound(brain.clone())))?;
        let project = self.populate_bookmarks(&project)?;
        host.projects.insert(brain.clone(), Arc::new(project));

        let host_arc = Arc::new(host);
        let host_scope = Scope::empty()
            .with_config(self.config.clone())
            .verify_host(host_arc);
        Ok(host_scope.verify_project(brain)?)
    }

    /// Build a bookmark-tier scope. Climbs to project, verifies the
    /// bookmark exists, attaches.
    pub(crate) fn bookmark(
        &self,
        brain: BrainName,
        name: BookmarkName,
    ) -> Result<Scope<AtBookmark>, ComposeError> {
        let project_scope = self.project(brain)?;
        Ok(project_scope.verify_bookmark(name)?)
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
                bookmarks: HashMap::new(),
            };
            projects.insert(name, Arc::new(project));
        }

        Ok(HostInfra { projects })
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
            bookmarks.insert(name.clone(), Arc::new(BookmarkInfra { name }));
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
    use std::path::PathBuf;
    use tempfile::TempDir;

    use super::*;

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
