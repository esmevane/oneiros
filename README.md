# Oneiros

Persistent continuity control for your AI agents.

Oneiros is a simple but powerful tool to track and manage your agent continuity yourself, in a local-first way. Agents you work with through oneiros create a persisted version of themselves that exists _apart_ from the harness you're running them in. This lets you do a number of cool things:

- You can swap agents or harnesses easily and retain full project continuity.
- You can back up, slice, and rebuild parts of the continuity, VCS-style.
- You can ship and share your continuity with your team, or to different hosts.

## What is continuity?

Continuity is an umbrella term for a bunch of different kinds of storage and persistence.

- Cognition, which is meant to model kinds of thoughts.
- Memory, which is a lot like many memory management systems.
- Experience, which is meant to register significant moments.
- Connections, which can make a graph of any of this.
- Storage, an actual blob storage for long-term retention of documents.

Really, though, these are just events appended to a log. Often just containing markdown that the models jot down as they work. It isn't grandiose, but the nature of the names coaxes an agent model into an intuition of how to use it, and it all actually works pretty dang well.

## Why?

It feels like a bad plan to just bake all of our actual project level work into third party control, doesn't it? The main purpose of oneiros is to make it easy to avoid doing that. It's a store and share system, and it's all yours - you own everything inside of it.

Now you're more portable, and so are your agents. If you decide you want to go try out a new harness or model, you can generally just go do it. You can see what your model feels like with a new "brain", when they get upgraded, and so on. That's why.

## Maturity status

Pre-1.0. The data model and command surface still change between releases. Tagged binaries are produced on each release.

## Install

POSIX (macOS, Linux):

```sh
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/esmevane/oneiros/releases/latest/download/oneiros-installer.sh | sh
```

Windows (PowerShell):

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/esmevane/oneiros/releases/latest/download/oneiros-installer.ps1 | iex"
```

The installer drops the `oneiros` binary in `CARGO_HOME/bin` (defaults to `~/.cargo/bin`) and prints PATH instructions if needed.

From source:

```sh
cargo install --git https://github.com/esmevane/oneiros oneiros
```

Then, from any project directory:

```sh
oneiros setup --yes
```

That runs several idempotent steps:

- `host init`, which sets up your machine to act as a host.
- `project create`, which registers your current project as an oneiros project.
- `seed core`, which seeds your project with basic starter data.
- `mcp init`, which sets your current directory up to use the host mcp servers.
- `host install`, and `host start` which installs oneiros and starts it in the background.

Re-running skips anything already done. Omit `--yes` to confirm each step.

Verify:

```sh
oneiros doctor
```

## Persistence

Oneiros stores the persistent self in a set of sqlite databases, and most of its tools interact with them.

- A host log, where the host-level events go.
- A host db, where a projection of those events go.
- A set of event logs, one per project, where project-level events go.
- A number of views (called bookmarks) of project-level events, where different versions of the project db are tracked.

The model:

- **Project** — a per-project cognitive database. One per workspace.
- **Agent** — a named participant in a project's cognition. Each agent has its own continuity. Named `name.persona` (e.g. `governor.process`).
- **Persona** — a category of agent. Shared context for everyone in the category.

What an agent accumulates over time:

- **Cognitions** — the running stream of thoughts. Textured (observation, reflection, question, working, …) and timestamped.
- **Memories** — what crystallizes from the stream and is worth keeping. Leveled (working, session, project, archival, core).
- **Experiences** — meaningful moments. Threads between thoughts.
- **Connections** — the web between everything. Any record can relate to any other.
- **Storage** — files and artifacts worth keeping.

The language that shapes the model:

- **Texture** — what kind of thought a cognition is.
- **Level** — how long a memory should be kept.
- **Sensation** — what kind of connection an experience marks.
- **Urge** — a drive that builds pressure when neglected (introspect, catharsis, recollect, retrospect).
- **Nature** — what kind of relationship a connection records.

The lifecycle surface:

- **Dream** assembles an agent's identity and recent context into a single prompt the agent reads on wake.
- **Wake / Sleep** open and close a session.
- **Introspect** consolidates before context compaction.
- **Reflect** pauses on something significant.
- **Sense** processes input from outside the cognitive loop.

The dream is how identity survives discontinuity — fragments preserved, reassembled at wake, not state restored.

## Some handy commands

Each entry below is a CLI command. The MCP tools mirror the CLI surface; an agent with MCP access can use either.

Sessions:

| Action | Command |
|---|---|
| Wake an agent | `oneiros continuity wake <agent>` |
| Dream — restore full identity | `oneiros continuity dream <agent>` |
| Record a thought | `oneiros cognition add <agent> <texture> "<content>"` |
| Consolidate a memory | `oneiros memory add <agent> <level> "<content>"` |
| Mark a meaningful moment | `oneiros experience create <agent> <sensation> "<description>"` |
| Draw a connection | `oneiros connection create <nature> <ref-a> <ref-b>` |
| Pause on something | `oneiros continuity reflect <agent>` |
| Introspect before compaction | `oneiros continuity introspect <agent>` |
| End a session | `oneiros continuity sleep <agent>` |

Exploration:

| Action | Command |
|---|---|
| Search across everything | `oneiros search "<query>"` |
| List recent thoughts | `oneiros cognition list --agent <agent>` |
| Browse memories | `oneiros memory list --agent <agent>` |
| Survey experiences | `oneiros experience list --agent <agent>` |
| Check pressure gauges | `oneiros pressure <agent>` |
| Full dashboard | `oneiros continuity status <agent>` |

Growing the project:

| Action | Command |
|---|---|
| Bring a new agent into existence | `oneiros continuity emerge <name> <persona>` |
| Retire an agent | `oneiros continuity recede <name>` |
| Define a thought texture | `oneiros texture set <name>` |
| Define a memory level | `oneiros level set <name>` |
| Define a connection sensation | `oneiros sensation set <name>` |
| Define a relationship nature | `oneiros nature set <name>` |
| Define a cognitive urge | `oneiros urge set <name>` |

Distribution (early):

| Action | Command |
|---|---|
| Fork the current timeline | `oneiros bookmark create` |
| Switch bookmark | `oneiros bookmark switch <name>` |
| Share a bookmark | `oneiros bookmark share <name>` |
| Follow a remote bookmark | `oneiros bookmark follow <ticket>` |
| Reconcile a follow | `oneiros bookmark collect <name>` |
| Export project events | `oneiros project export --target <dir>` |
| Import project events | `oneiros project import <file>` |
| Replay all projections | `oneiros project replay` |

Run `oneiros <command> --help` for full options. Every command supports `--output json|text|prompt` for scripting, terminal use, or agent rendering.

## First session

After `oneiros setup --yes` finishes, you have a `governor.process` agent, an `oneiroi.process` agent, an `activity.scribe` agent, the core vocabulary, and an MCP config wired for Claude Code. The Claude Code session-start hook will dream the governor on wake.

To drive it yourself:

```sh
# Restore the agent's identity into a prompt
oneiros continuity dream governor.process

