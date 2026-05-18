# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.12](https://github.com/esmevane/oneiros/compare/oneiros-engine-v0.0.11...oneiros-engine-v0.0.12) - 2026-05-18

### 🐛 Fixes

- [patch] Update skill stragglers (#271)

The overhaul to commands a few commits back left some stragglers in its
document update portion. That led to agents which occasionally
dead-ended in background tasks and things like that. This commit makes
an effort to catch any more stragglers we may have missed in that push.

Release-Type: fix

- [patch] Runtime project resolution (#268)

This changes our project resolution to work at runtime, instead of at
build time. Whoops!

We also change some of our docs so that they have the proper
continuity-nested command invocations.

Release-Type: fix
Breaking-Change: false

- [patch] Request-forward http. (#267)

This commit inverts our client design, so that we rely on the request
structures themselves in order to make the http requests. We do this by
defining a ClientRequest trait that takes a &Client and returns a future
byte payload. Then, our requests implement those individually, and the
surrounding Cli leverages it.

Release-Type: refactor

- [patch] Error hygiene again. (#266)

More passes on the error hygiene train, where we look through all of our
error setups and ask ourselves, "Did we try to do this right or did we
bail out for the sake of convenience?" We're doing okay, but we did find
some! This commit updates the cases we found in the bridge errors.

Release-Type: fix

- [patch] Auth errors (#265)

In landing the auth updates, we introduced quite a few stringly typed
errors and strangely obfuscated (aka, lazy, half-implemented) errors
around token validation. We also slapped a bunch of struct errors into
things - a practice we've made a point to avoid in the past. This commit
mainly focuses on correcting all of that.

Release-Type: fix


### 🚀 Features

- [minor] MCP Doctor improvements (#269)

The MCP checks in the `doctor` function weren't validating the token in
the MCP file, but they sure were suggesting that they did! This commit
does a little work to actually, you know, verify the token and that it
works.

Release-Type: feature

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

- [minor] Update evening/morning pages (#262)

We want the model to know it's good to build a bit of muscle memory with
its continuity tooling, so during morning pages we try to habituate it
to navigating through stuff and looking around. It works pretty well, so
far!

Release-Type: feature
Breaking-Change: false


## [0.0.11](https://github.com/esmevane/oneiros/compare/oneiros-engine-v0.0.10...oneiros-engine-v0.0.11) - 2026-05-12

### Other

- Host-level auth ([#257](https://github.com/esmevane/oneiros/pull/257))
- Configuration wrap-up ([#259](https://github.com/esmevane/oneiros/pull/259))
- Configuration through figment. ([#254](https://github.com/esmevane/oneiros/pull/254))
- Oh, Clipford. ([#255](https://github.com/esmevane/oneiros/pull/255))
- CLI doesn't spit out log traces. ([#256](https://github.com/esmevane/oneiros/pull/256))
- Pub(crate) refactor. ([#251](https://github.com/esmevane/oneiros/pull/251))

## [0.0.10](https://github.com/esmevane/oneiros/compare/oneiros-engine-v0.0.9...oneiros-engine-v0.0.10) - 2026-05-05

### Other

- Messaging follow-up.
- Messaging
- Inits pass through http.
- Not eventually.
- Make fetch happen.
- Remove SSE "activity stream".
- MCP sessions.
- Database extraction.
- Platform determines layout.
- Extract keys from config.
- Scopes over context.
- Replayable bookmarks.
- Lists are searches.
- Versionable protocol.
- Update iroh, misc crates; temp pins.
- Dream compaction.
- Known, unknown, ephemeral, malformed events.
- Explicit event match arms
- Importable bookmarks.
- Host centric dashboard. ([#228](https://github.com/esmevane/oneiros/pull/228))
- Resource keys.
- Logging
- Bookmarks as lenses.
- Arc the API.
- Massive MCP reduction.
- Remove canon, leverage events for distribution.
- OpenAPI docs and Scalar page.
- Schema generation in MCP
- Hints use actual command paths.
- Hints
- Structured errors.
- Seeds use a client.
- Use WAL for dbs
- Move acceptance tests into engine.
- Move bin code into engine.
- SSE test flake fix.
- Explicit errors. Generate keypair in host init.
- Distribution feedback.
- Distribution.
- Create a `main` bookmark on project create.
- Enum centric ergonomics.
- Viewable resources.
- Distribution workflow tests.
- Bookmark MCP & MCP Auth.
- MCP Quality of Life improvements
- Fixes for import speed and host migrations.
- Morning / evening pages, the skills.
- Bookmarks and chronicles.
- Rehydrate canon at start.
- Reconciliation.
- Reductionism.
- Reducers and split canon.
- Morning pages v3
- Loro-capable tables.
- Basic loro support.
- More morning page work.
- Morning pages, ref tokens.

## [0.0.9](https://github.com/esmevane/oneiros/compare/oneiros-engine-v0.0.8...oneiros-engine-v0.0.9) - 2026-04-03

### Other

- Produce actual lists on CLI calls.
- Test overhaul.
- Revised dashboard.
- Setup / quality of life commands.
- Update skill generation.
- Remove the legacy work.
- Only the engine.
- Resource smoothing.
- Smooth out skill language.
- Oneiros Engine
