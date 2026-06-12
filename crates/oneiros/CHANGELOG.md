# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.12](https://github.com/esmevane/oneiros/compare/v0.0.11...v0.0.12) - 2026-06-12

### 🚀 Features

- [minor] Dashboard (#272)

This commit establishes the node package setup for our dashboard, docs
page, component library, dashboard harness, its react bindings, and our
design token definitions. It also puts into play the linting and
formatting setup we're hoping to go with, and puts those into CI.

Here's the main topology:

- In xtask, we added some commands to leverage the build and get them to
  the binary when we build it.
- In workflows, we added CI/CD setup that uses those commands.
- In the repo root, we created a pnpm workspace that describes the apps
  and packages directories.
- apps/dashboard is the home of the new dashboard, which is meant to
  replace the existing one in the oneiros-engine crate
- apps/docs is the home of our documentation website, which is where
  we'll put reference, workflows, tutorials, etc
- packages/components is our component library, and has an overall setup
  like a whole bunch of other kinds of react component libraries,
  nothing incredibly noteworthy except that these are visual bindings of
  the components, mostly, not behavioral (yet)
- packages/loom is a formalization of an xstate-first set of dashboard
  primitives that I use in almost every project I work in; this is kind
  of an audition of the loom concept in general, which begins with a
  config file and emits a sophisticated logical harness for common UI
  patterns in applications
- packages/loom-react binds those to React context, etc
- packages/tokens is just a shipped css of the tokens emitted from its
  inner config, managed by the tangible rust crate

All in all it _seems_ like a lot, but it isn't! It's mostly boilerplate
setup with some basic defaults and tool integrations. There's just a lot
of parts involved in the whole thing. This resolves the Dashboard
charter to a considerable degree, though there's still some work
remaining, I suspect.

Release-Type: feature
Breaking-Change: false

- [minor] Merge project/brain and system/service/host (#264)

We've been wanting to merge "project" and "brain", and "system",
"service", and "host" for a while now, and that's what this commit
does.

- Project and "brain" were always essentially the same thing, but
  differentiating the two of these resulted in a lot of weird behavior
  with the robot, where they would enshrine the idea of a brain and sort
  of work to talk about it everywhere. Not altogether a problem, except
  that it made me realize that I'd rather talk about brains when I have
  a reason to actually model something like that out.
- System/Service/Host - this is sort of the same thing, where a host is
  how I spoke about all of these things and so having multiple
  conceptual wrappers around it was making it unclear what I wanted in
  many cases.

Release-Type: feature


## [0.0.10](https://github.com/esmevane/oneiros/compare/v0.0.9...v0.0.10) - 2026-05-05

### Other

- Structured errors.
- Move bin code into engine.
- Viewable resources.

## [0.0.9](https://github.com/esmevane/oneiros/compare/v0.0.8...v0.0.9) - 2026-04-03

### Other

- Update skill generation.
- Only the engine.

## [0.0.8](https://github.com/esmevane/oneiros/compare/v0.0.7...v0.0.8) - 2026-03-13

### Other

- Test harness.
- Add bon to context.
- HttpService.
- Unify dispatch callers.
- MCP pressure Resource, pressure CTAs.
- Pressure CLI support.
- Urges
- General fixes & observability.

## [0.0.7](https://github.com/esmevane/oneiros/compare/v0.0.6...v0.0.7) - 2026-03-09

### Other

- Protocol pass.
- Cursor-based event access.
- Blob storage represented in export/import.
- Import Events via serde
- Source is included in events.
- Durable events.
- Http / Service crate split.

## [0.0.6](https://github.com/esmevane/oneiros/compare/v0.0.5...v0.0.6) - 2026-03-02

### Other

- HTTP & Activity feed.
- Dream config.
- Dream collector.
- Search.
- Connections.
- Refs.
- Core memories.
- Flatten out the shape wrappers.
- Eventful shapes.
- Move protocol package to model.
- Export project events to jsonl.
- Flatten commands and model.
- Permit key access to row ids.
- Event commands.
- Timestamp.
- Formalize links.
- Connections and natures.
- Linked refs.
- Migrate entities to identity structures.
- Migrate agent to identity structure.
- Untagged refs.
- Agent lifecycle work.
- Robust agent integration.
- Move projections to service crate.
- Introduce oneiroi.process
- Improve ergonomics for common operations.
- release v0.0.5-rc4.1

## [0.0.5](https://github.com/esmevane/oneiros/compare/v0.0.5-rc4...v0.0.5) - 2026-02-16

### Other

- Feedback pass.
- Experiences.

## [0.0.4](https://github.com/esmevane/oneiros/compare/oneiros-v0.0.3...oneiros-v0.0.4) - 2026-02-14

### Other

- update Cargo.lock dependencies

## [0.0.2](https://github.com/esmevane/oneiros/compare/oneiros-v0.0.1...oneiros-v0.0.2) - 2026-02-14

### Added

- oneiros project create
- Host init, readme, migrations.
- Project detection

### Other

- Skill support.
- Dream, introspect, reflect.
- Rename {report_outcome,structured_output}
- Structured output.
- Service manager.
- Storage
- Memory.
- Cognition.
- Agents.
- Memory levels.
- Textures.
- Outcomes, as a macro.
- Personas and tickets.
- Outcomes.
- Fix commands module typo.
- Remove todo
- Add dist.
