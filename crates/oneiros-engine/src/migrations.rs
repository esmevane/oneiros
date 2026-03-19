//! Migration orchestration — runs all domain migrations.

use rusqlite::Connection;

use crate::*;

/// Initialize the event store and run all project-scoped migrations.
pub fn migrate_project(conn: &Connection) -> Result<(), EventError> {
    event::repo::migrate(conn)?;

    // Vocabulary domains
    LevelRepo::new(conn).migrate()?;
    TextureRepo::new(conn).migrate()?;
    SensationRepo::new(conn).migrate()?;
    NatureRepo::new(conn).migrate()?;
    PersonaRepo::new(conn).migrate()?;
    UrgeRepo::new(conn).migrate()?;

    // Entity domains
    AgentRepo::new(conn).migrate()?;
    CognitionRepo::new(conn).migrate()?;
    MemoryRepo::new(conn).migrate()?;
    ExperienceRepo::new(conn).migrate()?;
    ConnectionRepo::new(conn).migrate()?;

    // Derived / infrastructure
    PressureRepo::new(conn).migrate()?;
    SearchRepo::new(conn).migrate()?;
    StorageRepo::new(conn).migrate()?;

    Ok(())
}

/// Initialize the event store and run all system-scoped migrations.
pub fn migrate_system(conn: &Connection) -> Result<(), EventError> {
    event::repo::migrate(conn)?;

    TenantRepo::new(conn).migrate()?;
    ActorRepo::new(conn).migrate()?;
    BrainRepo::new(conn).migrate()?;
    TicketRepo::new(conn).migrate()?;

    Ok(())
}
