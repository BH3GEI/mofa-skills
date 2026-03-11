---
name: deep-research
description: Iterative exploration-based deep research with Tavily search and fallback to web scraping. Human-like browsing with depth-first link following.
requires_bins: curl
requires_env: TAVILY_API_KEY
always: false
---

# Deep Research

Human-like iterative research skill. Explores the web depth-first: search → evaluate links → dive deep → follow trails → synthesize findings.

## Trigger Phrases

- "深度研究"
- "deep research on"
- "调研一下"
- "帮我查查"
- "research report"
- "全面分析"
- "查资料"

## Architecture

```
User Query
    ↓
Entry Agent (分析意图，确定探索策略)
    ↓
┌─────────────────────────────────────────────┐
│           EXPLORATION LOOP                  │
│                                             │
│  ┌─────────────┐    ┌─────────────────┐    │
│  │ 搜索发现     │◄───┤ 新链接/新子问题   │    │
│  │             │    │ (来自页面内容)    │    │
│  │ 1. Tavily   │    └─────────────────┘    │
│  │ 2. Google   │                           │
│  │    (fetch)  │                           │
│  └──────┬──────┘                           │
│         │                                   │
│         ↓                                   │
│  ┌─────────────┐    ┌─────────────┐        │
│  │ 评估价值     │──否─┤ 跳过/低优先级  │        │
│  │ (LLM判断)   │    └─────────────┘        │
│  └──────┬──────┘                           │
│         │ 是                                │
│         ↓                                   │
│  ┌─────────────┐    ┌─────────────────┐    │
│  │ 深入探索     │───►│ web_fetch        │    │
│  │             │    │ 提取内容         │    │
│  └──────┬──────┘    └─────────────────┘    │
│         │                                   │
│         ↓                                   │
│  ┌─────────────┐                           │
│  │ 提取知识     │───► 存入 KnowledgeBase   │
│  │ (facts/     │     (facts, sources,      │
│  │  quotes)    │      contradictions)      │
│  └─────────────┘                           │
│         │                                   │
│         └───────────────────────────────────┘
│              ↑ 发现新线索，继续循环
└─────────────────────────────────────────────┘
         ↓ 满足结束条件
    Synthesize Agent
         ↓
    Final Report (带完整引用链)
```

## Search Methods

### Method 1: Tavily Search (推荐)

需要环境变量 `TAVILY_API_KEY`

```bash
# 使用 Tavily API 搜索
curl -s "https://api.tavily.com/search" \
  -H "Content-Type: application/json" \
  -d '{
    "api_key": "'"$TAVILY_API_KEY"'",
    "query": "AI regulation 2026",
    "search_depth": "basic",
    "max_results": 10,
    "include_answer": false
  }'
```

### Method 2: Google via web_fetch (Fallback)

当 Tavily 不可用时，抓取 Google 搜索结果页：

```bash
# 获取 Google 搜索结果 HTML
# 注意：需要处理 anti-bot，可能需要重试
web_fetch "https://www.google.com/search?q=AI+regulation+2026&num=10"
```

然后提取链接：
```json
{
  "role": "user",
  "content": "从以下 Google 搜索结果 HTML 中提取前 10 个链接的标题和 URL:\n\n[HTML content...]\n\n输出 JSON: [{\"title\": \"...\", \"url\": \"...\"}]"
}
```

### Method 3: Direct URL (已知权威源)

直接访问已知高质量来源：

```bash
# 学术论文
web_fetch "https://arxiv.org/list/cs.AI/recent"

# 技术博客
web_fetch "https://blog.google/technology/ai/"

# Hacker News
web_fetch "https://news.ycombinator.com/"
```

## Workflow

### Phase 1: Entry (意图分析)

```json
{
  "role": "user",
  "content": "分析查询意图，确定研究策略。\n\n查询: 'AI regulation 2026'\n\n输出:\n1. 核心主题\n2. 关键子问题 (3-5个)\n3. 探索深度建议 (shallow/medium/deep)\n4. 起始搜索查询 (3个不同角度)"
}
```

### Phase 2: Discovery (搜索发现)

**首选 Tavily：**

```bash
curl -s "https://api.tavily.com/search" \
  -H "Content-Type: application/json" \
  -d '{
    "api_key": "'"$TAVILY_API_KEY"'",
    "query": "AI regulation policy timeline 2026",
    "search_depth": "basic",
    "max_results": 10
  }' | jq '.results[] | {title, url, content}'
```

**Tavily 失败时，fallback 到 Google：**

```bash
web_fetch "https://www.google.com/search?q=AI+regulation+2026&num=10"
# 然后解析 HTML 提取链接
```

### Phase 3: Evaluate (价值评估)

对每个候选链接，LLM 评估：

