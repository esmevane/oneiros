---
description: Issue a project access ticket to an actor
argument-hint: "--actor-id <id> --project-name <name>"
---

Run `oneiros ticket issue $ARGUMENTS` to issue a ticket granting an actor access to a project. Tickets are the authentication tokens that authorize project-scoped operations. Use `--permission read` (default) for pull access or `--permission write` for push/submit access.
