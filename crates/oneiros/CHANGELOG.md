# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.12](https://github.com/esmevane/oneiros/compare/v0.0.11...v0.0.12) - 2026-05-18

### 🚀 Features

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
