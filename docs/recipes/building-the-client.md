# Building an oneiros typescript client

We generate an automatic representation of our client with every release, a tool that can be used to make web or node consumers for any arbitrary oneiroi host.

## Why?

Right now? Mostly because the dashboard needs it. But, broadly, this works as a foundation for letting other folks make oneiros tools. That doesn't actually happen right now but, well, we know how to make it a nice experience and so not having that around is motivation enough. It feels good to have it here and although it looks big, it's mostly glue code - and took about a day.

## How?

The `oneiros` engine can be mounted against an arbitrary config, and run with a `oneshot` approach, where we essentially treat the whole thing as a function that takes a request and returns a response. It makes things really convenient for work like this. So, in our `xtask` crate, we do just that: 

1. We mount the engine with a "you're a schema generator" abbreviated config that knows nothing about actual apps, but enough to know its routing.
2. We create a request for our OpenAPI schema route. The server generates this and oneshots back a response.
3. We use some OpenAPI client generation code to give ourselves client-side primitives.
4. We wrap all that in additional client generation stuff that gives us typed pathways, like `api.get.health()`, so type hints can help us navigate things.

## What?

A lot of this is tooling that we're just stitching together!

- `aide` and `axum` give us the OpenAPI spec and the oneshot capability.
- `@hey-api/openapi-ts` does the generation work with the schema.
- We already followed a discipline of strongly typing requests and responses from our API endpoints.
- `rust-embed` lets us pop all this into the binary instead of relying on a bunch of local files to install/manage.

And the rest of it is typescript mysticism and boilerplate management on the files that stitch the client together. Afterwards, it's ready to go in the client consumers (just the dashboard, for now).

## Where?

- The process itself occurs in the `xtask` build support crate.
- Schema generation occurs in `crates/oneiros-engine`, in the `Engine` package.
- The OpenAPI annoatation is in the `http.rs` files scattered throughout the engine.
- The OpenAPI root concerns are  in the `docs.rs` file at the engine root.
- The client operations are in `packages/oneiros-client`.
- The consumer is in `apps/dashboard`.
- The entire thing goes into dashboard's `dist`, which is then embedded via `rust-embed`.

## When?

This should happen as part of the build step, and so it should occur on every single release automatically.

You can do it manually as well, with `cargo xtask dashboard-build`.