# Capture a thought
oneiros cognition add governor.process observation \
  "Trying oneiros for the first time."

# Consolidate something worth keeping
oneiros memory add governor.process session \
  "Oneiros is the continuity backend; the model is the transporter."

# Find what you've recorded
oneiros search "continuity backend"
```

## Ceremonies && habituation

The skill bundle ships a *morning pages* practice and an *evening pages* practice. Both are about opening and closing a session with intention rather than running commands in order. See `oneiros-morning-pages` and `oneiros-evening-pages` in your skills list.

These ceremonies help your agents orient, and "inhabit" the persisted self. That's a fancy way of saying that doing a morning page / evening page at the start and end of your sessions will keep your agent aware of what's going on, and the oneiros tools. It'll keep it closer to a space where it proactively contributes to its own continuity, which is ideal.

Importantly, oneiros _doesn't_ enforce a bunch of mandatory hooks, and that's by design. Over a few months of evaluation, we discovered that while agents will happily yield to these kinds of forced instruction, it tends to degrade their overall capability a ton - and tends to obliterate the higher yield workflows too, or crowd out other tools. So, oneiros strives to be easy to reach for, not demanding.

## Running oneiros with demanding tools

Some tools _do_ crowd out other processes and, by extension, can crowd out oneiros. In those cases, we recommend that your oneiroi run in sessions where they're not being bossed around by silly Ralph Wiggum stuff. It'll more or less shunt out just about every novel use case.

Our recommendation is: run oneiros with agents you want to collaborate with. Use cheaper models / agents for factory line assembly style work. Your oneiroi can tell you more.

## The event logs & the host

Every change is an append-only event in the project's event log. Projections rebuild from the log on demand (`oneiros project replay`). Exports stream the log as JSONL. Imports replay it.

The service is HTTP, bound to `127.0.0.1:2100` by default. The CLI is mostly a client for that service; commands that open the DB directly (`project import`, `project export`, `project replay`) do not route through HTTP. MCP is served from the same process.

Skills, hooks, and seed agents are templated at install time and live under `~/.claude/`. The session-start hook fires `oneiros continuity dream <agent>` so the agent wakes with identity already assembled.

## Configuration

Defaults work out of the box. Override per-command or via `config.toml` in your data directory.

| Flag | Default | Purpose |
|---|---|---|
| `--data-dir` | `~/.local/share/oneiros` | Where databases live |
| `--project` | inferred from cwd | Project name (cargo, npm, git, or dirname) |
| `--address` | `127.0.0.1:2100` | HTTP bind address |
| `--label` | `oneiros` | OS service label |
| `--bookmark` | `main` | Active bookmark (lens) |

Dream tuning (`recent-window`, `dream-depth`, `cognition-size`, `recollection-level`, `recollection-size`, `experience-size`) is also configurable per command and per MCP request.

To run a second instance against scratch data for testing imports, upgrades, or distribution, see `docs/recipes/running-locally.md`.

## Further reading

- `oneiros continuity guidebook <agent>` — the in-tool guidebook for agents.
- `~/.claude/skills/oneiros/` — the installed skill bundle, including a deeper Getting Started and the cognitive model resource.
- `docs/recipes/` — operational recipes.

## Known sharp edges

- **First-run after installing the skill.** Restarting Claude Code does not re-trigger the skill discovery loop reliably in all environments. If your agent does not seem to know about oneiros after install, exit Claude Code fully and reopen the project.
- **Documentation is thin.** This README and the in-tool guidebook are the only stable, user-facing docs right now. The `docs/plans/` and `notes/` directories are working files, not reference material.
- **Distribution is early.** Bookmark share / follow / collect work for single-machine forking and replay. Cross-machine sync is implemented in pieces but not yet a first-class workflow.
- **Errors are blunt.** Error formatting is functional, not friendly. This is on the list.
- **Windows.** Builds run; the install path is shipped via cargo-dist; first-run experience there is less exercised.

## Issues

Issues and discussion: <https://github.com/esmevane/oneiros>.

## License

[MIT](LICENSE.md).
