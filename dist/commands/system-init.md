---
description: Set up the oneiros host for the first time
argument-hint: "[--name <name>] [--yes]"
---

Run `oneiros system init` to set up the local oneiros host. This creates the system database, default tenant, and host keypair. Use `--name` to specify a host name, or `--yes` to accept defaults without prompting.

This is a one-time setup step. Running it again is safe — initialization is idempotent.
