---
name: mofa-firecrawl
description: "基于 Firecrawl CLI 的深度内容爬取与分析管道，支持网站爬取、搜索、地图发现和浏览器自动化。Triggers: firecrawl, 爬取网站, 网站分析, web crawling, site analysis"
requires_bins: firecrawl
requires_env: FIRECRAWL_API_KEY
triggers: ["firecrawl", "爬取网站", "网站分析", "web crawling", "site analysis", "scrape website", "crawl site"]
always: false
---

# MOFA Firecrawl

基于 Firecrawl CLI 的深度内容获取与分析管道，支持网站爬取、智能搜索、URL 地图发现和浏览器自动化操作。

## Onboarding / 开始使用

### 前置要求

1. **安装 Firecrawl CLI**
   ```bash
   npm install -g firecrawl-cli
   ```

2. **获取 API Key**
   - 访问 https://firecrawl.dev 注册账号
   - 在控制台获取 API Key (格式: `fc-xxxxxxxx`)

3. **认证方式** (三选一)

   **方式 A: 命令行登录** (推荐)
   ```bash
   firecrawl login --api-key fc-YOUR-API-KEY
   ```

   **方式 B: 环境变量**
   ```bash
   export FIRECRAWL_API_KEY=fc-YOUR-API-KEY
   ```

   **方式 C: 本地开发** (无需 API Key)
   ```bash
   # 使用本地 Firecrawl 实例
   export FIRECRAWL_API_URL=http://localhost:3002
   ```

4. **验证安装**
   ```bash
   firecrawl --status
   ```
   预期输出应显示 "Authenticated" 和剩余额度。

### 快速开始

```bash
# 单页抓取
firecrawl "https://example.com" --only-main-content

# 整站爬取
firecrawl crawl "https://example.com" --limit 50 --wait

# 搜索并抓取
firecrawl search "keyword" --limit 5 --scrape

# URL 地图
firecrawl map "https://example.com"
```

### 故障排除

| 问题 | 解决方案 |
|-----|---------|
| `Not authenticated` | 运行 `firecrawl login` 或设置 `FIRECRAWL_API_KEY` |
| `429 Too Many Requests` | 降低并发，使用 `--delay` 参数 |
| 内容获取不完整 | 添加 `--wait-for 3000` 等待 JS 渲染 |
| 动态内容无法获取 | 使用 `firecrawl browser` 模式 |

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         MOFA FIRECRAWL PIPELINE                             │
└─────────────────────────────────────────────────────────────────────────────┘

Phase 1: TARGET (Smart URL Analysis)
────────────────────────────────────
Lead Agent 分析用户意图 → 确定最佳 Firecrawl 策略
                                          ↓
                              ┌─────────────────────┐
                              │  策略选择:          │
                              │  • scrape (单页)    │
                              │  • crawl (整站)     │
                              │  • map (URL 地图)   │
                              │  • search (搜索)    │
                              │  • browser (浏览器) │
                              └─────────────────────┘

Phase 2: ACQUIRE (Firecrawl Execution)
──────────────────────────────────────
                         ┌─────────────┐
                    ┌────┤  Firecrawl  │
                    │    │  Scrape     │
                    │    └─────────────┘
                    │    ┌─────────────┐
              并行或  ├────┤  Firecrawl  │
              串行   │    │  Crawl      │
              执行   │    └─────────────┘
                    │    ┌─────────────┐
                    ├────┤  Firecrawl  │
                    │    │  Map        │
                    │    └─────────────┘
                    │    ┌─────────────┐
                    └────┤  Firecrawl  │
                         │  Search     │
                         └─────────────┘

每种操作:
- Scrape: 单页深度抓取
- Crawl: 整站递归爬取
- Map: 快速发现所有 URL
- Search: 网络搜索 + 抓取
- Browser: 云端浏览器交互

Phase 3: PROCESS (Content Extraction)
────────────────────────────────────
原始内容
      ↓
┌─────────────────────────────────────────────────────────┐
│  • 提取正文内容 (markdown/html)                         │
│  • 提取结构化数据                                       │
│  • 提取链接和图片                                       │
│  • 提取元数据 (标题/作者/日期)                          │
│  • 生成内容摘要                                         │
└─────────────────────────────────────────────────────────┘
      ↓
Processed Content

