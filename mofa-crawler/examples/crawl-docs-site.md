# Example: Crawl Documentation Site for RAG

## Scenario

**User**: "爬取 React 官方文档，用于我的 RAG 知识库"

## Phase 1: Entry

**Analysis:**
- Intent: RAG data ingestion
- Target: https://react.dev
- Site type: React-based (JS rendering needed)
- Estimated size: Medium (100-500 pages)

**Config:**
```json
{
  "target_url": "https://react.dev",
  "crawl_config": {
    "limit": 300,
    "formats": ["markdown"],
    "render": true,
    "include_patterns": ["/learn/**", "/reference/**"],
    "exclude_patterns": ["/blog/**", "/community/**"]
  }
}
```

## Phase 2: Discovery

**Submit:**
```bash
curl -X POST "https://api.cloudflare.com/client/v4/accounts/{id}/browser-rendering/crawl" \
  -H "Authorization: Bearer {token}" \
  -d '{
    "url": "https://react.dev",
    "limit": 300,
    "formats": ["markdown"],
    "render": true,
    "includePatterns": ["/learn/**", "/reference/**"]
  }'
```

**Response:**
```json
{"result": {"jobId": "react_docs_001"}}
```

**Poll:**
```bash
# Every 10 seconds...
curl "https://api.cloudflare.com/client/v4/accounts/{id}/browser-rendering/crawl/react_docs_001" \
  -H "Authorization: Bearer {token}"
```

## Phase 3: Analyze

**Analysis Results:**
```json
{
  "total_pages": 287,
  "topics": [
    {"topic": "React Hooks", "pages": 45},
    {"topic": "Components", "pages": 38},
    {"topic": "State Management", "pages": 32}
  ],
  "avg_page_length": 3500
}
```

## Phase 4: Synthesize

**Output:**
```
./crawl/react.dev/2024-03-12_10-00-00/
├── content.md (12.5 MB)
├── summary.md
└── metadata.json
```

**Usage for RAG:**
```python
# Chunk content.md
# Embed and store in vector DB
# Use for React Q&A
```
