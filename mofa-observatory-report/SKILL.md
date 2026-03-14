---
name: observatory-report
description: Monitor MoFA Gateway health and system metrics, outputting beautiful visual dashboards or text reports.
---

# MoFA Observatory Report

This skill connects the internal MoFA Gateway/Observatory infrastructure with the visual generation capabilities of `mofa-skills`. It fetches real-time system metrics (active agents, node health, task queues) and synthesizes them into visual infographics or text summaries.

## Trigger Phrases

Activate this skill when user says:
- "Check gateway status"
- "Are all agents online?"
- "Generate system health report"
- "Show me the observatory dashboard"
- "Gateway metrics"

## Usage (CLI Examples)

```bash
# Generate a visual Cyberpunk dashboard report
mofa infographic --style cyber-dash --out report.png --input gateway_metrics.json

# Generate a minimal text summary
mofa slides --style minimal-report --input gateway_metrics.json
```

## Examples

The skill expects a raw JSON payload from the Gateway/Observatory metrics endpoint. You can find a sample payload in `examples/mock_gateway_telemetry.json`:

```text
{
  "gateway_status": "ONLINE",
  "total_active_agents": 40,
  "pending_tasks_queue": 150
}
```

This telemetry JSON is passed directly to the Gemini engine as the input data (for example, via the section prompt), while the selected style template is used verbatim as a prefix to guide how the text report or dashboard infographic is synthesized.

## Features

- **System Bridging**: The very first skill bridging internal MoFA coordination (Gateway) with visual output.
- **Visual Telemetry**: Turns boring JSON health checks into stunning, easy-to-read infographics.
- **Multi-modal Support**: Fallback to clean text summaries when visuals are not required.
