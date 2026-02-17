---
description: Create or update a texture (cognitive category)
argument-hint: "<name> [--description <text>] [--prompt <text>]"
---

Run `oneiros texture set $ARGUMENTS` to create or update a texture. The name is required. Use `--description` for a brief category description and `--prompt` for instructions on when to use this texture.

Textures are idempotent — setting the same name again updates the existing texture. Use seed data for standard textures: `oneiros seed core`.
