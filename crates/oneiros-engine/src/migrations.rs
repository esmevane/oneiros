//! Migration orchestration — runs all domain migrations.

use rusqlite::Connection;

use crate::domains;
use crate::store::{self, StoreError};

/// Initialize the event store and run all project-scoped migrations.
pub fn migrate_project(conn: &Connection) -> Result<(), StoreError> {
    store::initialize(conn)?;

    // Vocabulary domains
    domains::level::repo::LevelRepo::new(conn).migrate()?;
    domains::texture::repo::TextureRepo::new(conn).migrate()?;
    domains::sensation::repo::SensationRepo::new(conn).migrate()?;
    domains::nature::repo::NatureRepo::new(conn).migrate()?;
    domains::persona::repo::PersonaRepo::new(conn).migrate()?;
    domains::urge::repo::UrgeRepo::new(conn).migrate()?;

    // Entity domains
    domains::agent::repo::AgentRepo::new(conn).migrate()?;
    domains::cognition::repo::CognitionRepo::new(conn).migrate()?;
    domains::memory::repo::MemoryRepo::new(conn).migrate()?;
    domains::experience::repo::ExperienceRepo::new(conn).migrate()?;
    domains::connection::repo::ConnectionRepo::new(conn).migrate()?;

    // Derived / infrastructure
    domains::pressure::repo::PressureRepo::new(conn).migrate()?;
    domains::search::repo::SearchRepo::new(conn).migrate()?;
    domains::storage::repo::StorageRepo::new(conn).migrate()?;

    Ok(())
}

/// Initialize the event store and run all system-scoped migrations.
pub fn migrate_system(conn: &Connection) -> Result<(), StoreError> {
    store::initialize(conn)?;

    domains::tenant::repo::TenantRepo::new(conn).migrate()?;
    domains::actor::repo::ActorRepo::new(conn).migrate()?;
    domains::brain::repo::BrainRepo::new(conn).migrate()?;
    domains::ticket::repo::TicketRepo::new(conn).migrate()?;

    Ok(())
}
