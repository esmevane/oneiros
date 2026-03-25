//! Engine bootstrap — the single entry point for the oneiros engine.
//!
//! `Engine` owns everything needed to run: databases, buses, contexts,
//! and the knowledge of how to set itself up. Both direct access (CLI,
//! tests) and the HTTP server go through `Engine`.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::event_bus::EventBus;
use crate::event_log::EventLog;
use crate::*;

// ── Frame definitions ──────────────────────────────────────────

/// Project-scoped frames, ordered by dependency.
fn project_frames() -> Vec<Frame> {
    vec![
        // Frame 0: Base entity materialization
        Frame::new(LevelProjections.all()),
        Frame::new(TextureProjections.all()),
        Frame::new(SensationProjections.all()),
        Frame::new(NatureProjections.all()),
        Frame::new(PersonaProjections.all()),
        Frame::new(UrgeProjections.all()),
        Frame::new(AgentProjections.all()),
        Frame::new(CognitionProjections.all()),
        Frame::new(MemoryProjections.all()),
        Frame::new(ExperienceProjections.all()),
        Frame::new(ConnectionProjections.all()),
        Frame::new(StorageProjections.all()),
        // Frame 1: Derived / cross-domain (depends on frame 0 entities)
        Frame::new(PressureProjections.all()),
        Frame::new(SearchProjections.all()),
    ]
}

/// System-scoped frames.
fn system_frames() -> Vec<Frame> {
    vec![
        Frame::new(TenantProjections.all()),
        Frame::new(ActorProjections.all()),
        Frame::new(BrainProjections.all()),
        Frame::new(TicketProjections.all()),
    ]
}

/// Bootstrap a bus + frames for a database connection.
///
/// Creates the bus, runs EventLog and projection migrations,
/// constructs Frames, spawns the FrameRunner task, and returns
/// both the bus and a Frames handle for replay operations.
fn bootstrap(conn: Connection, frame_defs: Vec<Frame>) -> Result<(EventBus, Frames), Error> {
    let db = Arc::new(Mutex::new(conn));

    // Run EventLog migration
    {
        let conn = db.lock().expect("db lock");
        EventLog::new(&conn)
            .migrate()
            .map_err(|e| Error::Context(format!("event log migration: {e}")))?;
    }

    // Create bus (gets the dispatch sender)
    let (bus, receiver) = EventBus::new(db.clone());

    // Create and migrate Frames
    let frames = Frames::new(frame_defs, db);
    frames
        .migrate()
        .map_err(|e| Error::Context(format!("projection migration: {e}")))?;

    // Spawn FrameRunner as a background task (clone of Frames for live events)
    let runner = FrameRunner::new(frames.clone(), receiver);
    tokio::spawn(runner.run());

    Ok((bus, frames))
}

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
    /// Opens (or creates) the system database, creates the bus and
    /// Frames, runs migrations, and spawns the projection task.
    pub fn init(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        std::fs::create_dir_all(path)
            .map_err(|e| Error::Context(format!("create engine dir: {e}")))?;

        let db_path = path.join("system.db");
        let conn = Connection::open(&db_path)
            .map_err(|e| Error::Context(format!("open system db: {e}")))?;

        let (bus, _frames) = bootstrap(conn, system_frames())?;
        let system = SystemContext::new(bus);

        Ok(Self {
            system,
            project: None,
            brain_name: String::new(),
            data_dir: path.to_path_buf(),
        })
    }

    /// Bootstrap an in-memory engine for tests.
    pub fn in_memory() -> Result<Self, Error> {
        let conn = Connection::open_in_memory()
            .map_err(|e| Error::Context(format!("open in-memory db: {e}")))?;

        let (bus, _frames) = bootstrap(conn, system_frames())?;
        let system = SystemContext::new(bus);

        Ok(Self {
            system,
            project: None,
            brain_name: String::new(),
            data_dir: PathBuf::new(),
        })
    }

    /// Activate the project (brain) context.
    pub fn init_project(&mut self, name: impl Into<String>) -> Result<(), Error> {
        self.init_project_with_config(name, None)
    }

    /// Activate the project with a specific service address.
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

        let data_dir = if self.data_dir.as_os_str().is_empty() {
            None
        } else {
            let dir = self.data_dir.join("data");
            std::fs::create_dir_all(&dir)
                .map_err(|e| Error::Context(format!("create data dir: {e}")))?;
            Some(dir)
        };

        let (bus, frames) = bootstrap(conn, project_frames())?;
        let mut project = ProjectContext::new(bus, frames);

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
    pub async fn execute(&self, command: &Command) -> Result<Rendered<Responses>, Error> {
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
    pub fn brain_name(&self) -> BrainName {
        BrainName::new(&self.brain_name)
    }

    /// Build the project-scoped HTTP router.
    pub fn project_router(&self) -> Result<axum::Router, Error> {
        Ok(project_router(self.project()?.clone()))
    }

    /// Build the system-scoped HTTP router.
    pub fn system_router(&self) -> axum::Router {
        system_router(self.system.clone())
    }

    /// The service address — from project config or default.
    pub fn service_addr(&self) -> std::net::SocketAddr {
        self.project
            .as_ref()
            .and_then(|p| p.config().map(|c| c.service_addr))
            .unwrap_or_else(|| std::net::SocketAddr::from(([127, 0, 0, 1], 2100)))
    }

    /// The engine data directory.
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }
}
