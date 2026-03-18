// Vocabulary domains (project-scoped)
pub mod level;
pub mod nature;
pub mod persona;
pub mod sensation;
pub mod texture;
pub mod urge;

// Entity domains (project-scoped)
pub mod agent;
pub mod cognition;
pub mod connection;
pub mod experience;
pub mod memory;

// Derived / cross-domain (project-scoped)
pub mod pressure;

// Lifecycle operations (project-scoped, compose other domains)
pub mod lifecycle;

// System-scoped domains
pub mod actor;
pub mod brain;
pub mod tenant;
pub mod ticket;

// Project-scoped infrastructure domains
pub mod search;
pub mod storage;
