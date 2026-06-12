# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.12](https://github.com/esmevane/oneiros/compare/oneiros-engine-v0.0.11...oneiros-engine-v0.0.12) - 2026-06-12

### 🐛 Fixes

- [patch] Cow town for skills (#292)

We're moving over to `Cow` for our strings in the skill structure,
mainly because we want to explore generating these things, and that
means we can't rely on exclusively static strings.

Release-Type: refactor

- [patch] Resource requests (#291)

This puts a macro in place where we'd been implementing `ClientRequest`,
which was a little boilerplate heavy. I'm actually not too terribly sure
about this one - it seems like the client request idea is stable enough
to toss the boilerplate, but is it? If so, our idea is to let a caller
assemble them as if they're closures in a match statement. It feels like
a positive move to me, and it tosses around ~420 lines, so that's nice,
but every time I do something like this I sit and stare at it much too
long, wondering.

Release-Type: refactor

- [patch] Bookmark submission and project peers (#290)

This commit evolves our concept of peers a considerable amount, making
way for an additional distribution model: project sharing and bookmark
submission.

Project sharing
---

Currently, oneiros supports bookmark sharing. Sharing a bookmark gives
you a uri you can give to others. With this uri, others can use the
`bookmark follow` command to establish a peering bridge to your oneiros,
and afterwards they can collect events from your oneiros and add them to
their own bookmarks.

This model works - but it's a narrower use than we need for some
distribution cases. By design! It limits what foreign peers have access
to, letting them see just the bookmark scope you've made. We needed
something a little broader / more robust to allow for actual third party
repositories or mirror-style features.

Enter the project share. Project shares work, ergonomically, like
bookmark shares. You call the share command, give someone a link, they
follow it. However, unlike bookmark shares, they grant both push and
pull functionality, and they do it project-wide.

The purpose of this is simple: to let users set up oneiros repositories,
and push/pull to them, the way we do git repositories.

Bookmark submit
---

The first major usage of this is the ability to read and submit
bookmarks to a project peer. This takes a local bookmark and pushes it
to the project peer of your choice (or attempts to, to be more precise).
This was the actual major feature we were angling towards: a push-based
sharing mechanism. We just learned we needed project shares to get
there, I suppose.

New and changed commands
---

- `project share` creates a project share url for you
- `project follow <uri> --name <name>` follow a foreign oneiros,
  establishing a project remote
- `bookmark submit <peer-name> <bookmark-name>` submits the bookmark to
  the project peer
- `bookmark collect` gains a `--from <peer-name>` flag you can use to
  grab a bookmark from a project peer

Release-Type: feature

- [patch] Dashboard build & client. (#288)

This commit introduces the `@oneiros/client` package, and the dashboard
build process.

The package
---

This is a generated client & type enforced hint package that mostly
generates from the OpenAPI schema. That means we had to implement the
generation of the schema itself, and update the parts of it that needed
more info. This means some new types showed up in the rust side, and we
wound up exploring a lot about the way the OpenAPI spec actually shapes
up through aide. It isn't great, but it isn't awful?

Anyway, the client itself is handy: it provides typed routing and typed
client access, and will yell at us if we get any API drift wrong. It
lets any TS-side consumer know at a glance if they've got what they
need, and helps the TS LSP autocomplete, etc. It's nice!

The build process
---

The generated dashboard now fully replaces the legacy dashboard. We did
this by popping rust embed into it. It should auto-build and pop into
the binary in the existing CI processes. Fingers crossed? IDK, seems
like trust falling is the only way to really cook using Github Actions.

Release-Type: feature

- [patch] Slicing (#286)

This commit introduces a new feature set, one of the last remaining
primitives before we roll out push-based CCS stuff: slicing.

A "slice" is a view of continuity, powered by a lens. Essentially, the
workflow boils down to this:

1. Build a lens that describes an important part of continuity to you
2. Look at the lens from time to time
3. Decide you want to just have the lens available to you on demand
4. Make a slice for the lens

Now you have a permanent lens that lets you peek at the continuity from
that direction at any point. Slices are handy! They cross bookmark
boundaries, you can have lots of them, and they're important for the
push-based workflows we want to support with the CCS updates.

Under the hood, slices work similarly to bookmarks: they track a
chronicle of the events they're concerned about, and they diff on every
event update to see if there are new events they need to accumulate.
However, unlike bookmarks, they don't need to be "active" in a request
in order to accumulate those events.

Finally, bookmarks have been taught a bit about slices. You can now seed
a bookmark not from your current focus, but from a slice, or a lens. See
the `--from-slice` and `--from-lens` flags on `bookmark create`.

- [patch] Use MCP init compositionally in setup. (#282)

We weren't using MCP init as a directly composed service in our setup
command, which meant that there were some contexts in which odd behavior
would crop up. We'd stall some, but not all, tests, for example, with a
"Do you want to install MCP json config?" style prompts. Not great!

This commit updates this so that we leverage the existing plumbing with
some defaults instead of duplicating the logic.

Release-Type: fix

- [patch] Use dream prompt in mcp. (#281)

We forgot to update our MCP setup to use the view layer components, and
that felt like an issue we should resolve sooner rather than later.
That's what this commit does.

Release-Type: fix

- [patch] Protocol workflow guarantees. (#280)

This sets up the idea of the protocol as a workflow-level contract,
proven by tests. Throughout the program so far we use our super enums
and payloads interchangeably, and just use a discipline to make sure
they're named in such a way that they don't collide. But, we have no
proof of the negative case: we can be sure that when a part of the
system is _prefiltering_ with a sub-enum, it will get the right thing.
But what about when they're all running through a single byte envelope
IO stream? That might not hold up. So we introduce a way to provide at
least a logical guarantee of that, if not a type level one.

Release-Type: fix

- [patch] Deprecation of project log, versionable APIs. (#279)

This is a custodial pass that adds some doc notation to versioned
methods, deprecates project log, and makes sure we don't hit common
namespace collisions.

Release-Type: fix

- [patch] Add project to requests. (#277)

Whoops! We missed this one. We want project requests in the requests
super enum so the protocol can actually route to this part of our
request space.

Release-Type: fix

- [patch] Trails (#274)

Trails are a new kind of relational projection that lets us get
fundamental attribution of events to the entities they create. This is
kind of a helper system that we'll need to power the lens system we
spiked out in #273, and in fact one of the reasons why we halted work
(not the only reason, though) is that the need for this component became
clear.

Trails give you the ability to go from ref to event source, and vice
versa, using the new `oneiros trail` commands. Internally, this is
powered by a lineage table that just joins events to refs in the
bookmarked db.

On continuity events
===

Notably, we don't yet have support for continuity events. That's
probably okay _temporarily_ because we don't have lingo for continuity
event traversal in lenses, yet. But, we'll need to create a migration of
some kind for them so they have ids instead of having names. The
alternative is that distributed name collisions can point to the wrong
record. That is, if two systems have an "alice" agent, and they merge,
which do we want to point at? Right now, the host bookmark would win -
and that might be desired, but "we're accidentally doing it right" feels
brittle. We should make stuff like that explicit, but there's some work
left to get there.

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

- [minor] Lens sets and readers (#285)

This takes the lens representation a level further, and completes our
local implementation of it all.

- Lenses have readers for most spaces now.
- You can create lens aliases in config for known lenses.
- MCP uses lenses like our endpoints do.

But, most crucially, the actual syntax of lenses is a little uniform as
well. Previously, this expression would be regarded as a concrete
fn-style invocation:

```lens
agent(governor.process)
```

In other words, `agent` here takes only `name` args. That's no longer
the case. The new structure is `agent(<lens>)`, and we've taught the
system how to expand lenses to lists of names. So, `governor.process` is
now thought of as a lens, which resolves terminally to a name. Meaning
we can do this:

```lens
agent(governor.process | thinker.process)
```

Which yields agents that match the set of interior args. Overall this
means that lenses should expand across their interior args, and work a
little closer to set types in general. It's my hope that this gives us a
strong expressive ability to select sweeping collections of things, and
sets us up in an ideal space for slicing.

Release-Type: feature

- [minor] Lens intermediate representation. (#278)

This carves out a structural topology for the lens intermediate
representation, which is important for letting us create lenses from
syntactical inputs, and turn those into predictable result sets. Lenses
are a querying mechanism that lets a user reach into a project and pull
a subset of its entities and events, for whatever purpose. Our idea here
is that we need something like this for slices and continuity control,
but in a broader sense, lenses answer an unmet part of the codebase
we've been fumbling around for a while: basically, async reads of our
persisted layer shouldn't be tightly coupled to one method of
persistence or backing impl, and right now they are. Lenses stand to
alleviate that and unify many of our read-style approaches.

We've also got an example "reader", the search index reader, which
collects lens hits from search results.

Release-Type: feature

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
