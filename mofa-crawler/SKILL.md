---
name: mofa-crawler
description: "Web crawling via Cloudflare Browser Rendering API - full-site extraction with JavaScript rendering, AI structured extraction"
triggers: ["crawl", "爬虫", "抓取", "爬取网站", "cloudflare crawl"]
requires_env: [CF_API_TOKEN, CF_ACCOUNT_ID]
requires_bins: curl
always: false
---

# MOFA Crawler

Cloudflare Browser Rendering Crawl API 集成 —— 一站式网站抓取解决方案。

> 做反爬虫起家的 Cloudflare，转头发布了爬虫 API。
>
> 免费版：5 任务/天，100 页/任务
>
> 付费版：$5/月，1000 任务/天，1000 页/任务

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    CLOUDFLARE CRAWLER PIPELINE                              │
└─────────────────────────────────────────────────────────────────────────────┘

User Request
    ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 1: ENTRY (Strong Model)                           │
│ ────────────────────────────                            │
│ • Analyze user intent                                   │
│ • Validate target URL                                   │
│ • Determine crawl strategy                              │
│   - Limit: pages to crawl                               │
│   - Format: html | markdown | json                      │
│   - JS rendering: true | false                          │
│   - AI extraction: field definitions                    │
│   - Filters: include/exclude patterns                   │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 2: DISCOVERY (API Client)                         │
│ ───────────────────────────────                         │
│                                                         │
│  Step 1: Submit Crawl Job                               │
│  POST /accounts/{id}/browser-rendering/crawl            │
│       ↓                                                 │
│  Return: job_id                                         │
│       ↓                                                 │
│  Step 2: Poll Status (async)                            │
│  GET /accounts/{id}/browser-rendering/crawl/{job_id}    │
│       ↓                                                 │
│  Status: queued → crawling → completed | failed         │
│       ↓                                                 │
│  Retry on failure (max 3)                               │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 3: ANALYZE (Strong Model)                         │
│ ─────────────────────────────                           │
│ • Parse crawled content                                 │
│ • Extract key information                               │
│ • Apply user-defined filters                            │
│ • Validate data quality                                 │
│                                                         │
│ Output: Structured content with metadata                │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 4: SYNTHESIZE (Output Formatter)                  │
│ ─────────────────────────────────────                   │
│ • Format based on user request                          │
│   - Markdown: clean text for AI/RAG                     │
│   - JSON: structured data with AI extraction            │
│   - HTML: raw page content                              │
│ • Generate summary statistics                           │
│ • Save to ./crawl/{domain}/{timestamp}/                 │
│   - content.{md|json|html}                              │
│   - summary.md                                          │
│   - metadata.json                                       │
└─────────────────────────────────────────────────────────┘
```

## Quick Start (5 minutes)

### TL;DR

```bash
# 1. 复制配置模板
cp .env.example .env

# 2. 编辑 .env 填入你的凭证
# CF_ACCOUNT_ID=xxxxx  (从 dash.cloudflare.com 右侧获取)
# CF_API_TOKEN=xxxxx   (从 My Profile → API Tokens 创建)

# 3. 测试
export $(cat .env | xargs)
curl -H "Authorization: Bearer $CF_API_TOKEN" \
  https://api.cloudflare.com/client/v4/accounts/$CF_ACCOUNT_ID/browser-rendering
```

---

## Prerequisites

### Step 1: Get Account ID

1. 登录 https://dash.cloudflare.com (免费注册)
2. 查看页面**右侧边栏**
3. 找到 **Account ID** (32位字符串，如 `1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p`)
4. 复制备用

### Step 2: Create API Token

1. 登录后点击右上角**头像** → **My Profile**
2. 选择 **API Tokens** 标签
3. 点击 **Create Token**
4. 选择 **Custom token** 模板
5. 填写配置：
   - **Token name**: `crawler-token`
   - **Permissions**:
     - Account → Browser Rendering → Edit
   - **(可选) Client IP Address Filtering**: 限制你的IP更安全
6. 点击 **Continue to summary** → **Create Token**
7. ⚠️ **立即复制 token**（只显示一次！）

### Step 3: Configure Environment

**方式 1: 环境变量（推荐）**
```bash
export CF_API_TOKEN="your_api_token_here"
export CF_ACCOUNT_ID="your_account_id_here"
```

**方式 2: .env 文件**
```bash
cp .env.example .env
# 编辑 .env 填入真实值
```

**方式 3: 验证配置**
```bash
curl -X GET "https://api.cloudflare.com/client/v4/accounts/$CF_ACCOUNT_ID/browser-rendering" \
  -H "Authorization: Bearer $CF_API_TOKEN"
