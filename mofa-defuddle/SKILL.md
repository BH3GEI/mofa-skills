---
name: mofa-defuddle
description: "Extract clean article content from web pages using Defuddle. Converts cluttered HTML to clean Markdown or structured data. Triggers: defuddle, extract content, clean html, article extract, readable"
requires_bins: npx
always: false
---

# MOFA Defuddle

基于 [Defuddle](https://github.com/kepano/defuddle) 的网页内容提取工具，从杂乱的网页中提取干净的文章内容，转换为 Markdown 或结构化数据。

## Onboarding / 开始使用

### 前置要求

1. **Node.js 和 npm**
   ```bash
   node --version  # 需要 Node.js 14+
   npm --version
   ```

2. **Defuddle 会自动通过 npx 安装**
   ```bash
   npx defuddle --version
   ```

3. **验证安装**
   ```bash
   npx defuddle --version
   # 预期输出: 0.13.0 或更高版本
   ```

### 快速开始

```bash
# 从 URL 提取内容（Markdown 格式）
npx defuddle parse https://example.com/article --markdown

# 提取为 JSON（包含元数据）
npx defuddle parse https://example.com/article --json

# 保存到文件
npx defuddle parse https://example.com/article --markdown --output article.md

# 只提取标题
npx defuddle parse https://example.com/article --property title
```

### 故障排除

| 问题 | 解决方案 |
|-----|---------|
| `command not found: npx` | 安装 Node.js: https://nodejs.org |
| `fetch failed` | 检查网络连接，或网站有反爬机制 |
| `parse failed` | 页面可能是动态渲染，尝试用浏览器先获取 HTML |
| 内容提取不完整 | 某些网站结构特殊，尝试结合其他工具 |

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         MOFA DEFUDDLE PIPELINE                              │
└─────────────────────────────────────────────────────────────────────────────┘

User Request (URL or HTML file)
    ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 1: INPUT                                          │
│ ─────────────                                           │
│ • Validate URL or file path                             │
│ • Determine output format (markdown/json/property)      │
│ • Check if site needs special handling                  │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 2: EXTRACT                                        │
│ ───────────────                                         │
│                                                         │
│   npx defuddle parse <source> [options]                 │
│                                                         │
│   Options:                                              │
│   • --markdown : Clean markdown output                  │
│   • --json     : Structured data with metadata          │
│   • --property : Extract specific field                 │
│   • --output   : Save to file                           │
│                                                         │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 3: POST-PROCESS                                   │
│ ─────────────────────                                   │
│ • Clean up extracted content                            │
│ • Validate output quality                               │
│ • Format for downstream use                             │
└─────────────────────────┬───────────────────────────────┘
                          ↓
                    Processed Output
```

## Defuddle 能力映射

| 功能 | CLI 命令 | 用途 |
|-----|---------|------|
| **Markdown 提取** | `defuddle parse <url> --markdown` | 干净的文章内容 |
| **JSON 元数据** | `defuddle parse <url> --json` | 结构化数据（标题/作者/日期/内容）|
| **单字段提取** | `defuddle parse <url> --property title` | 提取特定字段 |
| **文件处理** | `defuddle parse <file.html> --markdown` | 处理本地 HTML |

## Output Format

### Markdown 输出

```markdown
# Article Title

Article content in clean markdown format...

- No ads
- No navigation
- No sidebars
- Just the article
```

### JSON 输出

```json
{
  "title": "Article Title",
  "description": "Article description or excerpt",
  "author": "Author Name",
  "date": "2024-01-15",
  "domain": "example.com",
  "content": "Clean HTML content...",
  "markdown": "# Article Title\n\nContent..."
}
```

## Phase 1: Input Validation

```bash
# 检查输入类型
if [[ "$input" =~ ^https?:// ]]; then
    type="url"
elif [[ -f "$input" ]]; then
    type="file"
else
    echo "Error: Input must be a URL or existing file"
    exit 1
fi
```

## Phase 2: Extraction

### 提取模式选择

| 用户需求 | 推荐命令 | 说明 |
|---------|---------|------|
| "提取文章内容" | `--markdown` | 最常用，干净可读 |
| "获取文章元数据" | `--json` | 包含标题、作者、日期等 |
| "只取标题" | `--property title` | 精准提取 |
| "需要完整数据" | `--json` | 包含原始 HTML 和 Markdown |

### 基础提取

```bash
# 标准 Markdown 提取
npx defuddle parse "https://example.com/article" --markdown

# JSON 格式（推荐用于后续处理）
npx defuddle parse "https://example.com/article" --json

# 保存到文件
npx defuddle parse "https://example.com/article" --markdown --output ./output/article.md
```

### 批量处理

```bash
# 批量提取多个 URL
urls=(
    "https://site.com/article-1"
    "https://site.com/article-2"
    "https://site.com/article-3"
)

for url in "${urls[@]}"; do
    slug=$(echo "$url" | sed 's/[^a-zA-Z0-9]/-/g')
    npx defuddle parse "$url" --markdown --output "./output/${slug}.md"
done
```

## Phase 3: Post-Processing

### 质量检查

```bash
# 检查提取结果
if [[ -s output.md && $(wc -c < output.md) -gt 100 ]]; then
    echo "Extraction successful"
else
    echo "Extraction failed or content too short"
fi
```

### 与其他工具配合

```bash
# 提取后用 glow 预览
npx defuddle parse https://example.com --markdown | glow -

# 提取后统计字数
npx defuddle parse https://example.com --markdown | wc -w
```

## Usage Examples

### 示例 1: 基础文章提取

```
用户: "提取这篇文章 https://blog.example.com/my-post"

执行:
npx defuddle parse "https://blog.example.com/my-post" --markdown --output ./defuddle/my-post.md

输出: ./defuddle/my-post.md (干净的 Markdown)
```

### 示例 2: 获取元数据

```
用户: "获取这个网页的标题和作者 https://news.example.com/story"

执行:
npx defuddle parse "https://news.example.com/story" --json | jq '{title, author, date}'

输出:
{
  "title": "Story Title",
  "author": "Jane Doe",
  "date": "2024-03-14"
}
```

### 示例 3: 批量文章提取

```
用户: "提取这 5 个链接的内容"

执行:
urls=("url1" "url2" "url3" "url4" "url5")
for url in "${urls[@]}"; do
    npx defuddle parse "$url" --markdown --output "./defuddle/$(basename $url).md"
done
```

### 示例 4: 配合研究流程

```
mofa-research-2.0: 发现文章 URL
      ↓
mofa-defuddle: 提取文章内容
      ↓
mofa-research-2.0: 分析提取的内容
      ↓
生成综合报告
```

## Configuration

### 环境变量

```bash
# 可选: 设置默认输出目录
export DEFUDDLE_OUTPUT_DIR="./extracted"

# 可选: 设置请求超时
export DEFUDDLE_TIMEOUT="30000"
```

## Error Handling

### 常见错误

| 错误 | 原因 | 解决方案 |
|-----|------|---------|
| `fetch failed` | 网络问题或反爬 | 尝试其他工具如 mofa-firecrawl |
| `parse failed` | 页面结构不标准 | 检查是否为单页应用 (SPA) |
| `content empty` | 提取失败 | 页面可能需要 JavaScript 渲染 |
| `timeout` | 请求超时 | 检查网络或增加超时时间 |

### 降级策略

```bash
# Defuddle 失败时，尝试其他方案
defuddle_extract() {
    url=$1
    output=$2

    # 尝试 defuddle
    if npx defuddle parse "$url" --markdown --output "$output" 2>/dev/null; then
        echo "Defuddle succeeded"
        return 0
    fi

    # 降级到 firecrawl
    echo "Defuddle failed, trying firecrawl..."
    firecrawl "$url" -o "$output"
}
```

## Output Requirements

### 必须生成的文件

```
./defuddle/{task-slug}/
├── content.md          # 提取的 Markdown 内容
├── metadata.json       # 元数据（如果用 --json）
└── report.md           # 执行报告（可选）
```

### 质量检查清单

- [ ] 内容不为空
- [ ] 标题已提取
- [ ] Markdown 格式正确
- [ ] 无乱码
- [ ] 原文链接已记录

## Related Skills

### mofa-research-2.0
深度研究管道。

**组合使用模式**:
```
mofa-research-2.0: 搜索发现文章 URL
      ↓
mofa-defuddle: 批量提取文章内容
      ↓
mofa-research-2.0: 分析提取的内容 → 研究报告
```

### mofa-firecrawl
高级网页爬取工具。

**区别**:
- mofa-defuddle: 轻量快速，专门提取文章内容，适合已知 URL
- mofa-firecrawl: 功能全面，支持整站爬取、搜索、浏览器自动化

**组合使用**:
```
mofa-firecrawl: 发现目标文章 URL
      ↓
mofa-defuddle: 快速提取文章内容
      ↓
综合分析
```

### mofa-crawler
Cloudflare Browser Rendering 爬取。

**区别**:
- mofa-defuddle: 纯内容提取，输出 Markdown
- mofa-crawler: 全站爬取，输出多页面

### mofa-pinchtab
轻量级浏览器控制。

**组合使用**:
```
mofa-pinchtab: 获取页面 HTML
      ↓
mofa-defuddle: 提取干净内容
      ↓
分析处理
```

## Comparison: Content Extraction Tools

| 工具 | 速度 | 输出格式 | 最佳场景 | 依赖 |
|-----|------|---------|---------|------|
| **defuddle** | 快 | Markdown/JSON | 文章内容提取 | Node.js |
| **firecrawl** | 中 | Markdown/JSON/HTML | 大规模爬取、搜索 | Firecrawl API |
| **crawler** | 中 | Markdown/JSON/HTML | Cloudflare 生态 | CF API |
| **pinchtab** | 快 | Text | Token 高效提取 | 本地 Chrome |
| **readability** | 快 | HTML | 浏览器内置 | 无 |

## Notes

- Defuddle 专为文章类页面优化，对列表页、首页效果可能不佳
- 某些网站有反爬机制，可能需要配合 mofa-firecrawl 或 mofa-verge-browser 使用
- 提取质量取决于网页结构，标准博客/新闻网站效果最佳