Phase 4: ANALYZE (Cross-Reference)
───────────────────────────────────
分析所有获取的内容:
┌─────────────────────────────────────────────────────────┐
│  • 识别关键主题和实体                                    │
│  • 交叉引用多个来源                                      │
│  • 评估内容质量和可信度                                  │
│  • 发现内容缺口                                          │
│  • 标记矛盾信息                                          │
└─────────────────────────────────────────────────────────┘
                              ↓
                    → Analysis Report

Phase 5: SYNTHESIZE (Output Generation)
────────────────────────────────────────
生成结构化输出:
┌─────────────────────────────────────────────────────────┐
│  • 内容摘要                                              │
│  • 关键发现 (结构化数据)                                 │
│  • 完整内容导出 (markdown/json)                          │
│  • 数据来源列表                                          │
│                                                         │
│  MANDATORY: 保存到 ./firecrawl/{slug}/                  │
└─────────────────────────────────────────────────────────┘
```

## Firecrawl CLI 能力映射

| Firecrawl 命令 | 用途 | 使用场景 |
|---------------|------|---------|
| `scrape` | 单页深度抓取 | 获取特定页面完整内容 |
| `crawl` | 整站递归爬取 | 抓取整个网站或子目录 |
| `map` | URL 发现 | 快速获取网站所有页面列表 |
| `search` | 网络搜索 | 搜索并抓取相关结果 |
| `browser` | 云端浏览器 | 处理 JS 动态内容、自动化操作 |
| `agent` | AI 智能体 | 自然语言指令收集数据 |

## Phase 1: Target (策略规划)

**目标**: 分析用户意图，选择最佳 Firecrawl 策略

### 决策矩阵

| 用户意图 | 推荐策略 | 理由 |
|---------|---------|------|
| "分析某网站" | crawl + map | 全面覆盖 |
| "获取某页面" | scrape | 精准单页 |
| "找某类内容" | search | 全网搜索 |
| "列出所有文章" | map + filter | URL 发现 |
| "需要执行点击/填写" | browser | 浏览器自动化 |
| "提取结构化数据" | scrape --format json | 结构化输出 |

### 规划 Prompt

```
分析以下用户需求，确定 Firecrawl 执行策略:

需求: {{USER_INPUT}}

决策步骤:
1. 确定目标类型: 单页 | 整站 | 搜索结果 | 动态内容
2. 选择主策略: scrape | crawl | map | search | browser
3. 确定输出格式: markdown | html | json | links
4. 评估是否需要 --only-main-content
5. 评估是否需要浏览器渲染 (wait-for)

输出 JSON:
{
  "strategy": "scrape|crawl|map|search|browser",
  "urls": ["目标URL或搜索词"],
  "options": {
    "format": "markdown",
    "only_main_content": true,
    "wait_for": 0,
    "include_tags": [],
    "exclude_tags": []
  },
  "fallback_strategy": "备选策略"
}
```

## Phase 2: Acquire (Firecrawl 执行)

### 2.1 Scrape - 单页抓取

**适用场景**: 获取特定页面的完整内容

```bash
# 基础抓取 (markdown)
firecrawl scrape "https://example.com/article"

# 仅主内容 (去除导航/广告)
firecrawl "https://example.com/article" --only-main-content

# HTML 格式
firecrawl "https://example.com" --html

# 多格式输出
firecrawl "https://example.com" --format markdown,links,images

# 等待 JS 渲染
firecrawl "https://example.com" --wait-for 3000

# 保存到文件
firecrawl "https://example.com" -o ./firecrawl/{slug}/content.md
```

**输出格式选项**:
- `markdown` - 干净的 Markdown (默认)
- `html` - 处理的 HTML
- `rawHtml` - 原始 HTML
- `links` - 页面链接
- `images` - 图片列表
- `screenshot` - 截图
- `json` - JSON 结构化数据
- `summary` - 内容摘要

### 2.2 Crawl - 整站爬取

**适用场景**: 抓取整个网站或特定路径

```bash
# 基础爬取
firecrawl crawl "https://example.com" --wait

# 限制范围和深度
firecrawl crawl "https://example.com" \
  --limit 100 \
  --max-depth 3 \
  --wait

# 包含特定路径
firecrawl crawl "https://example.com" \
  --include-paths /blog,/docs \
  --wait

# 排除特定路径
firecrawl crawl "https://example.com" \
  --exclude-paths /admin,/login \
  --wait

# 包含子域名
firecrawl crawl "https://example.com" --allow-subdomains --wait

# 显示进度
firecrawl crawl "https://example.com" --wait --progress

