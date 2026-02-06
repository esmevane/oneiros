# Oneiros Outcomes

Outcomes are a reporting wrapper that helps abstract feedback output into an Error-ish structure.

## Why a convoluted tracing wrapper?

Sometimes we want to trace, sometimes we want to talk to the user, and sometimes we just want to test that we're reporting the right thing. Sometimes it's a mix of all three. Making a trait and a collection wrapper for these things helps in all those cases.

There's some additional plumbing here to play nice with tracing, specifically so we can get tracing in the right spot for instrumentation while [working around some known shortcomings of the crate](https://github.com/tokio-rs/tracing/issues/2429).
