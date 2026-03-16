---
name: mofa-memory
description: "Persistent vector memory across agent runs. Store, search, and recall knowledge using semantic similarity. Triggers: remember, recall, memorize, store memory, search memory, vector memory, semantic search, prior research, 记忆, 存储, 检索, 语义搜索"
requires_env: OPENAI_API_KEY
version: 1.0.0
author: hagency
always: false
---

# MoFA Memory

Persistent vector memory skill for storing and semantically retrieving knowledge across agent runs. Uses OpenAI `text-embedding-3-small` for embeddings and stores everything in a local SQLite database at `~/.mofa/memory.db`.

Every other MoFA skill is stateless — results are computed and discarded. MoFA Memory changes this: research reports, crawled summaries, extracted facts, and any other content can be stored once and recalled in future sessions by semantic similarity, not just exact keyword match.

## When to Use

- **After research**: store the synthesized report so future queries can build on it instead of starting cold
- **Before research**: check if a similar query was researched before and surface prior findings to the user
- **Knowledge base building**: accumulate facts, summaries, or source excerpts across many agent runs
- **Cross-session continuity**: recall what was learned in previous conversations

## Tools

### store_memory

Embed content with OpenAI and save it to persistent SQLite memory. Returns the assigned memory ID.

**Input:**
```json
{
  "content": "OpenAI released GPT-5 in March 2026, with multimodal reasoning...",
  "tags": ["ai", "openai", "gpt5", "research-report"],
  "source": "https://openai.com/blog/gpt-5"
}
```

- `content` (required): text to embed and store
- `tags` (required): array of strings for categorizing and filtering
- `source` (optional): URL or file path identifying where this content came from

**Output:** `Stored memory <id> (<N> chars, tags: [...])`

---

### retrieve_memory

Semantic search over stored memories. Embeds the query, computes cosine similarity against all stored embeddings, and returns the top-k most relevant matches.

**Input:**
```json
{
  "query": "OpenAI model releases in 2026",
  "top_k": 5,
  "min_score": 0.7
}
```

- `query` (required): natural language search query
- `top_k` (optional, default 5): maximum number of results
- `min_score` (optional, default 0.7): minimum cosine similarity (0.0–1.0)

**Output:** JSON array of matches sorted by score descending:
```json
[
  {
    "id": "uuid",
    "score": 0.921,
    "content": "...",
    "tags": ["ai", "openai"],
    "source": "https://...",
    "created_at": "2026-03-15T10:00:00Z"
  }
]
```

---

### clear_memory

Delete memories by tag filter. If no tags provided, deletes ALL memories.

**Delete by tag:**
```json
{"tags": ["research", "draft"]}
```

**Delete everything:**
```json
{}
```

**Output:** `Deleted N memories with tags: [...]` or `Deleted all N memories.`

---

### list_memories

List stored memories with metadata and content preview. Optionally filter by tags.

**Input:**
```json
{"tags": ["research"]}
```

**Output:** Formatted table of id / created / tags / source / preview (80 chars).

---

## Configuration

| Environment Variable | Default | Description |
|---------------------|---------|-------------|
| `OPENAI_API_KEY` | (required) | OpenAI API key for `text-embedding-3-small` |
| `OPENAI_BASE_URL` | `https://api.openai.com` | Configurable base URL for OpenAI-compatible gateways |
| `MOFA_MEMORY_DB` | `~/.mofa/memory.db` | SQLite database path override |

`OPENAI_BASE_URL` follows the mofa-skills convention from PR #1 — you can route through r9s.ai, OpenRouter, or any OpenAI-compatible gateway for unified billing.

---

## Integration with mofa-research-2.0

The `mofa-research-2.0/architecture.dot` already shows a `ResearchMemory` node receiving from `KnowledgeBase` and `Synthesize`. This skill is what makes that node real.

**Before starting research (pre-check):**
```json
{"query": "<user's original query>", "top_k": 3, "min_score": 0.75}
```
Call `retrieve_memory`. If results score above 0.75, surface to the user:
> "I found prior research on this topic from {created_at}. Shall I build on it or start fresh?"

**After synthesis (persist results):**
```json
{
  "content": "<full text of ./research/{slug}/report.md>",
  "tags": ["{query-slug}", "{primary-topic}", "research-report"],
  "source": "./research/{query-slug}/report.md"
}
```

Also persist the knowledge base:
```json
{
  "content": "<content of ./research/{slug}/kb/merged_outputs.md>",
  "tags": ["{query-slug}", "{primary-topic}", "kb-cache"],
  "source": "./research/{query-slug}/kb/merged_outputs.md"
}
```

---

## Storage Details

- **Format**: SQLite with a single `memories` table
- **Embedding model**: `text-embedding-3-small` (1536 dimensions, ~$0.00002/1K tokens)
- **Search**: in-process cosine similarity — no external vector DB required
- **Portability**: single file at `~/.mofa/memory.db`, easily backed up or moved
