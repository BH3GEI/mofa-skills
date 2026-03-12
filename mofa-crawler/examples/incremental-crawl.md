# Example: Incremental Crawling

## Scenario

**User**: "只爬取最近一个月更新的文档"

## Config

```json
{
  "url": "https://docs.example.com",
  "limit": 500,
  "formats": ["markdown"],
  "modifiedSince": "2024-02-01T00:00:00Z"
}
```

## Use Case

**Weekly Sync Workflow:**

```bash
#!/bin/bash
# weekly-sync.sh

LAST_CRAWL=$(cat last_crawl.txt)

curl -X POST "https://api.cloudflare.com/client/v4/accounts/{id}/browser-rendering/crawl" \
  -H "Authorization: Bearer {token}" \
  -d "{
    \"url\": \"https://docs.example.com\",
    \"limit\": 500,
    \"formats\": [\"markdown\"],
    \"modifiedSince\": \"$LAST_CRAWL\"
  }"

date -u +%Y-%m-%dT%H:%M:%SZ > last_crawl.txt
```

## Benefits

- Faster (only changed pages)
- Cheaper (fewer pages)
- Keeps knowledge base fresh
