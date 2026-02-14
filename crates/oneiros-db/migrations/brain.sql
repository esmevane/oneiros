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

-- Cognitions are the thoughts agents log. Each cognition is bound to an
-- agent (who thought it) and a texture (what kind of thought it is).
-- Cognitions are append-only — they are created but never updated or deleted.
--
create table if not exists cognition (
    id          text primary key not null,
    agent_id    text not null references agent(id),
    texture     text not null references texture(name),
    content     text not null,
    created_at  text not null
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

-- Memories are consolidated knowledge records tied to an agent and
-- classified by retention level. Like cognitions, memories are
-- append-only — created but never updated or deleted.
--
create table if not exists memory (
    id          text primary key not null,
    agent_id    text not null references agent(id),
    level       text not null references level(name),
    content     text not null,
    created_at  text not null
);

-- Blobs are content-addressable binary storage. Each blob is identified
-- by its SHA-256 hash and stores zlib-compressed data. Blobs are NOT
-- projections — they are a peer data source alongside the event log.
-- Deduplication is automatic: same content = same hash = one row.
--
create table if not exists blob (
    hash text primary key not null,
    data blob not null,
    size integer not null default 0
);

-- Storage entries map user-facing keys to content hashes. This table
-- is a projection from storage-set and storage-removed events.
--
create table if not exists storage (
    key         text primary key not null,
    description text not null default '',
    hash        text not null references blob(hash)
);
