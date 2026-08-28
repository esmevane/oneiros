//! Scope — typestate ladder for capability via graduation.
//!
//! Scope is pure shape. Reaching a tier IS the capability —
//! `Scope<AtBookmark>` exists because the bookmark tier was actually
//! reached, not because a constructor declared it.
//!
//! Scope's transitions take pre-built data and verify trivial
//! invariants (e.g. that a Pick name lives in the registry it was
//! handed). Scope does NOT fetch from the filesystem or query the
//! host DB. That work belongs to [`ComposeScope`], the factory
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

    pub(crate) fn with_config_and_databases(
        self,
        config: Config,
        databases: Databases,
    ) -> Scope<Configured> {
        Scope::wrap(Configured { config, databases })
    }
}

#[derive(Clone)]
pub(crate) struct Configured {
    config: Config,
    databases: Databases,
}

impl Scope<Configured> {
    /// Transition scope to its [`AtHost`] tier, which means it has
    /// access to everything it needs to manage a host instance, and
    /// we've verified that the host instance can run.
    ///
    pub(crate) fn verify_host(self, host: Arc<HostInfra>) -> Scope<AtHost> {
        let Configured { config, databases } = self.inner;
        Scope::wrap(AtHost {
            config,
            databases,
            host,
        })
    }
}

#[derive(Clone)]
pub(crate) struct AtHost {
    config: Config,
    databases: Databases,
    host: Arc<HostInfra>,
}

#[derive(Clone)]
pub(crate) struct AtProject {
    config: Config,
    databases: Databases,
    project: Arc<ProjectInfra>,
}

#[derive(Clone)]
pub(crate) struct AtBookmark {
    config: Config,
    databases: Databases,
    project: Arc<ProjectInfra>,
    bookmark: Arc<BookmarkInfra>,
}

// Capability markers.
//
// Each trait says "this scope tier carries enough info to open the
// resources at <its> tier." DB types take `&impl HasHost` /
// `&impl HasProject` / `&impl HasBookmark` and ask scope for what
// they need — scope is shape, db is opening. The hierarchical bounds
// (`HasProject: HasHost`, `HasBookmark: HasProject`) mean lower tiers
// can open higher-tier resources for free.

pub(crate) trait HasHost {
    fn config(&self) -> &Config;
    fn databases(&self) -> &Databases;

    /// Check out a handle to the host database.
    async fn host_db(&self) -> Result<DbHandle<'_>, DbError> {
        self.databases().host().await
    }
}

pub(crate) trait HasProject: HasHost {
    fn project(&self) -> &ProjectInfra;

    /// Check out a handle to this project's event log.
    async fn project_log(&self) -> Result<DbHandle<'_>, DbError> {
        self.databases().project_log(&self.project().name).await
    }
}

pub(crate) trait HasBookmark: HasProject {
    fn bookmark(&self) -> &BookmarkInfra;

    /// Check out a handle to this bookmark's projection database.
    async fn bookmark_db(&self) -> Result<DbHandle<'_>, DbError> {
        self.databases()
            .bookmark(&self.project().name, &self.bookmark().name)
            .await
    }
}

// Resource bundles
//
// Hold paths and registry data. Never connections — those are per-call
// work at the operation layer.

#[derive(Clone)]
pub(crate) struct HostInfra {
    pub(crate) projects: HashMap<ProjectName, Arc<ProjectInfra>>,
}

#[derive(Clone)]
pub(crate) struct ProjectInfra {
    pub(crate) name: ProjectName,
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
    ProjectNotFound(ProjectName),

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

