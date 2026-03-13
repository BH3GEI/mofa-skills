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
mofa infographic --skill mofa-observatory-report --style cyber-dash --out report.png --input gateway_metrics.json

# Generate a minimal text summary
mofa text --skill mofa-observatory-report --style minimal-report --input gateway_metrics.json
```

## Examples

The skill expects a raw JSON payload from the Gateway/Observatory metrics endpoint. You can find a sample payload in `examples/mock_gateway_telemetry.json`:

```json
{
  "gateway_status": "ONLINE",
  "total_active_agents": 40,
  "pending_tasks_queue": 150,
  ...
}
```

This telemetry JSON is injected into the style template's `{data}` placeholder, which the Gemini engine then uses to synthesize either the text report or the dashboard infographic content.

## Features

- **System Bridging**: The very first skill bridging internal MoFA coordination (Gateway) with visual output.
- **Visual Telemetry**: Turns boring JSON health checks into stunning, easy-to-read infographics.
- **Multi-modal Support**: Fallback to clean text summaries when visuals are not required.
