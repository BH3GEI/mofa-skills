---
name: news
description: Fetch news headlines and synthesize structured briefings from multiple sources.
requires_bins: curl
requires_env: GEMINI_API_KEY
---

# News Digest

Aggregate news from RSS feeds and APIs, then synthesize structured briefings.

## Trigger Phrases

- "今天有什么新闻"
- "科技新闻简报"
- "daily briefing"
- "news digest"
- " headlines"
- "今日要闻"

## Workflow

1. **Fetch** headlines via `web_fetch` or `curl`
2. **Rank** by recency and source credibility
3. **Deep-read** top articles via `web_fetch`
4. **Synthesize** structured digest with `write_file`

## RSS Sources

| Category | URL |
|----------|-----|
| Tech | `https://news.google.com/rss/topics/CAAqJggKIiBDQkFTRWdvSUwyMHZNRGRqTVhZU0FtVnVHZ0pWVXlnQVAB?hl=en-US` |
| World | `https://news.google.com/rss/topics/CAAqJggKIiBDQkFTRWdvSUwyMHZNRFZxYUdjU0FtVnVHZ0pWVXlnQVAB?hl=en-US` |
| HN | `https://hacker-news.firebaseio.com/v0/topstories.json` |

## Example Usage

```bash
# Fetch Google News tech feed
curl -s "https://news.google.com/rss/topics/CAAqJggKIiBDQkFTRWdvSUwyMHZNRGRqTVhZU0FtVnVHZ0pWVXlnQVAB?hl=en-US"

# Fetch HN top stories
curl -s "https://hacker-news.firebaseio.com/v0/topstories.json" | head -10
```

## Output Format

Save to `news-digest-YYYY-MM-DD.md`:

```markdown
# News Digest — 2026-03-11

## Technology
- **Headline** — Summary [Source](url)
- **Headline 2** — Summary [Source](url)

Key takeaway: ...

## World
...
```

## Deep Research Integration

For deeper analysis on a story, use `web_search` then `read_file` on results.

## Tips

- Google News RSS returns XML; use `grep` or `sed` to extract `<title>` and `<link>`
- HN API returns story IDs; fetch details at `v0/item/{id}.json`
- Cache results to avoid rate limits
