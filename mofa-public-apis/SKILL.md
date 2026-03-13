---
name: mofa-public-apis
description: "Search and discover free public APIs from local cache — browse by category, filter by auth/HTTPS/CORS, get smart recommendations. Triggers: 找API, search api, 推荐api, public api"
requires_bins: python3
always: false
---

# MOFA Public APIs

Local-first API discovery with intelligent filtering and recommendations.

## Onboarding / 开始使用

### 前置要求

1. **Python 3.8+**
   ```bash
   python3 --version
   ```

2. **数据源** (本地缓存)
   - 数据文件: `apis.json` (位于 skill 目录)
   - 包含 1000+ 公共 API 的元数据
   - 无需外部 API key

3. **验证安装**
   ```bash
   # 检查 Python
   python3 -c "import json; print('OK')"

   # 检查数据文件
   ls -la apis.json
   ```

### 快速开始

```bash
# 搜索 API
python3 -c "
import json
with open('apis.json') as f:
    apis = json.load(f)['entries']
    results = [a for a in apis if 'weather' in a['description'].lower()]
    for r in results[:5]:
        print(f\"{r['name']}: {r['description'][:80]}...\")
"
```

### 故障排除

| 问题 | 解决方案 |
|-----|---------|
| `apis.json not found` | 确保在 skill 目录运行，或指定完整路径 |
| `json parse error` | 重新下载 apis.json，可能文件损坏 |
| 搜索结果为空 | 尝试不同的关键词，或使用模糊搜索 |

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         MOFA PUBLIC APIs PIPELINE                           │
└─────────────────────────────────────────────────────────────────────────────┘

User Query
    ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 1: PARSE                                          │
│ ─────────────────                                       │
│ • Detect search type (keyword | category | filter)      │
│ • Extract constraints (auth, HTTPS, CORS)               │
│ • Parse intent: quick lookup vs discovery vs compare    │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 2: SEARCH                                         │
│ ─────────────                                           │
│                                                         │
│  Load apis.json (local cache)                           │
│       ↓                                                 │
│  ┌─────────────────────────────────────────────┐        │
│  │ Multi-field matching:                       │        │
│  │ • name: exact → fuzzy                       │        │
│  │ • description: semantic keywords            │        │
│  │ • category: exact match                     │        │
│  └─────────────────────────────────────────────┘        │
│       ↓                                                 │
│  Apply filters (AND logic)                              │
│       ↓                                                 │
│  Score & rank results                                   │
│       ↓                                                 │
│  Limit to top N (default: 20)                           │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 3: ENHANCE                                        │
│ ───────────────                                         │
│                                                         │
│  For each result:                                       │
│  ┌─────────────────────────────────────────────┐        │
│  │ Add badges:                                 │        │
│  │ [推荐] - if no-auth + HTTPS + CORS          │        │
│  │ [免认证] - if auth=none                     │        │
│  │ [前端友好] - if HTTPS + CORS=yes            │        │
│  └─────────────────────────────────────────────┘        │
│                                                         │
│  Calculate match score                                  │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 4: OUTPUT                                         │
│ ─────────────                                           │
│                                                         │
│  Format as table / compact / JSON                       │
│       ↓                                                 │
│  Include summary stats                                  │
│  (found X of Y, filtered by Z)                          │
└─────────────────────────────────────────────────────────┘
```

## Data Source

Local cache of [public-apis](https://github.com/public-apis/public-apis) with 1415 APIs across 50 categories.

| Attribute | Description | Values |
|-----------|-------------|--------|
| `name` | API name | String |
| `description` | What it does | String |
| `url` | Documentation link | URL |
| `category` | Category | See categories list |
| `auth` | Authentication | `none` / `apiKey` / `OAuth` |
| `https` | HTTPS support | `true` / `false` |
| `cors` | CORS support | `yes` / `no` / `unknown` |

## Search Types

### 1. Keyword Search
```
"weather api"
"crypto"
"machine learning"
```
Matches: name, description, category

### 2. Category Browse
```
category="Weather"
category="Cryptocurrency"
```
Exact category match from 50 available.

### 3. Filtered Discovery
```
auth="none" AND https=true
category="Music" AND cors="yes"
```
AND logic between filters.

## Scoring Algorithm

```
Score = BaseMatch + Boosts

BaseMatch:
  exact name match:     +1.0
  name contains:        +0.8
  description contains: +0.5
  category match:       +0.3

Boosts:
  HTTPS support:        +0.2
  CORS=yes:             +0.1
  no auth required:     +0.15
```

## Recommendation Badges

| Badge | Condition | Meaning |
|-------|-----------|---------|
| [推荐] | no-auth + HTTPS + CORS | Best for quick prototyping |
| [免认证] | auth=none | Use immediately, no signup |
| [前端友好] | HTTPS + CORS=yes | Browser/frontend ready |

## API Categories

**Popular (13):**
Development, Weather, Cryptocurrency, Finance, Machine Learning, Music, News, Social, Video, Geocoding, Books, Games & Comics, Health

**All 50:**
Animals, Anime, Anti-Malware, Art & Design, Authentication & Authorization, Books, Business, Calendar, Cloud Storage & File Sharing, Continuous Integration, Cryptocurrency, Currency Exchange, Data Validation, Development, Dictionaries, Documents & Productivity, Email, Entertainment, Environment, Events, Finance, Food & Drink, Games & Comics, Geocoding, Government, Health, Jobs, Machine Learning, Music, News, Open Data, Open Source Projects, Patent, Personality, Phone, Photography, Programming, Science & Math, Security, Shopping, Social, Sports & Fitness, Test Data, Text Analysis, Tracking, Transportation, URL Shorteners, Vehicle, Video, Weather

## Usage Examples

### Quick Search
```
User: "找个天气 API"

→ Search: keyword="weather"
→ Results: 10 APIs
→ Highlight: [推荐] OpenWeatherMap, [免认证] wttr.in
```

### Category Browse
```
User: "推荐一些加密货币 API"

→ Filter: category="Cryptocurrency"
→ Sort: no-auth first, then by popularity
→ Results: CoinGecko [推荐], CoinMarketCap, etc.
```

### Filtered Search
```
User: "需要免认证且支持 HTTPS 的音乐 API"

→ Filter: auth="none", https=true, category="Music"
→ Results: filtered list
```

### Discovery
```
User: "有什么好玩的 API"

→ Random/sample from: Animals, Games & Comics, Entertainment
→ Show diverse picks with descriptions
```

## Output Formats

### Table (default)
```
| Name | Description | Auth | HTTPS | CORS |
|------|-------------|------|-------|------|
| OpenWeatherMap | Weather data | apiKey | Yes | Yes |
| wttr.in | Weather in terminal | No | Yes | No |
```

### Compact
```
• OpenWeatherMap [推荐] - Weather data (apiKey, HTTPS, CORS)
• wttr.in [免认证] - Weather in terminal (no auth, HTTPS)
```

### JSON
Full structured output for programmatic use.

## Configuration

See `config.toml` for:
- `search.default_limit`: 20
- `search.https_boost`: 0.2
- `filter.auth_types`: ["none", "apiKey", "OAuth"]
- `categories.popular`: 13 categories

## Sync Data

To update the local cache from GitHub:

```bash
cd mofa-public-apis/scripts
python3 sync.py
```

This will:
1. Fetch latest README from public-apis repo
2. Parse all API entries
3. Update `apis.json` with fresh data
4. Report statistics

## Performance

- Local JSON load: ~50ms
- Search + filter: ~10ms for 1415 entries
- Total response: <100ms (no network calls)
