---
name: mofa-agent-topology
description: Visualize multi-agent network maps, topological routes, and agent coordination graphs directly from Gateway configurations.
---

# MoFA Agent Topology Visualizer

This skill generates stunning visual network maps and coordination graphs representing the structure of a multi-agent MoFA deployment. By ingesting a Gateway `RouteRegistry` or coordination graph layout, it produces professional infographics illustrating how agents connect, communicate, and handoff tasks.

## Trigger Phrases

Activate this skill when user says:
- "Show me the agent network map"
- "Visualize gateway routing topology"
- "Generate coordination flow diagram"
- "Draw the agent architecture"
- "Agent topography"

## Usage (CLI Examples)

```bash
# Generate a futuristic node-based network map
mofa infographic --style network-map --out topology.png --input route_registry.json

# Generate a sequence/coordination flow visualization
mofa infographic --style coordination-flow --out handoff.png --input handoff_routes.json
```

## Examples

The skill expects a raw JSON payload describing the topological connections or gateway routing tables. You can find a sample payload in `examples/mock_topology_config.json`:

```text
{
  "gateway_node": "MoFA-Main-Gateway",
  "routing_table": [
    { "source": "User-Intent", "target": "Researcher-Agent", "protocol": "HTTP" },
    { "source": "Researcher-Agent", "target": "Review-Agent", "protocol": "gRPC" }
  ]
}
```

This telemetry JSON is passed directly to the Gemini engine as the input data (for example, via the section prompt), while the selected style template is used verbatim as a prefix to guide how the topology infographic is synthesized.

## Features

- **Architectural Clarity**: The first skill to visually represent the *structural* side of MoFA (Gateway Routing).
- **Presentation Ready**: Perfect for generating architecture slides or GitHub README graphics.
- **Dynamic**: Adapts its visual complexity based on the size of the swarm provided in the JSON input.
