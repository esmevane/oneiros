pragma journal_mode = WAL;
pragma foreign_keys = ON;

-- Brain-level event log
--
-- Each brain has its own event log, independent of the service-level log.
-- This enables portability: a brain.db file can be exported, imported,
-- forked, or merged by set-union of events by UUID.
--
create table if not exists events (
    id        text primary key default (uuid()) not null,
    timestamp text not null default (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    data      text not null default '{}',
    meta      text not null default '{ "type": "__unmarked" }'
);

create index if not exists events_timestamp on events(timestamp);
create index if not exists events_type on events(json_extract(meta, '$.type'));

-- Personas are named roles that subagents in a brain adopt. They carry
-- a description (human-readable purpose) and a prompt (the system prompt
-- or instruction text the agent uses when adopting this persona).
--
-- Personas are identified by name. The upsert pattern uses ON CONFLICT
-- to allow set-or-replace semantics.
--
create table if not exists persona (
    name        text primary key not null,
    description text not null default '',
    prompt      text not null default ''
);

-- Textures are cognitive categories that classify agent thoughts. Each
-- texture carries a description (human-readable purpose) and a prompt
-- (guidance text for agents when logging cognition with this texture).
--
-- Examples: observation, learning, insight, hope, fear, bond, reflection.
-- Textures are emergent — they can be seeded, added, or removed over time.
--
create table if not exists texture (
    name        text primary key not null,
    description text not null default '',
    prompt      text not null default ''
);

-- Agents are named participants in a brain's cognition. Each agent adopts
-- a persona (FK to persona.name) and may carry its own description and
-- prompt that specialize or override the persona's defaults.
--
-- Agents are identified by a UUID primary key for stable FK references
-- from cognition and memory, and carry a unique human-readable name for
-- the user-facing API surface.
--
create table if not exists agent (
    id          text primary key not null,
    name        text unique not null,
    persona     text not null references persona(name),
    description text not null default '',
    prompt      text not null default ''
);

-- Levels are memory retention tiers that determine how memories surface
-- in agent context. Each level carries a description (human-readable
-- purpose) and a prompt (guidance for agents when assigning memories to
-- this level).
--
-- Examples: core (always included), active (included by default),
-- passive (on request), archived (excluded unless retrieved).
-- Levels are emergent — they can be seeded, added, or removed over time.
--
create table if not exists level (
    name        text primary key not null,
    description text not null default '',
    prompt      text not null default ''
);
