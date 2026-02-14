# Architecture

Oneiros is structured as a client-server system with event-sourced persistence.

## Component Overview

```
CLI (oneiros) → Client → Unix Socket → Service (Axum) → SQLite
```

### CLI (`oneiros`)

The command-line interface parses arguments via clap and delegates to command modules. Each command produces an `Outcomes<T>` stream that gets reported through the configured output format (prompt, quiet, or json).

### Client (`oneiros-client`)

A typed HTTP client that communicates with the service over a Unix domain socket. Provides methods for all brain operations: `set_texture`, `set_level`, `set_persona`, `create_agent`, `add_cognition`, `add_memory`, `dream`, `introspect`, `reflect`, etc.

All mutating operations require a token obtained from `project init`.

### Service (`oneiros-service`)

An Axum web server listening on a Unix socket at `~/.local/share/oneiros/oneiros.sock`. Routes requests to handlers that interact with the database layer.

### Database (`oneiros-db`)

Event-sourced SQLite storage using per-brain database files. All mutations are recorded as events; read models (tables) are projections derived from the event stream.

Brain databases live at `~/.local/share/oneiros/brains/<brain-name>.db`.

## Data Flow

### Write Path

1. CLI parses command → calls client method
2. Client sends HTTP request over Unix socket
3. Service handler validates request
4. Database records event and updates projections
5. Response returned through the stack

### Read Path (Dream)

1. CLI calls `oneiros dream <agent>`
2. Client fetches agent, persona, textures, levels, cognitions, memories
3. Template engine assembles all data into a single prompt
4. Prompt returned as command output

## Workspace Crates

| Crate | Purpose |
|-------|---------|
| `oneiros` | CLI binary and command implementations |
| `oneiros-client` | Typed Unix socket HTTP client |
| `oneiros-service` | Axum service and route handlers |
| `oneiros-db` | Event-sourced SQLite persistence |
| `oneiros-model` | Domain types (entities, values, events) |
| `oneiros-outcomes` | Outcome reporting framework with derive macro |
| `oneiros-outcomes-derive` | Proc macro for `#[derive(Outcome)]` |
| `oneiros-templates` | Tera templates for dream/introspect/reflect |
| `oneiros-skill` | Skill artifacts, seed data, and plugin definitions |
| `oneiros-fs` | Filesystem utilities and path resolution |
| `oneiros-detect-project-name` | Project name auto-detection |
| `oneiros-terminal` | Terminal interaction utilities |

## Key Design Decisions

- **Event sourcing** — All mutations are events, enabling audit trails and replay
- **Per-brain databases** — Each project gets an isolated SQLite file
- **Unix socket** — Local-only communication, no network exposure
- **Idempotent upserts** — Set operations (texture, level, persona) are safe to repeat
- **Template-driven prompts** — Dream/introspect/reflect use Tera templates for flexibility
- **Outcome-driven reporting** — Commands emit typed outcomes for structured output
