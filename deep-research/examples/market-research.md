# Example: Market Research

**Query**: "Electric vehicle market share China vs Europe 2026"

---

## Entry Phase

### Analysis
- **Intent**: Comparative market analysis
- **Core Topics**: EV market share, China market, Europe market
- **Depth**: medium (needs current data, but broadly available)

### Search Queries
1. "China EV market share 2026 BYD Tesla"
2. "Europe electric vehicle sales statistics 2026"
3. "global EV market share comparison China Europe"

---

## Exploration Log

### Iteration 1: China Market
```
web_search "China EV market share 2026"
  ↓
Evaluate:
  - caam.org.cn (中国汽车工业协会) - score: 0.95 → EXPLORE
  - cleantechnica.com - score: 0.7 → EXPLORE
  ↓
web_fetch "caam.org.cn/2026-ev-sales"
  ↓
Extract: Q1-Q3 2026 data, BYD 35%, Tesla 12%, total sales 8.5M units
  ↓
Follow: Link to export data
  ↓
Extract: Export growth 45% YoY to Europe
```

### Iteration 2: Europe Market
```
web_search "Europe EV market share 2026"
  ↓
Evaluate:
  - acea.auto (European Automobile Manufacturers) - score: 0.95 → EXPLORE
  - transportenvironment.org - score: 0.8 → EXPLORE
  ↓
web_fetch "acea.auto/press-release/2026-ev-registrations"
  ↓
Extract: EU+EFTA 2.8M units, Tesla 18%, VW 15%, BYD 8%
  ↓
Contradiction check: CAAM says BYD exported 800k to Europe, ACEA shows BYD only 224k registered
  ↓
INVESTIGATE: Time lag? Inventory in transit? Different counting methods?
```

### Iteration 3: Resolve Data Gap
```
web_search "BYD Europe sales vs exports discrepancy 2026"
  ↓
Find: cnevpost.com analysis on export vs registration timing
  ↓
Extract: 3-6 month lag between export and registration
  ↓
RECONCILE: CAAM export data = shipped, ACEA = registered. 800k exported, 224k registered suggests 576k in transit/pipeline
```

### Iteration 4: Competitive Analysis
```
Follow link from ACEA report: Tesla EU pricing strategy
  ↓
web_fetch
  ↓
Extract: Tesla price cuts in response to BYD competition
  ↓
Follow: Link to VW Group response
  ↓
Extract: VW announces ID.2 pricing to compete with BYD
```

---

## Key Insight

Discovered nuance in export vs registration data that simple search would miss:
- Raw numbers suggest discrepancy
- Deep exploration reveals timing difference
- Market dynamics: BYD building inventory for Q4 push

---

## Final Report Highlights

```markdown
# EV Market Share: China vs Europe 2026

## Key Finding: The Export-Registration Gap

Contrary to apparent contradiction in headline numbers:
- China reports 800k BYD exports to Europe (Jan-Sep)
- Europe reports 224k BYD registrations (Jan-Sep)

**Resolution**: 3-6 month registration lag. ~576k units in transit/pipeline.

## Market Position

| Market | Leader | Share | Key Insight |
|--------|--------|-------|-------------|
| China | BYD | 35% | Domestic dominance, price leadership |
| Europe | Tesla | 18% | Under pressure from BYD expansion |

## Trend: BYD's European Push

Evidence of aggressive market entry:
- Export volume growth 45% YoY
- Price positioning €20k below Tesla Model 3
- Local manufacturing rumors (follow-up investigation suggested)

## Sources
[1] CAAM: China Auto Sales 2026 - caam.org.cn (Authority: high, primary)
[2] ACEA: EU Vehicle Registrations - acea.auto (Authority: high, primary)
[3] CnEVPost: Export Analysis - cnevpost.com (Authority: medium, trade pub)
```