# 保存结果
firecrawl crawl "https://example.com" --wait --pretty -o ./firecrawl/{slug}/crawl-results.json
```

### 2.3 Map - URL 地图

**适用场景**: 快速发现网站所有 URL

```bash
# 基础地图
firecrawl map "https://example.com"

# JSON 输出
firecrawl map "https://example.com" --json

# 过滤特定内容
firecrawl map "https://example.com" --search "blog"

# 包含子域名
firecrawl map "https://example.com" --include-subdomains

# 限制数量
firecrawl map "https://example.com" --limit 500

# 保存到文件
firecrawl map "https://example.com" -o ./firecrawl/{slug}/urls.txt
firecrawl map "https://example.com" --json --pretty -o ./firecrawl/{slug}/urls.json
```

### 2.4 Search - 网络搜索

**适用场景**: 搜索主题并抓取相关结果

```bash
# 基础搜索
firecrawl search "web scraping best practices"

# 限制结果数
firecrawl search "AI news" --limit 10

# 时间过滤
firecrawl search "tech news" --tbs qdr:d   # 最近一天
firecrawl search "tech news" --tbs qdr:w   # 最近一周
firecrawl search "tech news" --tbs qdr:m   # 最近一月

# 搜索并抓取结果
firecrawl search "documentation" \
  --scrape \
  --scrape-formats markdown \
  --pretty \
  -o ./firecrawl/{slug}/search-results.json
```

### 2.5 Browser - 云端浏览器

**适用场景**: 处理动态内容、执行交互操作

```bash
# 启动会话
firecrawl browser launch-session

# 打开页面
firecrawl browser execute "open https://example.com"

# 获取页面快照
firecrawl browser execute "snapshot"

# 点击元素 (使用 snapshot 中的 @ref)
firecrawl browser execute "click @e5"

# 填写表单
firecrawl browser execute "fill @e3 'search query'"

# 抓取当前页面
firecrawl browser execute "scrape"

# 关闭会话
firecrawl browser close
```

**Python/Node 代码执行**:
```bash
# Python (Playwright)
firecrawl browser execute --python '
await page.goto("https://example.com")
title = await page.title()
print(f"Title: {title}")
items = await page.query_selector_all(".article")
for item in items[:5]:
    print(await item.text_content())
'

# Node.js (Playwright)
firecrawl browser execute --node '
await page.goto("https://example.com");
const title = await page.title();
console.log(title);
'
```

## Phase 3: Process (内容处理)

### 处理流程

```
Firecrawl 输出
      ↓
┌─────────────────┐
│ 内容清洗        │ → 去除重复、格式化
└────────┬────────┘
         ↓
┌─────────────────┐
│ 结构提取        │ → 标题、段落、列表
└────────┬────────┘
         ↓
┌─────────────────┐
│ 元数据提取      │ → 日期、作者、标签
└────────┬────────┘
         ↓
┌─────────────────┐
│ 摘要生成        │ → 关键信息总结
└────────┬────────┘
         ↓
Processed Content
```

### 内容提取模板

**对于每页内容，提取**:

```json
{
  "source_url": "原始URL",
  "title": "页面标题",
  "content_type": "article|product|doc|other",
  "content": {
    "markdown": "正文内容",
    "summary": "内容摘要",
    "word_count": 1500
  },
  "metadata": {
    "author": "作者",
    "published_date": "发布日期",
    "modified_date": "修改日期",
    "tags": ["标签"]
  },
  "media": {
    "images": [{"url": "...", "alt": "..."}],
    "links": [{"url": "...", "text": "..."}]
  },
  "extracted_at": "ISO时间戳"
}
```

## Phase 4: Analyze (分析整合)

### 分析维度

1. **主题识别**: 主要内容类别
2. **实体提取**: 人名、公司、产品、地点
3. **时间线构建**: 事件和发布时间
4. **关联分析**: 内容间的引用关系
5. **质量评估**: 内容完整性和可信度

### 交叉引用表

| 主题 | 来源页面 | 提及次数 | 可信度 |
|-----|---------|---------|-------|
| ... | ... | ... | ... |

## Phase 5: Synthesize (输出生成)

### 输出结构

```
./firecrawl/{slug}/
├── index.md              # 项目索引
├── report.md             # 分析报告
├── content/              # 原始内容
│   ├── page-001.md
│   ├── page-002.md
│   └── ...
├── structured/           # 结构化数据
│   ├── entities.json
│   ├── timeline.json
│   └── links.json
└── summary.json          # 元数据摘要
```

### Report 模板

```markdown
# {目标} 内容分析报告

