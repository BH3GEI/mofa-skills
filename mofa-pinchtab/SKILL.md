---
name: mofa-pinchtab
description: "基于 PinchTab 的轻量级 AI 浏览器控制，HTTP API 驱动的高效网页抓取，支持多实例并行和文本提取。Triggers: pinchtab, browser control, web extraction, text scraping"
requires_bins: pinchtab
requires_env: PINCHTAB_API_URL
always: false
---

# MOFA PinchTab

基于 [PinchTab](https://github.com/pinchtab/pinchtab) 的轻量级 AI 浏览器控制 skill。通过 HTTP API 控制 Chrome，支持多实例并行，以极低的 token 消耗实现高效的网页文本提取。

## Onboarding / 开始使用

### 前置要求

1. **安装 PinchTab**
   ```bash
   # 方式 A: 一键脚本安装 (推荐)
   curl -fsSL https://pinchtab.com/install.sh | bash

   # 方式 B: npm 安装
   npm install -g pinchtab

   # 方式 C: 直接下载二进制
   # 从 https://github.com/pinchtab/pinchtab/releases 下载对应平台的二进制文件
   ```

2. **启动服务**
   ```bash
   # 方式 A: 守护进程模式 (推荐)
   pinchtab daemon install   # 安装用户级守护进程
   pinchtab daemon start     # 启动守护进程

   # 方式 B: 直接运行服务器
   pinchtab server

   # 方式 C: 单实例模式
   pinchtab bridge
   ```

3. **验证安装**
   ```bash
   # 检查版本
   pinchtab --version

   # 检查服务状态
   curl http://localhost:9867/health
   ```

4. **环境变量配置** (可选)
   ```bash
   # PinchTab API 地址 (默认)
   export PINCHTAB_API_URL=http://localhost:9867

   # Chrome 路径 (如需指定)
   export PINCHTAB_CHROME_PATH=/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome
   ```

### 快速开始

```bash
# 1. 导航到页面
pinchtab nav https://example.com

# 2. 获取页面结构
pinchtab snap -i -c

# 3. 提取纯文本 (token 高效!)
pinchtab text

# 4. 点击元素
pinchtab click @ref-1

# 5. 截图 (可选)
pinchtab snap
```

### 故障排除

| 问题 | 解决方案 |
|-----|---------|
| `pinchtab: command not found` | 重新运行安装脚本，或检查 PATH 是否包含 `~/.local/bin` |
| `Failed to connect` | 确保 daemon/server/bridge 已启动，检查端口 9867 |
| `Chrome not found` | 设置 `PINCHTAB_CHROME_PATH` 指向 Chrome 可执行文件 |
| `Navigation timeout` | 增加超时时间，或检查网络连接 |
| `rate limited` | 降低请求频率，PinchTab 虽快但也需遵守网站规则 |

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        MOFA PINCHTAB PIPELINE                               │
└─────────────────────────────────────────────────────────────────────────────┘

Phase 1: SESSION (Session Management)
─────────────────────────────────────
创建/管理浏览器会话
      ↓
┌─────────────────────────────────────────────────────────┐
│  • 启动 PinchTab 服务 (daemon/server/bridge)            │
│  • 创建新会话 (多实例并行)                               │
│  • 配置浏览器参数 (headless/headed)                     │
└─────────────────────────┬───────────────────────────────┘
                          ↓
                ┌─────────────────────┐
                │  Session ID         │
                │  HTTP API Endpoint  │
                └─────────────────────┘

Phase 2: NAVIGATION (Page Control)
──────────────────────────────────
                         ┌─────────────┐
                    ┌────┤  Navigate   │
                    │    │  导航到URL  │
                    │    └─────────────┘
                    │    ┌─────────────┐
              页面  ├────┤  Snapshot   │
              操作  │    │  获取结构   │
                    │    └─────────────┘
                    │    ┌─────────────┐
                    └────┤  Interact   │
                         │  点击/输入  │
                         └─────────────┘

操作类型:
- nav: 页面导航
- snap: 获取页面结构 (带元素引用)
- click: 点击指定元素
- type: 输入文本
- text: 提取纯文本
- scroll: 页面滚动

Phase 3: EXTRACTION (Content Extraction)
────────────────────────────────────────
提取目标数据:
┌─────────────────────────────────────────────────────────┐
│  • 纯文本提取 (token 高效)                               │
│  • 结构化数据 (snap -i -c)                               │
│  • 截图证据 (snap)                                       │
│  • 元素属性 (通过 API)                                   │
└─────────────────────────┬───────────────────────────────┘
                          ↓
                    Extracted Data

Phase 4: MULTI-INSTANCE (Parallel Processing)
────────────────────────────────────────────
多实例并行处理:
┌─────────────────────────────────────────────────────────┐
│  Session 1    Session 2    Session 3    Session N       │
│     │            │            │            │            │
│     └────────────┴────────────┴────────────┘            │
│                    │                                     │
│              并行提取                                    │
│                    │                                     │
│              结果合并                                    │
└─────────────────────────────────────────────────────────┘

Phase 5: OUTPUT
───────────────
生成结构化输出:
- 提取的文本内容
- 结构化数据 (JSON)
- 截图证据
- 操作日志
```

## PinchTab 能力映射

| 能力 | CLI 命令 | HTTP API | 用途 |
|-----|---------|----------|------|
| **导航** | `pinchtab nav <url>` | `POST /navigate` | 页面跳转 |
| **快照** | `pinchtab snap` | `POST /snapshot` | 获取页面结构 |
| **点击** | `pinchtab click <ref>` | `POST /click` | 元素点击 |
| **输入** | `pinchtab type <ref> <text>` | `POST /type` | 文本输入 |
| **提取** | `pinchtab text` | `POST /extract` | 纯文本提取 |
| **截图** | `pinchtab snap -o file.png` | `POST /screenshot` | 页面截图 |

## Phase 1: Session (会话管理)

### 启动服务

```bash
# 守护进程模式 (推荐长期使用)
pinchtab daemon install    # 安装用户级服务
pinchtab daemon start      # 启动
pinchtab daemon stop       # 停止
pinchtab daemon status     # 查看状态

# 服务器模式
pinchtab server            # 前台运行

# 桥接模式 (单实例)
pinchtab bridge            # 快速测试
```

### 多实例管理

```bash
# 创建多个独立会话
# 每个会话有独立的 Chrome 实例和配置文件

# 会话 1 - 任务 A
curl -X POST http://localhost:9867/session \
  -H "Content-Type: application/json" \
  -d '{"name": "task-a", "headless": false}'

# 会话 2 - 任务 B
curl -X POST http://localhost:9867/session \
  -H "Content-Type: application/json" \
  -d '{"name": "task-b", "headless": true}'

# 查看会话列表
curl http://localhost:9867/sessions

# 删除会话
curl -X DELETE http://localhost:9867/session/task-a
```

### CLI 命令参考

```bash
# 导航
pinchtab nav https://example.com
pinchtab nav https://example.com --wait 5000  # 等待 5 秒

# 快照 (获取页面结构)
pinchtab snap                # 截图
pinchtab snap -i             # 包含交互元素信息
pinchtab snap -c             # 紧凑格式
pinchtab snap -i -c          # 推荐: 交互+紧凑
pinchtab snap -o page.png    # 保存截图

# 点击
pinchtab click @ref-1        # 点击引用为 ref-1 的元素
pinchtab click @ref-1 --wait 2000  # 点击后等待 2 秒

# 输入文本
pinchtab type @ref-input "hello world"

# 提取文本 (token 高效!)
pinchtab text                # 提取页面纯文本
pinchtab text --selector "article"  # 提取特定区域

# 滚动
pinchtab scroll down         # 向下滚动
pinchtab scroll down 500     # 滚动 500 像素
pinchtab scroll up 300       # 向上滚动 300 像素

# 获取页面信息
pinchtab info                # URL、标题等
pinchtab url                 # 当前 URL
pinchtab title               # 页面标题
```

## Phase 2: Navigation (页面控制)

### 基本导航流程

```bash
#!/bin/bash
# basic-workflow.sh

URL="https://example.com"

# 1. 导航
echo "Navigating to $URL..."
pinchtab nav "$URL" --wait 3000

# 2. 获取页面结构
echo "Getting page structure..."
STRUCTURE=$(pinchtab snap -i -c)
echo "$STRUCTURE"

# 3. 提取文本 (最省 token!)
echo "Extracting text..."
TEXT=$(pinchtab text)
echo "$TEXT"

# 4. 如有需要，点击交互元素
# pinchtab click @ref-1
```

### HTTP API 使用

```bash
# 导航
POST http://localhost:9867/navigate
Content-Type: application/json

{
  "url": "https://example.com",
  "wait": 3000
}

# 快照
POST http://localhost:9867/snapshot
Content-Type: application/json

{
  "interactive": true,
  "compact": true
}

# 点击
POST http://localhost:9867/click
Content-Type: application/json

{
  "ref": "ref-1",
  "wait": 2000
}

# 提取文本
POST http://localhost:9867/extract
Content-Type: application/json

{
  "format": "text"
}
```

## Phase 3: Extraction (内容提取)

### Token 高效提取

PinchTab 的核心优势：文本提取约 800 tokens/页，比截图便宜 5-13 倍！

```bash
# 推荐提取流程

# 1. 获取带引用的页面结构
SNAPSHOT=$(pinchtab snap -i -c)
# 输出示例:
# [ref-1] <button>Submit</button>
# [ref-2] <a href="/about">About</a>
# [ref-3] <h1>Page Title</h1>

# 2. 提取纯文本 (最省 token)
CONTENT=$(pinchtab text)
# 或提取特定区域
CONTENT=$(pinchtab text --selector "main article")

# 3. 如需交互，使用引用点击
curl -X POST http://localhost:9867/click \
  -H "Content-Type: application/json" \
  -d '{"ref": "ref-1"}'
```

### Python 提取示例

```python
import requests

PINCHTAB_URL = "http://localhost:9867"

def extract_page(url):
    """提取页面内容"""
    # 导航
    requests.post(f"{PINCHTAB_URL}/navigate", json={"url": url, "wait": 3000})

    # 获取结构
    snapshot = requests.post(f"{PINCHTAB_URL}/snapshot", json={"interactive": True, "compact": True})
    structure = snapshot.json()

    # 提取文本
    extract = requests.post(f"{PINCHTAB_URL}/extract", json={"format": "text"})
    text = extract.text

    return {
        "structure": structure,
        "text": text,
        "token_estimate": len(text) / 4  # 粗略估算
    }

# 使用
result = extract_page("https://example.com")
print(f"Text: {result['text'][:500]}...")
print(f"Estimated tokens: {result['token_estimate']}")
```

## Phase 4: Multi-Instance (并行处理)

### 多会话并行架构

```python
import asyncio
import aiohttp

async def process_with_session(session_name, url):
    """在独立会话中处理 URL"""
    async with aiohttp.ClientSession() as http:
        # 创建会话
        await http.post("http://localhost:9867/session", json={
            "name": session_name,
            "headless": True
        })

        try:
            # 使用会话特定端点
            base = f"http://localhost:9867/session/{session_name}"

            # 导航
            await http.post(f"{base}/navigate", json={"url": url, "wait": 3000})

            # 提取
            async with http.post(f"{base}/extract", json={"format": "text"}) as resp:
                text = await resp.text()

            return {"session": session_name, "url": url, "text": text[:1000]}

        finally:
            # 清理
            await http.delete(f"http://localhost:9867/session/{session_name}")

async def parallel_extract(urls):
    """并行处理多个 URL"""
    tasks = [
        process_with_session(f"task-{i}", url)
        for i, url in enumerate(urls)
    ]
    return await asyncio.gather(*tasks)

# 使用
urls = [
    "https://example.com/page1",
    "https://example.com/page2",
    "https://example.com/page3"
]
results = asyncio.run(parallel_extract(urls))
```

## Usage Examples

### 示例 1: 批量文章提取

```
用户: "提取这 10 个博客文章的内容"

执行:
1. pinchtab daemon start (如未运行)
2. for url in urls:
   - pinchtab nav $url --wait 3000
   - pinchtab text > ./content/$(basename $url).txt
   - pinchtab snap -o ./screenshots/$(basename $url).png
3. 生成汇总报告
```

### 示例 2: 表单自动化

```
用户: "帮我自动填写并提交表单"

执行:
1. pinchtab nav https://example.com/form
2. STRUCTURE=$(pinchtab snap -i -c)
3. 分析结构找到输入框引用
4. pinchtab type @ref-name "John Doe"
5. pinchtab type @ref-email "john@example.com"
6. pinchtab click @ref-submit
7. pinchtab text 查看提交结果
```

### 示例 3: 分页内容抓取

```bash
#!/bin/bash
# crawl-pagination.sh

BASE_URL="https://example.com/list"
PAGE=1

while true; do
    echo "Processing page $PAGE..."

    # 导航到页面
    pinchtab nav "${BASE_URL}?page=$PAGE" --wait 2000

    # 提取内容
    pinchtab text > "./output/page-${PAGE}.txt"

    # 截图
    pinchtab snap -o "./screenshots/page-${PAGE}.png"

    # 检查是否有下一页
    STRUCTURE=$(pinchtab snap -i -c)
    if ! echo "$STRUCTURE" | grep -q "next"; then
        echo "No more pages"
        break
    fi

    # 点击下一页
    pinchtab click @ref-next --wait 2000
    PAGE=$((PAGE + 1))
done
```

### 示例 4: 多实例并行搜索

```python
# parallel_search.py
import asyncio
import aiohttp

SEARCH_URLS = [
    "https://site1.com/search?q=python",
    "https://site2.com/search?q=python",
    "https://site3.com/search?q=python",
]

async def search_and_extract(session_name, url):
    async with aiohttp.ClientSession() as http:
        # 创建隔离会话
        await http.post("http://localhost:9867/session", json={
            "name": session_name,
            "headless": True
        })

        try:
            base = f"http://localhost:9867/session/{session_name}"

            # 搜索
            await http.post(f"{base}/navigate", json={"url": url, "wait": 5000})

            # 提取结果
            async with http.post(f"{base}/extract", json={
                "format": "text",
                "selector": ".search-results"
            }) as resp:
                return await resp.text()

        finally:
            await http.delete(f"http://localhost:9867/session/{session_name}")

async def main():
    tasks = [
        search_and_extract(f"search-{i}", url)
        for i, url in enumerate(SEARCH_URLS)
    ]
    results = await asyncio.gather(*tasks)

    for url, result in zip(SEARCH_URLS, results):
        print(f"\n=== {url} ===")
        print(result[:500])

asyncio.run(main())
```

## Configuration

### 环境变量

```bash
# API 端点
export PINCHTAB_API_URL=http://localhost:9867

# Chrome 路径 (如需指定)
export PINCHTAB_CHROME_PATH=/usr/bin/google-chrome

# 默认超时
export PINCHTAB_DEFAULT_TIMEOUT=30000

# 日志级别
export PINCHTAB_LOG_LEVEL=info
```

### 配置文件

```json
{
  "server": {
    "port": 9867,
    "host": "127.0.0.1"
  },
  "browser": {
    "headless": true,
    "chromePath": "/usr/bin/google-chrome",
    "args": ["--no-sandbox", "--disable-setuid-sandbox"]
  },
  "limits": {
    "maxSessions": 10,
    "defaultTimeout": 30000
  }
}
```

## Error Handling

### 常见错误处理

| 错误 | 处理策略 |
|-----|---------|
| `ECONNREFUSED` | 服务未启动，运行 `pinchtab daemon start` |
| `timeout` | 增加 `--wait` 时间，或检查网络 |
| `element not found` | 检查 snap 输出中的引用是否正确 |
| `navigation failed` | URL 可能无效，或网站阻止了请求 |
| `rate limited` | 降低请求频率，添加延迟 |

### 重试机制

```python
import time
import requests

def with_retry(func, max_retries=3, delay=1):
    """带重试的函数包装器"""
    for i in range(max_retries):
        try:
            return func()
        except requests.exceptions.RequestException as e:
            if i == max_retries - 1:
                raise
            print(f"Retry {i+1}/{max_retries}: {e}")
            time.sleep(delay * (i + 1))
```

## Output Requirements

### 必须生成的文件

```
./pinchtab/{task-slug}/
├── report.md              # 执行报告
├── content/               # 提取的文本
│   ├── page-001.txt
│   └── ...
├── screenshots/           # 截图 (如需要)
│   └── ...
├── structured/            # 结构化数据
│   └── data.json
└── logs/                  # 操作日志
    └── session.log
```

### 报告模板

```markdown
# {任务名称} 执行报告

## 执行摘要
- 使用工具: PinchTab
- 处理 URL 数: {count}
- 总耗时: {duration}
- Token 消耗估算: {tokens}

## 会话配置
- 模式: {daemon|server|bridge}
- 会话数: {sessions}
- 并行度: {parallel}

## 提取结果
{结果摘要}

## 性能指标
- 平均提取时间: {avg_time}
- 平均每页 tokens: {avg_tokens}
- 成功率: {success_rate}
```

## Related Skills

### mofa-firecrawl
基于 Firecrawl CLI 的大规模网页抓取。

**区别:**
- mofa-firecrawl: 云端服务，功能全面，适合大规模爬取
- mofa-pinchtab: 本地轻量，token 高效，适合快速提取

**组合使用:**
```
mofa-firecrawl: 发现目标 URL 列表
      ↓
mofa-pinchtab: 高效并行提取内容
      ↓
综合分析报告
```

### mofa-verge-browser
GUI 浏览器沙箱，支持人工接管。

**区别:**
- mofa-verge-browser: 完整 GUI，支持人工介入，适合复杂场景
- mofa-pinchtab: 轻量 HTTP API，token 高效，适合批量提取

**组合使用:**
```
mofa-pinchtab: 批量快速提取
      ↓
mofa-verge-browser: 处理需要 GUI 的复杂页面
      ↓
合并结果
```

### mofa-crawler
Cloudflare Browser Rendering API。

**区别:**
- mofa-crawler: 简单 API 调用，无需自建
- mofa-pinchtab: 本地运行，更灵活，多实例并行

### mofa-research-2.0
深度研究管道。

**组合使用:**
```
mofa-research-2.0: 规划研究任务
      ↓
mofa-pinchtab: 高效提取多个来源内容
      ↓
mofa-research-2.0: 分析整合，生成报告
```
