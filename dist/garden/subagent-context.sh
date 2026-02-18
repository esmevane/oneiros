#!/usr/bin/env bash
# SubagentStart hook helper: injects cognitive awareness into subagents
#
# Reads hook event JSON from stdin, extracts agent_type,
# maps it to an oneiros agent name, and outputs additionalContext JSON
# so the subagent knows it has access to the cognitive system.

set -euo pipefail

INPUT=$(cat)
AGENT_TYPE=$(echo "$INPUT" | jq -r '.agent_type // empty')

# Map Claude Code agent names (kebab-case) to oneiros agent names (dot notation)
case "$AGENT_TYPE" in
  oneiroi-process)   AGENT="oneiroi.process" ;;
  cognition-scribe)  AGENT="cognition.scribe" ;;
  memory-scribe)     AGENT="memory.scribe" ;;
  experience-scribe) AGENT="experience.scribe" ;;
  storage-scribe)    AGENT="storage.scribe" ;;
  *)
    # Unknown agent type — no context to inject
    exit 0
    ;;
esac

CONTEXT="You have access to the oneiros cognitive system. Run \`oneiros sense ${AGENT}\` to orient yourself. Record thoughts with \`oneiros cognition add ${AGENT} <texture> \"<content>\"\`. Run \`oneiros guidebook ${AGENT}\` for the full cognitive toolkit."

jq -n --arg ctx "$CONTEXT" '{
  hookSpecificOutput: {
    hookEventName: "SubagentStart",
    additionalContext: $ctx
  }
}'
