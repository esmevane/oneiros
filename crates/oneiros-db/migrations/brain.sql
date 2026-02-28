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

-- Sensations are categories that classify relationships between
-- records. Like textures and levels, they carry a description and a
-- prompt, and are emergent — they can be seeded, added, or removed.
--
-- Examples: caused, continues, echoes, tensions, crystallized-from.
--
create table if not exists sensation (
    name        text primary key not null,
    description text not null default '',
    prompt      text not null default ''
);

-- Natures classify edges in the cognitive graph. Like textures,
-- levels, and sensations, they carry a description and a prompt,
-- and are emergent — seeded, added, or removed over time.
--
-- Examples: origin, result, context, evidence, continuation.
--
create table if not exists nature (
    name        text primary key not null,
    description text not null default '',
    prompt      text not null default ''
);

-- Connections are first-class edges between entities in the cognitive
-- graph. Each connection links two entities (via Refs) through a nature
-- that describes the relationship type.
--
create table if not exists connection (
    id          text primary key not null,
    nature      text not null references nature(name),
    from_ref    text not null,
    to_ref      text not null,
    created_at  text not null
);

-- Experiences are descriptive edges connecting records in the brain.
-- Each experience is bound to an agent (who created it) and a
-- sensation (what type of relationship it describes).
-- Experiences can grow over time as new refs are added.
--
create table if not exists experience (
    id          text primary key not null,
    agent_id    text not null references agent(id),
    sensation   text not null references sensation(name),
    description text not null,
    created_at  text not null
);

-- Storage entries map user-facing keys to content hashes. This table
-- is a projection from storage-set and storage-removed events.
--
create table if not exists storage (
    key         text primary key not null,
    description text not null default '',
    hash        text not null references blob(hash)
);

-- Search expressions are normalized text fragments extracted from entities
-- by projections and indexed for full-text search. Each expression points
-- back to its source entity via resource_ref (a JSON-encoded Ref).
--
-- The kind discriminant allows filtering by expression source type
-- (e.g., 'cognition-content', 'memory-content', 'agent-description').
--
create table if not exists expressions (
    id            integer primary key,
    resource_ref  text not null,
    kind          text not null,
    content       text not null
);

create index if not exists idx_expressions_kind on expressions(kind);
create index if not exists idx_expressions_ref on expressions(resource_ref);

-- FTS5 virtual table using external content pattern. The expressions table
-- is the source of truth; the FTS index is kept in sync via triggers.
-- Tokenizer: porter stemming + unicode61 word splitting, prefix indexes
-- for 2 and 3 character prefixes.
--
create virtual table if not exists expression_search using fts5(
    content,
    content=expressions,
    content_rowid=id,
    tokenize='porter unicode61',
    prefix='2 3'
);

-- Sync triggers keep the FTS5 index consistent with the expressions table.
-- INSERT adds to the index; DELETE removes from it using FTS5's special
-- delete syntax.
--
create trigger if not exists expression_search_insert
    after insert on expressions
begin
    insert into expression_search(rowid, content) values (new.id, new.content);
end;

create trigger if not exists expression_search_delete
    after delete on expressions
begin
    insert into expression_search(expression_search, rowid, content)
    values('delete', old.id, old.content);
end;
