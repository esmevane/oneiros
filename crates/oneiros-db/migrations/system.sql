pragma journal_mode = WAL;
pragma foreign_keys = ON;

-- Event log
--
-- Events are a source of truth for the database, reflecting a log of
-- the operations which the service has executed. Events are append-only,
-- and they're meant to be left alone for the most part, while many other
-- tables are replayable.
--
create table if not exists events (
    id        text primary key default (uuid()) not null,
    timestamp text not null default (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    data      text not null default '{}',
    meta      text not null default '{ "type": "__unmarked" }'
);

create index if not exists events_timestamp on events(timestamp);
create index if not exists events_type on events(json_extract(meta, '$.type'));

-- Cursors track each projection and worker's position in the event log. A new
-- projection starts from the current epoch. In order to trigger a full replay,
-- delete the projection's row.
--
create table if not exists cursors (
    projection    text primary key,
    last_event_id text not null,
    timestamp     text not null,
    updated_at    text not null default (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- Tasks are multi-step workflow tracking markers.
--
-- Workers create and manage tasks to orchestrate asynchronous or long-running
-- work. The task runner picks up a task from this table and advances them
-- through their steps until completion. Tasks carry their own idea of a cursor
-- so that they can track event state internally, but they aren't meant to be
-- replayed, so they don't rely on the main table.
--
-- Tasks have a log concept. This is meant to be structured reporting data on
-- the log to keep it easy to review work history.
--
create table if not exists task_status (
    label       text primary key,
    description text
);

insert into task_status (label, description) values
    ('pending',   'Task is waiting to be picked up by the runner.'),
    ('running',   'Task is currently being executed.'),
    ('completed', 'Task finished successfully.'),
    ('failed',    'Task failed. Check error column for details. May be retried.');

create table if not exists tasks (
    id         text primary key default (uuid()) not null,
    kind       text not null,
    status     text not null references task_status(name) default 'pending',
    input      text not null default '{}',
    cursor     text not null default '{}',
    log        text default '[]',
    created_at text not null default (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at text not null default (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

create index if not exists tasks_status on tasks(status);
create index if not exists tasks_kind on tasks(kind);

-- Projections
--
-- Many of the below tables are materialized views of the event log, and
-- serve mostly to create projections of things that the system has done.
-- They are largely replayable, and so it is possible for ids to drift in
-- some cases between replays.
--

-- Tenants are a top-level organizational boundary. Tenants are arbitrary
-- markers. They can represent actors of any kind: services, organizations,
-- individuals. They can also represent lots of actors. Tenancy is mostly
-- useful for establishing a chain of command for ticket grants and things
-- of that nature.
--
-- For example, a single user would also be a single tenant. Meanwhile, an
-- org with lots of shared oneiroi, automation, etc, would be a tenant with
-- multiple actors.
--
create table if not exists tenant (
    id   text primary key default (uuid()) not null,
    name text not null,
    link text
);

-- Actors are distinct identities that interact with brains through ticket
-- issued capabilities. They belong to a tenant and they can be granted
-- access to things the tenant governs through ticket issuance.
--
create table if not exists actor (
    id         text primary key default (uuid()) not null,
    tenant_id  text not null references tenant(id) on delete cascade,
    name       text not null,
    link       text,

    UNIQUE(tenant_id, name)
);

create index if not exists actor_tenant_index on actor(tenant_id);

-- Brains are the registry of all oneiroi governed by a host, mostly
-- involving the curation of the host system's database files.
--
-- Each brain points to a discrete sqlite file stored on disk or in a
-- different repo of some kind. The host manages the lifecycle of the
-- brain (create, suspend, archive, destroy, merge, fork, etc.) and
-- manages this table via event projection in order to track the work,
-- but doesn't reach into the brain. That's the job of the oneiroi and
-- individual actors.
--
create table if not exists status (
    label       text primary key,
    description text
);

insert into status (label, description) values
    ('active',    'The brain is active and can be accessed normally.'),
    ('suspended', 'The brain is temporarily suspended and cannot be accessed.'),
    ('archived',  'The brain is archived and stored for long-term retention.');

create table if not exists brain (
    id         text primary key default (uuid()) not null,
    tenant_id  text not null references tenant(id) on delete cascade,
    name       text not null,
    path       text not null, -- path to brain.db file
    status     text not null references status(label) default 'active',
    link       text,

    unique(tenant_id, name)
);

create index if not exists brain_tenant_index on brain(tenant_id);
create index if not exists brain_status_index on brain(status);

-- Tickets are an access and control capability system.
--
-- They follow a chain of custody where ultimate authority to do
-- something is granted during brain initialization to the tenant
-- which controls the brain, and then granted to the user steering
-- the tenant.
--
-- Ticket chain of custody is limited by the upper bounds of the
-- chain, meaning if more recent tickets ostensibly grant a given
-- capability which the topmost issuer doesn't have, the system will
-- deny access to that capability.
--
-- Tokens contain most metadata required to self-authenticate and verify.
-- Permissions are a cache of the set of permissions granted for searching.
-- Granted by points to the ticket issuer - if nil, it's a root grant.
-- Without a row, actors are not permitted to access the brain via client.
--
create table if not exists tickets (
  id          text primary key default (uuid()) not null,
  token       text not null,
  granted_by  text references tickets(id),

  permissions text default '[]' not null,
  created_by  text not null references actor(id),
  expires_at  text,
  revoked_on  text,

  max_uses integer,
  uses     integer default 0 not null
);

create unique index if not exists unique_tickets_token on tickets (token);
