# Recursive Exploration Template

Use this template when continuing to deeper layers of research.

## Current State

- Parent Query: {{PARENT_QUERY}}
- Current Layer: {{CURRENT_LAYER}} (1, 2, or 3)
- Parent Finding: {{PARENT_FINDING}}
- Recursion Type: {{RECURSION_TYPE}} (event/person/company/data/policy/technology/reaction)
- Entity Name: {{ENTITY_NAME}}

## Layer-Specific Objectives

### Layer 1 (Event/Discovery)
**Objective**: Establish basic facts

**Search Queries**:
- "{{ENTITY_NAME}} what happened date"
- "{{ENTITY_NAME}} details specifications"
- "{{ENTITY_NAME}} official announcement"

**Extract**:
- Exact dates, times, locations
- Key numbers, statistics
- Direct quotes from primary sources
- Names of key people/organizations involved

### Layer 2 (Background/Context)
**Objective**: Understand why and how

**Search Queries**:
- "{{ENTITY_NAME}} history background"
- "{{ENTITY_NAME}} previous developments"
- "{{ENTITY_NAME}} industry context"
- "{{RELATED_ENTITY}} relationship to {{ENTITY_NAME}}"

**Extract**:
- Historical timeline leading to this event
- Previous similar events
- Industry/sector context
- Key stakeholders and their positions
- Economic/political/technical background

### Layer 3 (Impact/Reaction/Future)
**Objective**: Understand consequences and what's next

**Search Queries**:
- "{{ENTITY_NAME}} impact consequences"
- "{{ENTITY_NAME}} reactions responses"
- "{{ENTITY_NAME}} future predictions"
- "{{STAKEHOLDER}} response to {{ENTITY_NAME}}"
- "{{ENTITY_NAME}} market industry effect"

**Extract**:
- Immediate reactions from key stakeholders
- Short-term impacts (1-3 months)
- Long-term implications (6-12 months)
- Market/industry response
- Follow-up events or announcements
- Expert predictions and analysis

## Cross-Layer Verification

At each layer, verify against previous layers:
- Does new information contradict Layer 1 facts?
- Does background explain the event accurately?
- Do reactions align with stakeholder positions identified?

If contradictions found:
1. Flag for verification
2. Search for additional sources
3. Note in knowledge base with uncertainty marking

## Recursion Exit Criteria

Stop recursion on this branch when:
- [ ] Layer 3 completed with substantial findings
- [ ] No new information in 2 consecutive searches
- [ ] Sources become repetitive (same information, no new angles)
- [ ] 5+ sources consulted for this branch

## Output Format

```json
{
  "layer": 1|2|3,
  "entity": "name",
  "type": "event|person|company|data|policy|technology|reaction",
  "findings": [
    {
      "fact": "specific finding",
      "layer_appropriate": true,
      "supports_parent": true,
      "confidence": "high|medium|low",
      "sources": ["s1", "s2"]
    }
  ],
  "new_recursion_candidates": [
    {
      "entity": "new entity found",
      "type": "...",
      "suggested_next_layer": 2|3,
      "priority": "high|medium|low"
    }
  ],
  "cross_layer_consistency": {
    "consistent_with_previous": true|false,
    "contradictions_found": [],
    "explanations": []
  },
  "synthesis_with_parent": "How this layer's findings connect to parent finding",
  "continue_recursion": true|false,
  "reason": "why continue or stop"
}
```

## Example: News Trail

### Parent Finding (Initial)
"US announces new AI chip export controls (March 6, 2026)"

### Layer 1 - What
- Specific companies: NVIDIA, AMD required licenses
- Countries affected: 40+ countries tiered system
- Effective date: Immediate for new exports

### Layer 2 - Background
- Previous controls: October 2023 restrictions
- Policy evolution: Biden administration strategy
- Context: US-China tech competition escalation
- Stakeholder positions: Industry vs National security

### Layer 3 - Impact
- NVIDIA stock: -1.9% single day
- China response: Accelerate domestic chip development
- Industry reaction: Concerns about overreach
- EU response: Monitoring for WTO compliance
- Future: Potential retaliation measures

### Cross-Reference
- Layer 1 date matches Layer 2 policy timeline ✓
- Layer 3 stock impact verified by multiple sources ✓
- Layer 3 China response from official sources ✓

## Sub-Agent Spawning

For parallel exploration of multiple recursion candidates:

```bash
# Spawn sub-agents for high-priority candidates
spawn "recursive-explorer-1" "Layer 2 exploration of Entity A"
spawn "recursive-explorer-2" "Layer 2 exploration of Entity B"
spawn "recursive-explorer-3" "Layer 3 impact analysis"

# Collect and integrate results
```

## Minimum Requirements per Layer

- Layer 1: 3-5 specific facts with sources
- Layer 2: 3-5 background/context items
- Layer 3: 3-5 impact/reaction items
- Each layer: At least 2 independent sources
