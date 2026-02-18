#!/usr/bin/env bash
# Garden job: storage.scribe reviews the brain's archive
#
# Run periodically to survey storage for orphaned entries,
# missing references, and unarchived artifacts.
#
# Requires: oneiros service running, project brain initialized

set -euo pipefail

claude -p "Review the storage landscape for this brain. List all storage entries (oneiros storage list) and experiences (oneiros experience list). Note orphaned storage (entries not linked to any experience), missing references (experiences that discuss artifacts without storage links), and significant work products that exist only in the codebase but not in storage. Record your observations as cognitions under your own agent name." \
  --append-system-prompt "$(oneiros dream storage.scribe)" \
  --allowedTools "Bash(oneiros *),Read"
