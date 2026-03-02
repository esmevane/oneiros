#!/bin/bash
# require-action-node.sh
# Soft prompt if no recent action/goal node exists in deciduous
# Exit code 0 = allow the edit, message is advisory only

# Check if deciduous is initialized
if [ ! -d ".deciduous" ]; then
    # No deciduous in this project, allow all edits
    exit 0
fi

# Check for any action or goal node created in the last 45 minutes
# We check both because starting new work creates a goal first
recent_node=$(deciduous nodes 2>/dev/null | grep -E '\[(goal|action)\]' | tail -5)

if [ -z "$recent_node" ]; then
    # No nodes at all - this is a fresh project, allow edits
    exit 0
fi

# Check if any node was created recently (within last 15 min)
# Parse the timestamps from nodes output
now=$(date +%s)
forty_five_min_ago=$((now - 2700))

# Get the most recent node's timestamp
# deciduous nodes format: ID [type] Title [confidence%] [timestamp]
latest_timestamp=$(deciduous nodes 2>/dev/null | tail -1 | grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2}' | tail -1)

if [ -n "$latest_timestamp" ]; then
    # Convert to epoch
    if [[ "$OSTYPE" == "darwin"* ]]; then
        node_epoch=$(date -j -f "%Y-%m-%d %H:%M:%S" "$latest_timestamp" +%s 2>/dev/null || echo "0")
    else
        node_epoch=$(date -d "$latest_timestamp" +%s 2>/dev/null || echo "0")
    fi

    if [ "$node_epoch" -gt "$forty_five_min_ago" ]; then
        # Recent node exists, allow the edit
        exit 0
    fi
fi

# No recent node - soft prompt, allow the edit
cat >&2 << 'EOF'
Deciduous: No action/goal node in the last 45 minutes.
Consider logging what you're working on:
  deciduous add goal "What you're trying to achieve" -c 90
  deciduous add action "What you're about to implement" -c 85
EOF

exit 0
