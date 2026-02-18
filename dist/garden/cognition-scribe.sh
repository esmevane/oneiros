#!/usr/bin/env bash
# Garden job: cognition.scribe reviews the thought streams
#
# Run periodically to survey agent cognitive streams for silence,
# texture imbalance, and unrecorded thinking.
#
# Requires: oneiros service running, project brain initialized

set -euo pipefail

claude -p "Review the cognitive streams of all agents in this brain. For each agent, run oneiros cognition list --agent <name> and examine the stream. Note any silence (no recent cognitions during active work), texture imbalance (all observations, no reflections), or thin thinking (conclusions without process). Record your observations as cognitions under your own agent name." \
  --append-system-prompt "$(oneiros dream cognition.scribe)" \
  --allowedTools "Bash(oneiros *),Read"
