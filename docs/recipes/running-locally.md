# Running Multiple Instances Locally

Debugging oneiros locally can be risky if you're also using an oneiroi. So, if you want to do that, the most direct way is to run it with multiple instances, each pointing at different data directories.

The keys here are:

- Running it on different ports
- Pointing at different data directories
- If you're running it off-project, giving it a `--brain` param so it uses that as the project name instead of inferring it

## Why?

You might want to test import / export stuff, rehearse upgrades to see if there are any issues before committing to them, practice a version upcast, or simulate multiple hosts for distribution purposes. It might be mostly for contributors, but it's totally possible there's good reasons to do it as a user, too.

## How?

For both the server itself, and any CLI usage against the server, you'll add the `--data-dir <path> --address <ip:port>` args to your oneiros commands, basically.

- The data dir points you at a new storage location folder, which is where the event log and projections and config all go.
- Address is an ip (not host!) to bind to, usually `127.0.0.1`, but there's no defaults.

Two different instances with two different data directories and addresses won't ever see each other, or share db files, so no contention.

### Server

For example, kicking off a new instance of the server:

```sh
oneiros service run
```

That works against defaults. Meanwhile, this starts it up on a custom port with a scratchpad tmp directory:

```sh
oneiros service run \
  --data-dir /tmp/oneiros-scratch \
  --address 127.0.0.1:8081
```

### Client

Much the same as the server, running the client has defaults, so:

```sh
oneiros cognition list
```

Gets a list of the most recent cognitions from the default oneiros instance. So, to see what's on your fresh instance, use the same args:

```sh
oneiros cognition list \
  --data-dir /tmp/oneiros-scratch \
  --address 127.0.0.1:8081
```

## Local export / import

A pretty common reason to do this is to test out exports locally before doing anything with the main install.

```sh
# Export from the production install (defaults)
oneiros project export --target /tmp/export-dir

# Import into the scratch install
oneiros project import /tmp/export-dir/<brain>-<date>-export.jsonl \
  --data-dir /tmp/oneiros-scratch \
  --brain <brain>
```

## Peeking under the hood

Oneiros uses sqlite3 for persistence so you can take a gander at the databases if you like. It can be faster than using the CLI sometimes if you know where everything is, since you don't have to stand the server up to see what's in the db files.

```sh
# See the event log for a given brain.
sqlite3 /tmp/oneiros-scratch/<brain>/events.db \
  'select count(*) from events'

# See the table schemas for the brain's "main" bookmark.
sqlite3 /tmp/oneiros-scratch/<brain>/bookmarks/main.db \
  '.tables'

# See the total cognitions for the brain's "main" bookmark.
sqlite3 /tmp/oneiros-scratch/<brain>/bookmarks/main.db \
  'select count(*) from cognitions'
```

## Common gotchas

**`--address` is a no-op for local-DB commands.** `project import`, `project export`, and `project replay` open the data directory directly — they do not route through HTTP. Passing `--address 127.0.0.1:8081` on these commands does *not* send them to the instance on 8081. Use `--data-dir` to target a specific instance's data.

**Brain name is auto-detected from cwd.** Running `oneiros project import foo.jsonl` from `/tmp` will create a brain called `tmp`. Pass `--brain <name>` explicitly when running from an unrelated directory, or `cd` into a directory whose basename matches the target brain.

**Ticket/token are per-instance.** An imported brain has no ticket on the destination until one is issued. HTTP queries against the imported brain will return `401 Missing authorization header` until a token is in place. Direct CLI commands that open the DB locally work regardless. You can conjure a token with `oneiros project init --brain <brain>`.
