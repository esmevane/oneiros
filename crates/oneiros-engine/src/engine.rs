//! Engine bootstrap — the single entry point for the oneiros engine.
//!
//! `Engine` owns everything needed to run: databases, projections, contexts,
//! and the knowledge of how to set itself up. Both direct access (CLI, tests)
//! and the HTTP server go through `Engine`.

use std::path::{Path, PathBuf};

use rusqlite::Connection;

use crate::*;

// ── Projection constants ────────────────────────────────────────

/// All project-scoped projections in registration order.
static PROJECT_PROJECTIONS: &[&[Projection]] = &[
    LevelProjections.all(),
    TextureProjections.all(),
    SensationProjections.all(),
    NatureProjections.all(),
    PersonaProjections.all(),
    UrgeProjections.all(),
    AgentProjections.all(),
    CognitionProjections.all(),
    MemoryProjections.all(),
    ExperienceProjections.all(),
    ConnectionProjections.all(),
    PressureProjections.all(),
    StorageProjections.all(),
    SearchProjections.all(),
];

/// All system-scoped projections in registration order.
static SYSTEM_PROJECTIONS: &[&[Projection]] = &[
    TenantProjections.all(),
    ActorProjections.all(),
    BrainProjections.all(),
    TicketProjections.all(),
];

// ── Engine ──────────────────────────────────────────────────────

/// The oneiros engine — owns bootstrap, databases, and contexts.
///
/// Create with `Engine::init(path)` for filesystem-backed databases,
/// or `Engine::in_memory()` for tests. Then call `init_project(name)`
/// to activate the brain-scoped context.
pub struct Engine {
    system: SystemContext,
    pub(crate) project: Option<ProjectContext>,
    brain_name: String,
    data_dir: PathBuf,
}

impl Engine {
    /// Bootstrap an engine from a directory path.
    ///
    /// Opens (or creates) the system database, runs migrations, and
    /// constructs the system context. The project context is not yet
    /// available — call `init_project` to activate it.
    pub fn init(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        std::fs::create_dir_all(path)
            .map_err(|e| Error::Context(format!("create engine dir: {e}")))?;

        let db_path = path.join("system.db");
        let conn = Connection::open(&db_path)
            .map_err(|e| Error::Context(format!("open system db: {e}")))?;

        migrate_system(&conn)?;

        let system = SystemContext::new(conn, SYSTEM_PROJECTIONS);

        Ok(Self {
            system,
            project: None,
            brain_name: String::new(),
            data_dir: path.to_path_buf(),
        })
    }

    /// Bootstrap an in-memory engine for tests.
    ///
    /// Both system and project databases live in memory — no filesystem
    /// needed. The project context is not yet available.
    pub fn in_memory() -> Result<Self, Error> {
        let conn = Connection::open_in_memory()
            .map_err(|e| Error::Context(format!("open in-memory db: {e}")))?;

        migrate_system(&conn)?;

        let system = SystemContext::new(conn, SYSTEM_PROJECTIONS);

        Ok(Self {
            system,
            project: None,
            brain_name: String::new(),
            data_dir: PathBuf::new(),
        })
    }

    /// Activate the project (brain) context.
    ///
    /// Opens (or creates) the brain database, runs migrations, and makes
    /// project-scoped commands available. Uses the default service address.
    pub fn init_project(&mut self, name: impl Into<String>) -> Result<(), Error> {
        self.init_project_with_config(name, None)
    }

    /// Activate the project with a specific service address.
    ///
    /// Use this when the server binds to a dynamic port (e.g. port 0 in tests)
    /// and the client needs to know where to connect.
    pub fn init_project_with_addr(
        &mut self,
        name: impl Into<String>,
        addr: std::net::SocketAddr,
    ) -> Result<(), Error> {
        self.init_project_with_config(name, Some(addr))
    }

    fn init_project_with_config(
        &mut self,
        name: impl Into<String>,
        addr: Option<std::net::SocketAddr>,
    ) -> Result<(), Error> {
        self.brain_name = name.into();

        let conn = if self.data_dir.as_os_str().is_empty() {
            Connection::open_in_memory()
        } else {
            let brain_db = self.data_dir.join("brain.db");
            Connection::open(&brain_db)
        }
        .map_err(|e| Error::Context(format!("open brain db: {e}")))?;

        migrate_project(&conn)?;

        let data_dir = if self.data_dir.as_os_str().is_empty() {
            None
        } else {
            let dir = self.data_dir.join("data");
            std::fs::create_dir_all(&dir)
                .map_err(|e| Error::Context(format!("create data dir: {e}")))?;
            Some(dir)
        };

        let mut project = ProjectContext::new(conn, PROJECT_PROJECTIONS);

        if let Some(dir) = data_dir {
            let mut config = Config::new(dir);
            if let Some(addr) = addr {
                config = config.with_service_addr(addr);
            }
            project = project.with_config(config);
        }

        self.project = Some(project);
        Ok(())
    }

    /// Execute a CLI command against this engine.
    pub async fn execute(&self, command: &Command) -> Result<Response<Responses>, Error> {
        command.execute(self).await
    }

    /// Access the system context.
    pub fn system(&self) -> &SystemContext {
        &self.system
    }

    /// Access the project context, if initialized.
    pub fn project(&self) -> Result<&ProjectContext, Error> {
        self.project.as_ref().ok_or_else(|| {
            Error::Context("project context required — call init_project first".to_string())
        })
    }

    /// The brain name, if a project has been initialized.
    pub fn brain_name(&self) -> &str {
        &self.brain_name
    }

    /// Build the project-scoped HTTP router.
    pub fn project_router(&self) -> Result<axum::Router, Error> {
        Ok(project_router(self.project()?.clone()))
    }

    /// Build the system-scoped HTTP router.
    pub fn system_router(&self) -> axum::Router {
        system_router(self.system.clone())
    }
}
