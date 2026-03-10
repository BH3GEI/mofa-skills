# Example: Technology Research

**Query**: "Rust async runtime performance comparison 2026"

---

## Entry Phase

### Analysis
- **Intent**: Comparative analysis
- **Core Topics**: Rust async runtimes, performance benchmarks
- **Depth**: deep (technical topic, needs data)

### Search Queries
1. "Tokio vs async-std performance benchmark 2026"
2. "Rust async runtime benchmark comparison latest"
3. "embassy async runtime embedded performance"

---

## Exploration Log

### Iteration 1
```
web_search "Tokio vs async-std performance benchmark 2026"
  ↓
Results: 10 links
  ↓
Evaluate top 3:
  - tokio.rs/blog/2026-01-async-runtime (score: 0.95) → EXPLORE
  - github.com/tokio-rs/tokio/benches (score: 0.9) → EXPLORE
  - medium.com/random-dev/async-perf (score: 0.5) → SKIP (low authority)
  ↓
web_fetch "tokio.rs/blog/2026-01-async-runtime"
  ↓
Extract 8 facts, 3 follow-up links
  ↓
Follow: tokio.rs/blog/2026-01-async-runtime#methodology
```

### Iteration 2
```
web_fetch "github.com/tokio-rs/tokio/benches"
  ↓
Extract benchmark data
  ↓
Follow: Link to comparison with async-std
  ↓
web_fetch "async.rs/performance-comparison"
  ↓
Extract contradictory claim about memory usage
  ↓
FLAG CONTRADICTION: Tokio claims lower memory vs async-std claims lower memory
```

### Iteration 3 (HITL)
```
Found contradiction:
- Tokio blog: "30% lower memory than async-std"
- async-std docs: "25% lower memory than Tokio"

Ask user: "Found conflicting memory usage claims. Both sources tested different scenarios. Should I:
1. Look for third-party independent benchmarks
2. Note both claims with context
3. Focus on latency benchmarks instead?"

User: "Option 1 - find independent benchmarks"
```

### Iteration 4
```
web_search "independent rust async runtime benchmark 2026"
  ↓
Find: benchmarksgame-team.pages.debian.net
  ↓
web_fetch "benchmarksgame rust async"
  ↓
Extract: Independent measurements, methodology disclosed
  ↓
RESOLVE CONTRADICTION: Independent data shows Tokio faster on throughput, async-std lower memory
```

---

## Final Report Structure

```markdown
# Research Report: Rust Async Runtime Performance 2026

## Executive Summary
Independent benchmarks show Tokio excels in throughput scenarios while async-std demonstrates better memory efficiency. The choice depends on workload characteristics.

## Key Findings

### Finding 1: Throughput Performance
Tokio demonstrates superior throughput in high-concurrency scenarios [1][4]...

### Finding 2: Memory Usage
async-std shows 15-20% lower baseline memory consumption [2][4]...

## Contradictions Resolved
| Source A | Source B | Resolution |
|----------|----------|------------|
| Tokio claims 30% lower memory | async-std claims 25% lower | Independent tests show async-std lower memory, Tokio higher throughput - both tested different metrics |

## Sources
[1] Tokio Blog: Async Runtime Performance 2026 - tokio.rs/blog/... (Authority: high)
[2] async-std Documentation - async.rs/docs/... (Authority: high)
[4] Benchmarks Game: Rust Async Runtimes - benchmarksgame... (Authority: high, independent)
```