    #[error(transparent)]
    Db(#[from] DbError),
}

// Scope transitions — pure structural advances. No fetching.

impl Scope<AtHost> {
    /// Advance to a specific project, verifying its name is in the
    /// host's registry. Caller assembled the registry; scope just
    /// guarantees the named project is among them.
    pub(crate) fn verify_project(
        self,
        project: ProjectName,
    ) -> Result<Scope<AtProject>, ScopeError> {
        let AtHost {
            config,
            databases,
            host,
        } = self.inner;
        let project = host
            .projects
            .get(&project)
            .cloned()
            .ok_or(ScopeError::ProjectNotFound(project))?;
        Ok(Scope::wrap(AtProject {
            config,
            databases,
            project,
        }))
    }
}

impl Scope<AtProject> {
    /// Advance to a specific bookmark, verifying its name is in the
    /// project's registry.
    pub(crate) fn verify_bookmark(
        self,
        name: BookmarkName,
    ) -> Result<Scope<AtBookmark>, ScopeError> {
        let AtProject {
            config,
            databases,
            project,
        } = self.inner;
        let bookmark = project
            .bookmarks
            .get(&name)
            .cloned()
            .ok_or(ScopeError::BookmarkNotFound(name))?;
        Ok(Scope::wrap(AtBookmark {
            config,
            databases,
            project,
            bookmark,
        }))
    }
}

// Operations — substrate ops at each tier. Open connections per-call;
// no held resources.

impl HasHost for Scope<AtHost> {
    fn config(&self) -> &Config {
        &self.inner.config
    }
    fn databases(&self) -> &Databases {
        &self.inner.databases
    }
}

impl HasHost for Scope<AtProject> {
    fn config(&self) -> &Config {
        &self.inner.config
    }
    fn databases(&self) -> &Databases {
        &self.inner.databases
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
    fn databases(&self) -> &Databases {
        &self.inner.databases
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

// ComposeScope factory.
//
// Knows how to read filesystem, build Infra structs, and walk the
// scope ladder. Lives wherever Scope is wielded. Today: filesystem
// enumeration. Follow-up: intersect with host-DB projection tables
// (ProjectStore::list, BookmarkStore::list_for_project) for authoritative
// capability.

pub(crate) struct ComposeScope {
    config: Config,
    databases: Databases,
}

impl ComposeScope {
    pub(crate) fn new(config: Config, databases: Databases) -> Self {
        Self { config, databases }
    }

    /// Build a host-tier scope: validate `data_dir`, enumerate project
    /// directories, assemble HostInfra with each project's resolved
    /// paths and (empty) bookmark map.
    pub(crate) async fn host(&self) -> Result<Scope<AtHost>, ComposeError> {
        let host = self.build_host_infra().await?;
        Ok(Scope::empty()
            .with_config_and_databases(self.config.clone(), self.databases.clone())
            .verify_host(Arc::new(host)))
    }

    /// Build a project-tier scope for a specific project. Climbs to
    /// host, verifies the project exists, enumerates its bookmarks,
    /// and attaches the populated ProjectInfra.
    pub(crate) async fn project(
        &self,
        name: ProjectName,
    ) -> Result<Scope<AtProject>, ComposeError> {
        let mut host = self.build_host_infra().await?;
        let project = host
            .projects
            .remove(&name)
            .ok_or_else(|| ComposeError::Scope(ScopeError::ProjectNotFound(name.clone())))?;
        let project = self.populate_bookmarks(&project).await?;
        host.projects.insert(name.clone(), Arc::new(project));

        let host_arc = Arc::new(host);
        let host_scope = Scope::empty()
            .with_config_and_databases(self.config.clone(), self.databases.clone())
            .verify_host(host_arc);
        Ok(host_scope.verify_project(name)?)
    }

    /// Build a bookmark-tier scope. Climbs to project, verifies the
    /// bookmark exists, attaches.
    pub(crate) async fn bookmark(
        &self,
        project: ProjectName,
        name: BookmarkName,
    ) -> Result<Scope<AtBookmark>, ComposeError> {
        let project_scope = self.project(project).await?;
        Ok(project_scope.verify_bookmark(name)?)
    }

    async fn build_host_infra(&self) -> Result<HostInfra, ComposeError> {
        let platform = self.config.platform();
        if !platform.data_dir().is_dir() {
            return Err(ComposeError::HostHydrationFailed(format!(
                "data_dir does not exist: {}",
                platform.data_dir().display()
            )));
        }

        // Authoritative source: the `projects` projection in host DB.
        // The host recognizes a project when an event made it real; the
        // filesystem is the underlying medium. Intersection means
        // both must agree.
        let conn = self.databases.host().await?;
        let projection_names = ProjectStore::new(&conn).list()?;

        let mut projects = HashMap::new();
        for name in projection_names {
            // The host says the project exists; verify it's actually
            // reachable. In file mode, check the filesystem. In memory
            // mode, trust the projection — the db lives in the pool.
            if self.config.database.mode == DatabaseMode::File
                && !platform.events_db_path(&name).exists()
            {
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

    async fn populate_bookmarks(
        &self,
        project: &ProjectInfra,
    ) -> Result<ProjectInfra, ComposeError> {
        // Authoritative source: `bookmarks` projection scoped to
        // project. Filesystem must agree.
        let platform = self.config.platform();
        let conn = self.databases.host().await?;
        let projection_names = BookmarkStore::new(&conn).list_for_project(&project.name)?;

        let mut bookmarks = HashMap::new();
        for name in projection_names {
            // In file mode, verify the bookmark DB exists on disk.
            // In memory mode, trust the projection.
            if self.config.database.mode == DatabaseMode::File {
                let bookmark_db_path = platform.bookmark_db_path(&project.name, &name);
                if !bookmark_db_path.exists() {
                    continue;
                }
            }
            bookmarks.insert(name.clone(), Arc::new(BookmarkInfra { name }));
        }

        Ok(ProjectInfra {
            bookmarks,
            ..project.clone()
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tempfile::TempDir;

    use super::*;

    fn test_config(dir: &TempDir) -> Config {
        Config::builder().data_dir(dir.path().to_path_buf()).build()
    }

    /// Seed a project into the database via the pool — filesystem
    /// (project dir + events.db) AND projection (`projects` row).
    async fn seed_project(databases: &Databases, config: &Config, name: &str) {
        let platform = config.platform();
        let project_dir = config.data_dir.join(name);
        platform.ensure_dir(&project_dir).unwrap();
        platform.write(project_dir.join("events.db"), b"").unwrap();

        let conn = databases.host().await.unwrap();
        ProjectStore::new(&conn).migrate().unwrap();
        conn.execute(
            "insert or replace into projects (id, name, created_at) values (?1, ?2, ?3)",
            rusqlite::params![format!("project-{name}"), name, "2026-04-28T00:00:00"],
        )
        .unwrap();
    }

    /// Seed a bookmark into the database via the pool — filesystem
    /// (`bookmarks/{name}.db`) AND projection (`bookmarks` row).
    async fn seed_bookmark(databases: &Databases, config: &Config, project: &str, name: &str) {
        let platform = config.platform();
        let bookmarks_dir = config.data_dir.join(project).join("bookmarks");
        platform.ensure_dir(&bookmarks_dir).unwrap();
        platform
            .write(bookmarks_dir.join(format!("{name}.db")), b"")
            .unwrap();

        let conn = databases.host().await.unwrap();
        BookmarkStore::new(&conn).migrate().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO bookmarks (id, project, name, created_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                format!("bookmark-{project}-{name}"),
                project,
                name,
                "2026-04-28T00:00:00"
            ],
        )
        .unwrap();
    }

    #[tokio::test]
    async fn missing_data_dir_fails_at_host_compose() {
        let config = Config::builder()
            .data_dir(PathBuf::from("/this/path/does/not/exist"))
            .build();
        let databases = Databases::new(config.clone());
        let result = ComposeScope::new(config, databases).host().await;
        assert!(matches!(result, Err(ComposeError::HostHydrationFailed(_))));
    }

    #[tokio::test]
    async fn project_compose_unknown_project_errors() {
        let dir = TempDir::new().unwrap();
        let config = test_config(&dir);
        let databases = Databases::new(config.clone());
        let result = ComposeScope::new(config, databases)
            .project(ProjectName::from("nope"))
            .await;
        assert!(matches!(
            result,
            Err(ComposeError::Scope(ScopeError::ProjectNotFound(_)))
        ));
    }

    #[tokio::test]
    async fn project_compose_attaches_known_project() -> Result<(), ComposeError> {
        let dir = TempDir::new().unwrap();
        let config = test_config(&dir);
        let databases = Databases::new(config.clone());
        seed_project(&databases, &config, "alpha").await;

        let scope = ComposeScope::new(config, databases)
            .project(ProjectName::from("alpha"))
            .await?;
        assert_eq!(scope.project().name, ProjectName::from("alpha"));

        Ok(())
    }

    #[tokio::test]
    async fn bookmark_compose_picks_existing_bookmark() -> Result<(), ComposeError> {
        let dir = TempDir::new().unwrap();
        let config = test_config(&dir);
        let databases = Databases::new(config.clone());
        seed_project(&databases, &config, "alpha").await;
        seed_bookmark(&databases, &config, "alpha", "main").await;

        let scope = ComposeScope::new(config, databases)
            .bookmark(ProjectName::from("alpha"), BookmarkName::main())
            .await?;
        assert_eq!(scope.bookmark().name, BookmarkName::main());
        Ok(())
    }

    #[tokio::test]
    async fn bookmark_compose_unknown_bookmark_errors() {
        let dir = TempDir::new().unwrap();
        let config = test_config(&dir);
        let databases = Databases::new(config.clone());
        seed_project(&databases, &config, "alpha").await;

        let result = ComposeScope::new(config, databases)
            .bookmark(ProjectName::from("alpha"), BookmarkName::from("nope"))
            .await;
        assert!(matches!(
            result,
            Err(ComposeError::Scope(ScopeError::BookmarkNotFound(_)))
        ));
    }
}