```

返回 `{"success": true}` 表示成功！🎉

## API Reference

### Submit Crawl Job

```bash
POST https://api.cloudflare.com/client/v4/accounts/{account_id}/browser-rendering/crawl
```

**Request Body:**
```json
{
  "url": "https://example.com",
  "limit": 50,
  "formats": ["markdown"],
  "render": true,
  "includePatterns": ["/docs/**", "/blog/**"],
  "excludePatterns": ["/admin/**"],
  "modifiedSince": "2024-01-01T00:00:00Z"
}
```

### Poll Job Status

```bash
GET https://api.cloudflare.com/client/v4/accounts/{account_id}/browser-rendering/crawl/{job_id}
```

**Response:**
```json
{
  "success": true,
  "result": {
    "jobId": "abc123",
    "status": "completed",
    "pages": [
      {
        "url": "https://example.com/page1",
        "title": "Page Title",
        "content": "...markdown content..."
      }
    ]
  }
}
```

## Features

### 1. Full-Site Crawling

Give a starting URL,系统自动发现所有页面：

```bash
# Crawl entire site
POST /crawl
{
  "url": "https://docs.example.com",
  "limit": 100
}
```

### 2. JavaScript Rendering

启动真实 Chrome 浏览器渲染页面：

```json
{
  "url": "https://react-app.com",
  "render": true  // Wait for JS to load
}
```

For static sites, set `render: false` for speed.

### 3. AI Structured Extraction (JSON + AI)

Use natural language to extract structured data：

```json
{
  "url": "https://ecommerce.com/products",
  "formats": ["json"],
  "ai": {
    "prompt": "Extract product name, price, and description"
  }
}
```

### 4. Incremental Crawling

只爬最近更新的页面：

```json
{
  "url": "https://blog.com",
  "modifiedSince": "2024-03-01T00:00:00Z"
}
```

### 5. URL Filtering

Include/exclude patterns with wildcards：

```json
{
  "url": "https://site.com",
  "includePatterns": ["/docs/**", "/api/**"],
  "excludePatterns": ["/docs/legacy/**", "**/*.pdf"]
}
```

## Output Formats

| Format | Use Case | Description |
|--------|----------|-------------|
| **Markdown** | AI/RAG pipelines | Clean text, easy to chunk |
| **JSON** | Data analysis | Structured with AI extraction |
| **HTML** | Archive/backup | Raw page content |

## Pricing & Limits

| Tier | Price | Tasks/Day | Pages/Task | Browser Time |
|------|-------|-----------|------------|--------------|
| **Free** | $0 | 5 | 100 | 10 min |
| **Paid** | $5/month | 1000 | 1000 | 60 min |

**Result Retention:** 14 days

**Max Job Runtime:** 7 days

## Comparison with Alternatives

| Tool | Price | Setup | JS Rendering | AI Extraction |
|------|-------|-------|--------------|---------------|
| **Cloudflare Crawl** | $5/mo | Zero | ✅ | ✅ |
| Firecrawl | $47/mo | API key | ✅ | ✅ |
| Crawl4AI | Free | Self-hosted | ✅ | ❌ |
| Jina Reader | Free | API key | ❌ | ❌ |

## Use Cases

### RAG Pipeline Data Ingestion

```bash
# Crawl docs site to Markdown
POST /crawl
{
  "url": "https://docs.framework.com",
  "limit": 500,
  "formats": ["markdown"],
  "includePatterns": ["/docs/**"]
}

# Result: Clean markdown ready for vector DB
```

### Competitor Content Analysis

```bash
# Extract structured product data
POST /crawl
{
  "url": "https://competitor.com/products",
  "formats": ["json"],
  "ai": {
    "prompt": "Extract product name, price, description, rating"
  }
}
```

### CMS Migration

```bash
# Old site → Markdown → New CMS
POST /crawl
{
  "url": "https://old-blog.com",
  "formats": ["markdown"],
  "limit": 1000
}
```

## Error Handling

| Error | Reason | Solution |
|-------|--------|----------|
| 401 | Invalid API token | Check CF_API_TOKEN |
| 403 | Account not enabled | Enable Browser Rendering |
| 429 | Rate limit | Wait or upgrade plan |
| 500 | Crawl failed | Retry with smaller limit |

## Limitations

- ⚠️ No screenshots (use /screenshot endpoint separately)
- ⚠️ Can't bypass bot protection (by design)
- ⚠️ Respects robots.txt
- ⚠️ Free tier very limited for serious use

## Safety & Ethics

- Always check target site's robots.txt
- Respect rate limits
- Don't crawl private/authenticated content without permission
- Consider content ownership and copyright

## Output Directory Structure

```
./crawl/
└── {domain}/
    └── {timestamp}/
        ├── content.md       # Main content
        ├── content.json     # Structured data
        ├── summary.md       # Crawl summary
        ├── metadata.json    # Job metadata
        └── pages/           # Individual pages
            ├── index.md
            ├── about.md
            └── ...
```