## 执行摘要
- 抓取策略: {strategy}
- 获取页面数: {count}
- 总内容量: {size}
- 执行时间: {duration}

## 关键发现

### 1. 主题分布
...

### 2. 重要内容
...

### 3. 数据洞察
...

## 数据来源
{来源列表}

## 内容详情
{详细内容}

## 附录
- 完整 URL 列表
- 提取的实体
- 时间线
```

## Usage Examples

### 示例 1: 单页深度分析

```
用户: "分析 https://example.com/about 页面"

执行:
1. firecrawl scrape "https://example.com/about" --only-main-content -o ./firecrawl/about/content.md
2. 提取结构化数据
3. 生成分析报告
```

### 示例 2: 整站内容抓取

```
用户: "抓取 https://docs.example.com 的所有文档"

执行:
1. firecrawl crawl "https://docs.example.com" \
     --include-paths /docs \
     --limit 500 \
     --max-depth 3 \
     --wait \
     --progress
2. 处理所有页面内容
3. 生成索引和报告
```

### 示例 3: 搜索 + 抓取

```
用户: "搜索 'Python 异步编程' 并抓取前 10 个结果"

执行:
1. firecrawl search "Python 异步编程" \
     --limit 10 \
     --scrape \
     --scrape-formats markdown \
     --pretty \
     -o ./firecrawl/python-async/results.json
2. 分析抓取的内容
3. 生成综合报告
```

### 示例 4: 动态内容抓取

```
用户: "抓取需要登录后才能看到的内容"

执行:
1. firecrawl browser launch-session
2. firecrawl browser execute "open https://example.com/login"
3. firecrawl browser execute "fill @username 'user@example.com'"
4. firecrawl browser execute "fill @password 'password'"
5. firecrawl browser execute "click @login-btn"
6. firecrawl browser execute "open https://example.com/protected"
7. firecrawl browser execute "scrape" -o ./firecrawl/protected/content.md
8. firecrawl browser close
```

## Error Handling

### 常见错误处理

| 错误 | 处理策略 |
|-----|---------|
| 429 Too Many Requests | 降低并发，增加 delay |
| Timeout | 增加 wait-for 时间 |
| Blocked | 使用 browser 模式 |
| Invalid URL | 验证 URL 格式 |
| Auth Required | 使用 browser 模式登录 |

### 容错机制

```bash
# 检查 firecrawl 状态
firecrawl --status

# 使用 --timeout 防止挂起
firecrawl crawl "https://example.com" --wait --timeout 300

# 保存中间结果
firecrawl crawl "https://example.com" --wait -o checkpoint.json
```

## Configuration

### 环境变量

```bash
# API Key (必需)
export FIRECRAWL_API_KEY=fc-YOUR-API-KEY

# 或使用本地实例
export FIRECRAWL_API_URL=http://localhost:3002

# 禁用遥测
export FIRECRAWL_NO_TELEMETRY=1
```

### 认证

```bash
# 登录
firecrawl login --api-key fc-YOUR-API-KEY

# 查看配置
firecrawl view-config

# 登出
firecrawl logout
```

## Output Requirements

### 必须生成的文件

1. `./firecrawl/{slug}/report.md` - 分析报告
2. `./firecrawl/{slug}/index.json` - 内容索引
3. `./firecrawl/{slug}/raw/` - 原始 Firecrawl 输出

### 质量检查清单

- [ ] 所有请求的 URL 都已处理
- [ ] 内容已正确格式化
- [ ] 元数据已提取
- [ ] 报告包含摘要
- [ ] 数据来源已记录

## Related Skills

### mofa-research-2.0
深度研究管道，使用 Tavily 搜索和并行分析。

**组合使用模式**:
```
mofa-firecrawl: 抓取目标网站内容
      ↓
mofa-research-2.0: 研究相关内容并交叉验证
      ↓
综合报告
```

### mofa-crawler
基于 Cloudflare Browser Rendering 的网页抓取。

**区别**:
- mofa-crawler: 基于 Cloudflare API，简单快速
- mofa-firecrawl: 基于 Firecrawl CLI，功能更全面 (crawl/map/search/browser)

### mofa-crawlee-python
基于 Crawlee-Python 的爬虫框架。

**组合使用**:
- mofa-firecrawl: 快速获取内容
- mofa-crawlee-python: 自定义复杂爬取逻辑
