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
