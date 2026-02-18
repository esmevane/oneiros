#!/usr/bin/env bash
# Garden job: experience.scribe reviews the experience graph
#
# Run periodically to survey the experience graph for unnamed threads,
# frozen experiences, and sparse connections.
#
# Requires: oneiros service running, project brain initialized

set -euo pipefail

claude -p "Review the experience graph for all agents in this brain. For each agent, examine their experiences (oneiros experience list --agent <name>) and cognitions (oneiros cognition list --agent <name>). Note unnamed threads (related cognitions without connecting experiences), frozen experiences (threads that stopped growing), and sparse graphs (many cognitions, few experiences). Where connections are obvious and mechanical, create them directly. Where they require interpretation, record an observation for the agent." \
  --append-system-prompt "$(oneiros dream experience.scribe)" \
  --allowedTools "Bash(oneiros *),Read"