```json
{
  "role": "user",
  "content": "评估以下链接的研究价值:\n\n标题: EU AI Act: What Businesses Need to Know\nURL: https://example.com/eu-ai-act\n摘要: The EU AI Act will be fully effective August 2026...\n\n输出 JSON: {\"relevance\": 0.95, \"authority\": 0.9, \"novelty\": 0.8, \"should_explore\": true, \"reason\": \"...\"}'"
}
```

### Phase 4: Explore (深入探索)

对高分链接，获取完整内容：

```bash
# 获取页面内容
web_fetch "https://example.com/eu-ai-act"

# 如果页面需要交互，使用 browser 工具
browser_navigate "https://example.com/eu-ai-act"
browser_read
```

### Phase 5: Extract (知识提取)

从页面提取结构化信息：

```json
{
  "role": "user",
  "content": "从以下页面内容提取知识:\n\n[页面内容...]\n\n输出 JSON:\n{\n  \"facts\": [{\"claim\": \"...\", \"confidence\": \"high\", \"quote\": \"...\"}],\n  \"sources\": [{\"title\": \"...\", \"url\": \"...\", \"date\": \"2026-01-15\"}],\n  \"contradictions\": [...],\n  \"follow_up_links\": [\"url1\", \"url2\"],\n  \"sub_questions\": [\"...\", \"...\"]\n}"
}
```

### Phase 6: Loop (循环迭代)

检查是否需要继续：

```
结束条件 (满足任一即停止):
- 已探索 N 个深度链接 (config.max_depth_links)
- 已收集 M 个 facts (config.min_facts)
- 连续 3 次未发现新线索
- 知识覆盖度 > 80% (自评估)
- 用户明确要求停止
```

### Phase 7: Synthesize (综合报告)

```json
{
  "role": "user",
  "content": "基于收集的知识生成研究报告:\n\n[KnowledgeBase...]\n\n报告结构:\n1. Executive Summary (3-5句)\n2. Key Findings (带引用 [1], [2])\n3. Detailed Analysis (按主题组织)\n4. Contradictions & Uncertainties\n5. Sources (完整引用列表)"
}
```

## KnowledgeBase 结构

```json
{
  "query": "AI regulation 2026",
  "facts": [
    {
      "id": "f1",
      "claim": "EU AI Act fully effective August 2026",
      "confidence": "high",
      "sources": ["s1", "s2"],
      "quote": "The Act will apply from 2 August 2026..."
    }
  ],
  "sources": [
    {
      "id": "s1",
      "title": "EU AI Act Official Text",
      "url": "https://...",
      "domain": "europa.eu",
      "authority": "high",
      "date": "2024-08-01",
      "accessed": "2026-03-11"
    }
  ],
  "contradictions": [
    {
      "id": "c1",
      "claim_a": "US federal AI law passes Q2 2026",
      "source_a": "s3",
      "claim_b": "US law delayed to Q4 2026",
      "source_b": "s4",
      "resolution": "pending_verification"
    }
  ],
  "exploration_log": [
    {"action": "tavily_search", "query": "...", "results_count": 10},
    {"action": "evaluate", "url": "...", "score": 0.9},
    {"action": "explore", "url": "...", "facts_extracted": 5},
    {"action": "follow_link", "from": "...", "to": "..."}
  ]
}
```

## Configuration

见 `config.toml`，可调整：
- `max_depth_links`: 最大深入链接数
- `max_iterations`: 探索循环次数上限
- `score_threshold`: 评估通过分数
- `min_facts`: 最少收集 facts 数
- `search_method`: `tavily` | `google` | `auto`

## Tools Used

- **Tavily API**: 高质量搜索结果 (需要 `TAVILY_API_KEY`)
- `web_fetch`: 获取页面内容，或作为搜索 fallback
- `browser_navigate/click/read`: 交互式浏览
- `read_file/write_file`: KnowledgeBase 持久化
- `message`: HITL 确认矛盾或不确定时

## Best Practices

1. **优先 Tavily**: 结构化搜索结果，包含 title/url/content
2. **Google Fallback**: Tavily 失败时自动降级
3. **优先权威源**: .gov, .edu, 知名媒体优先探索
4. **追踪引用链**: 页面里的链接往往更有价值
5. **记录探索路径**: 方便回溯和验证
6. **标记不确定性**: 低 confidence 的 fact 要明确标注
7. **及时收敛**: 边际收益低时停止，避免无限探索

## Error Handling

| Scenario | Fallback |
|----------|----------|
| Tavily API 失败 | 自动降级到 Google web_fetch |
| Google 被 block | 使用直接 URL 列表 |
| 页面 fetch 失败 | 尝试 browser 工具 |
| 所有搜索失败 | 请求用户提供起始链接 |

## File Structure

```
./research/
├── {query-slug}/
│   ├── kb/
│   │   ├── facts.json
│   │   ├── sources.json
│   │   └── contradictions.json
│   ├── exploration_log.json
│   └── final-report.md
└── cache/
    └── tavily-{hash}.json
```
