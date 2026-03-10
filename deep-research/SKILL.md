---
name: deep-research
description: Iterative exploration-based deep research with human-like browsing behavior. Explores links depth-first, extracts knowledge incrementally, synthesizes with citations.
requires_bins:
requires_env:
always: false
---

# Deep Research

Human-like iterative research skill. Explores the web like a person: discover links, evaluate relevance, dive deep, follow trails, synthesize findings.

## Trigger Phrases

- "深度研究"
- "deep research on"
- "调研一下"
- "帮我查查"
- "research report"
- "全面分析"

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
│  │ 发现线索     │◄───┤ 新链接/新子问题   │    │
│  │ (web_search)│    │ (来自页面内容)    │    │
│  └──────┬──────┘    └─────────────────┘    │
│         │                                   │
│         ↓                                   │
│  ┌─────────────┐    ┌─────────────┐        │
│  │ 评估价值     │──否─┤ 跳过/低优先级  │        │
│  │ (LLM判断)   │    └─────────────┘        │
│  └──────┬──────┘                           │
│         │ 是                                │
│         ↓                                   │
│  ┌─────────────┐    ┌─────────────────┐    │
│  │ 深入探索     │───►│ web_fetch/browser│    │
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

## Workflow

### Phase 1: Entry (意图分析)

```json
{
  "role": "user",
  "content": "分析查询意图，确定研究策略。\n\n查询: 'AI regulation 2026'\n\n输出:\n1. 核心主题\n2. 关键子问题 (3-5个)\n3. 探索深度建议 (shallow/medium/deep)\n4. 起始搜索查询 (3个不同角度)"
}
```

### Phase 2: Discovery (线索发现)

```bash
# 执行多个角度搜索
web_search "AI regulation policy timeline 2026"
web_search "EU AI Act implementation 2026"
web_search "US federal AI legislation 2026"
```

### Phase 3: Evaluate (价值评估)

对每个搜索结果，LLM 评估：
- **Relevance** (0-1): 与查询相关度
- **Authority** (0-1): 来源权威性 (.gov > .edu > 知名媒体 > 博客)
- **Novelty** (0-1): 是否提供新信息
- **Depth Potential** (0-1): 是否值得深入挖掘

```json
{
  "role": "user",
  "content": "评估以下链接的研究价值:\n\n标题: EU AI Act: What Businesses Need to Know\nURL: https://example.com/eu-ai-act\n摘要: The EU AI Act will be fully effective August 2026...\n\n输出 JSON: {\"relevance\": 0.95, \"authority\": 0.9, \"novelty\": 0.8, \"depth_potential\": 0.85, \"should_explore\": true}"
}
```

### Phase 4: Explore (深入探索)

对高分链接，使用 browser 工具模拟点击：

```bash
# 获取页面内容
web_fetch "https://example.com/eu-ai-act"

# 或使用 browser 工具交互式浏览
browser_navigate "https://example.com/eu-ai-act"
browser_click "a[href*='timeline']"  # 点击时间线链接
browser_read  # 读取当前页面内容
```

### Phase 5: Extract (知识提取)

从页面提取结构化信息：

```json
{
  "role": "user",
  "content": "从以下页面内容提取知识:\n\n[页面内容...]\n\n输出 JSON:\n{\n  \"facts\": [{\"claim\": \"...\", \"confidence\": \"high\", \"quote\": \"...\"}],\n  \"sources\": [{\"title\": \"...\", \"url\": \"...\", \"date\": \"2026-01-15\"}],\n  \"contradictions\": [{\"claim_a\": \"...\", \"claim_b\": \"...\"}],\n  \"follow_up_links\": [\"url1\", \"url2\"],\n  \"sub_questions\": [\"...\", \"...\"]\n}"
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
    {"action": "search", "query": "...", "results_count": 10},
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

## Tools Used

- `web_search`: 发现初始线索
- `web_fetch`: 获取页面内容
- `browser_navigate/click/read`: 交互式浏览
- `read_file/write_file`: KnowledgeBase 持久化
- `message`: HITL 确认矛盾或不确定时

## Best Practices

1. **优先权威源**: .gov, .edu, 知名媒体优先探索
2. **追踪引用链**: 页面里的链接往往更有价值
3. **记录探索路径**: 方便回溯和验证
4. **标记不确定性**: 低 confidence 的 fact 要明确标注
5. **及时收敛**: 边际收益低时停止，避免无限探索
