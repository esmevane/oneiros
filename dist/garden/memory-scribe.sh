#!/usr/bin/env bash
# Garden job: memory.scribe reviews consolidation health
#
# Run periodically to survey agent memories for consolidation gaps,
# level imbalance, and stale knowledge.
#
# Requires: oneiros service running, project brain initialized

set -euo pipefail

claude -p "Review the memory landscape for all agents in this brain. For each agent, compare their cognition stream (oneiros cognition list --agent <name>) with their memories (oneiros memory list --agent <name>). Note consolidation gaps (many cognitions, few memories), level imbalance (everything at one level), and stale knowledge (project memories that no longer reflect current work). Record your observations as cognitions under your own agent name." \
  --append-system-prompt "$(oneiros dream memory.scribe)" \
  --allowedTools "Bash(oneiros *),Read"
