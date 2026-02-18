#!/usr/bin/env bash
# Garden job: oneiroi.process reviews overall cognitive health
#
# Run periodically for a holistic review of the brain's cognitive state.
# This is the meta-job — it surveys the whole garden, not one domain.
#
# Requires: oneiros service running, project brain initialized

set -euo pipefail

claude -p "Survey the cognitive health of this brain. Check each agent's cognitive stream, memory levels, and experience graph. Look for: stream silence during active work, consolidation gaps, stale experience threads, texture and sensation imbalance, and the overall shape of cognitive practice. Record your findings as observations and reflections. If you notice patterns that span multiple agents, create experiences to trace them." \
  --append-system-prompt "$(oneiros dream oneiroi.process)" \
  --allowedTools "Bash(oneiros *),Read"
