---
name: mofa-verge-browser
description: "基于 Verge Browser 的 GUI 浏览器沙箱自动化，支持真实 Chromium 可视化操作、CDP 自动化、GUI 截图和人工接管。Triggers: verge, verge-browser, GUI browser, visual automation, browser sandbox"
requires_bins: verge-browser
requires_env: VERGE_API_URL
always: false
---

# MOFA Verge Browser

基于 [Verge Browser](https://github.com/zzzgydi/verge-browser) 的 GUI 浏览器沙箱自动化 skill。支持真实可视化的 Chromium 浏览器操作，兼容 Playwright/Puppeteer CDP 协议，提供 GUI 级截图和人工接管能力。

## Onboarding / 开始使用

### 前置要求

1. **安装 Verge Browser CLI**
   ```bash
   npm install -g verge-browser
   ```

2. **部署 Verge Browser 服务** (两种方式)

   **方式 A: Docker Compose 部署** (推荐)
   ```bash
   git clone https://github.com/zzzgydi/verge-browser
   cd verge-browser

   # 启动服务
   docker compose -f deployments/docker-compose.yml up -d
   ```

   **方式 B: 本地开发模式**
   ```bash
   # 需要 Python 3.11+
   pip install -r requirements.txt
   python -m app.main
   ```

3. **配置环境变量**
   ```bash
   # API 服务地址
   export VERGE_API_URL=http://localhost:8000

   # 可选：管理员 Token（如果使用自定义）
   export VERGE_ADMIN_AUTH_TOKEN=dev-admin-token
   ```

4. **验证安装**
   ```bash
   # 检查 CLI
   verge-browser --version

   # 检查服务状态
   curl http://localhost:8000/health
   ```

### 快速开始

```bash
# 1. 创建浏览器沙箱
verge-browser sandbox create --alias my-browser --width 1440 --height 900

# 2. 获取人工接管 URL（在浏览器中打开）
verge-browser sandbox session my-browser

# 3. 截图查看当前状态
verge-browser browser screenshot my-browser --output ./screenshot.png

# 4. 执行自动化操作（通过 CDP 或 actions API）
verge-browser browser actions my-browser --input ./actions.json

# 5. 删除沙箱
verge-browser sandbox delete my-browser
```

### 故障排除

| 问题 | 解决方案 |
|-----|---------|
| `verge-browser: command not found` | 确保 `~/.local/bin` 或 npm 全局路径在 PATH 中 |
| `Connection refused` | 检查服务是否运行 `docker ps`，确认 `VERGE_API_URL` |
| `Sandbox creation failed` | 检查 Docker 是否有足够资源，查看日志 `docker logs verge-api` |
| `Screenshot timeout` | 浏览器可能还在加载，增加等待时间 |
| `CDP connection failed` | 确保沙箱状态为 `running`，检查 WebSocket 端点 |

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      MOFA VERGE BROWSER PIPELINE                            │
└─────────────────────────────────────────────────────────────────────────────┘

Phase 1: SANDBOX (Environment Setup)
────────────────────────────────────
创建/选择沙箱环境
      ↓
┌─────────────────────────────────────────────────────────┐
│  • 创建新沙箱 (sandbox create)                          │
│  • 或使用现有沙箱                                        │
│  • 配置分辨率 (width x height)                          │
│  • 选择运行时 (xvfb_vnc | xpra)                         │
└─────────────────────────┬───────────────────────────────┘
                          ↓
                ┌─────────────────────┐
                │  获取 session URL   │
                │  (noVNC / Xpra)     │
                └─────────────────────┘

Phase 2: AUTOMATION (Browser Control)
─────────────────────────────────────
                         ┌─────────────┐
                    ┌────┤  CDP 自动化  │
                    │    │  Playwright │
                    │    └─────────────┘
              选择  │    ┌─────────────┐
              方式  ├────┤  Actions API│
                    │    │  JSON 指令  │
                    │    └─────────────┘
                    │    ┌─────────────┐
                    └────┤  GUI 截图   │
                         │  视觉反馈   │
                         └─────────────┘

自动化方式:
- CDP: Playwright/Puppeteer 通过 WebSocket 连接
- Actions: JSON 格式的 GUI 操作序列
- Screenshot: 获取浏览器窗口截图

Phase 3: INTERACTION (Human-in-the-Loop)
────────────────────────────────────────
用户需要时介入:
      ↓
┌─────────────────────────────────────────────────────────┐
│  • 获取人工接管 URL (noVNC/Xpra)                        │
│  • 实时查看/操作浏览器                                   │
│  • 处理验证码、复杂登录等                                │
│  • 与自动化流程无缝切换                                  │
└─────────────────────────┬───────────────────────────────┘
                          ↓
                    ┌─────────────────────┐
                    │  继续自动化流程     │
                    └─────────────────────┘

Phase 4: DATA EXTRACTION
────────────────────────
提取获取的数据:
┌─────────────────────────────────────────────────────────┐
│  • 页面内容 (HTML/Markdown)                             │
│  • 截图证据                                             │
│  • 下载的文件                                           │
│  • 操作日志                                             │
└─────────────────────────┬───────────────────────────────┘
                          ↓
                    Processed Results

Phase 5: CLEANUP
────────────────
清理资源:
- 暂停沙箱 (保留状态)
- 或删除沙箱 (释放资源)
```

## Verge Browser 能力映射

| 能力 | CLI/API | 用途 |
|-----|---------|------|
| **Sandbox 管理** | `sandbox create/delete/pause/resume` | 浏览器环境生命周期 |
| **Session 接入** | `sandbox session` | 人工接管 (noVNC/Xpra) |
| **CDP 自动化** | WebSocket `/cdp` | Playwright/Puppeteer |
| **GUI 截图** | `browser screenshot` | 可视化验证 |
| **Actions** | `browser actions` | JSON 格式 GUI 操作 |
| **文件操作** | `/workspace/*` | 与沙箱共享文件 |

## Phase 1: Sandbox (沙箱管理)

### 创建沙箱

```bash
# 基础创建
verge-browser sandbox create --alias my-browser

# 指定分辨率
verge-browser sandbox create \
  --alias my-browser \
  --width 1920 \
  --height 1080

# 指定运行时类型
verge-browser sandbox create \
  --alias my-browser \
  --runtime xpra  # xvfb_vnc | xpra

# 输出 JSON 格式
verge-browser sandbox create --alias my-browser --json
```

**返回信息:**
```json
{
  "id": "sandbox-uuid",
  "alias": "my-browser",
  "status": "creating",
  "websocket_url": "ws://localhost:8000/sandbox/{id}/cdp",
  "vnc_url": "http://localhost:8000/sandbox/{id}/vnc",
  "workspace": "/workspace"
}
```

### 沙箱生命周期管理

```bash
# 查看沙箱列表
verge-browser sandbox list

# 查看沙箱状态
verge-browser sandbox status my-browser

# 暂停沙箱 (保留状态)
verge-browser sandbox pause my-browser

# 恢复沙箱
verge-browser sandbox resume my-browser

# 删除沙箱
verge-browser sandbox delete my-browser
```

### 获取 Session URL (人工接管)

```bash
# 获取 noVNC/Xpra 访问链接
verge-browser sandbox session my-browser

# 返回可访问的 URL，在浏览器中打开即可实时操作
```

## Phase 2: Automation (自动化操作)

### 方式 1: CDP + Playwright

通过 WebSocket 连接使用 Playwright 控制浏览器。

```python
import asyncio
from playwright.async_api import async_playwright

async def main():
    # 先创建沙箱获取 ws_endpoint
    # ws_endpoint = "ws://localhost:8000/sandbox/{id}/cdp"

    async with async_playwright() as p:
        browser = await p.chromium.connect_over_cdp(
            ws_endpoint="ws://localhost:8000/sandbox/my-browser/cdp"
        )
        context = await browser.new_context()
        page = await context.new_page()

        # 正常 Playwright 操作
        await page.goto("https://example.com")
        title = await page.title()
        print(f"Title: {title}")

        # 截图 (页面级)
        await page.screenshot(path="page.png")

        await browser.close()

asyncio.run(main())
```

### 方式 2: Actions API

通过 JSON 指令执行 GUI 操作。

```bash
# 创建 actions.json
{
  "actions": [
    {"type": "goto", "url": "https://example.com"},
    {"type": "click", "selector": "#button"},
    {"type": "type", "selector": "#input", "text": "hello"},
    {"type": "wait", "ms": 2000},
    {"type": "screenshot", "output": "/workspace/step1.png"}
  ]
}

# 执行
verge-browser browser actions my-browser --input ./actions.json
```

**Actions 类型:**

| Action | 参数 | 说明 |
|--------|------|------|
| `goto` | `url` | 导航到 URL |
| `click` | `selector`, `x`, `y` | 点击元素或坐标 |
| `type` | `selector`, `text` | 输入文本 |
| `scroll` | `selector`, `direction`, `amount` | 滚动 |
| `wait` | `ms`, `selector` | 等待 |
| `screenshot` | `output` | 截图 |
| `download` | `url`, `output` | 下载文件 |

### 方式 3: GUI 截图

```bash
# 基础截图
verge-browser browser screenshot my-browser --output ./screenshot.png

# 指定区域 (相对于浏览器窗口)
verge-browser browser screenshot my-browser \
  --x 100 --y 100 --width 800 --height 600 \
  --output ./region.png

# 完整窗口 (包含浏览器 UI)
verge-browser browser screenshot my-browser \
  --full-window \
  --output ./full.png
```

## Phase 3: Human-in-the-Loop (人工接管)

### 何时需要人工介入

- 复杂的验证码识别
- 多因素认证 (MFA)
- 敏感操作确认
- 自动化失败时的调试
- 需要人类判断的决策点

### 接管流程

```bash
# 1. 获取接管 URL
verge-browser sandbox session my-browser
# 输出: http://localhost:8000/sandbox/{id}/vnc?ticket=xxx

# 2. 在浏览器中打开该 URL
# 3. 完成人工操作
# 4. 关闭浏览器或返回自动化流程
```

### 混合模式示例

```python
async def hybrid_automation():
    # 自动化部分
    await page.goto("https://example.com/login")
    await page.fill("#username", "user@example.com")
    await page.fill("#password", "password")
    await page.click("#login-btn")

    # 检测到验证码，暂停等待人工
    print("请完成验证码...")
    print(f"接管 URL: {get_session_url('my-browser')}")
    input("完成后按回车继续...")

    # 继续自动化
    await page.goto("https://example.com/dashboard")
    # ...
```

## Phase 4: Data Extraction (数据提取)

### 文件共享 (/workspace)

沙箱内的 `/workspace` 目录与主机共享，可用于:
- 上传待处理的文件
- 下载抓取的数据
- 保存截图证据
- 共享配置文件

```bash
# 上传文件到沙箱
curl -X POST "http://localhost:8000/sandbox/{id}/workspace/upload" \
  -F "file=@./data.csv"

# 下载文件
wget "http://localhost:8000/sandbox/{id}/workspace/download/output.json"
```

### 提取页面内容

```python
# 通过 Playwright 提取
content = await page.content()
text = await page.inner_text("body")

# 结构化数据
items = await page.query_selector_all(".item")
data = []
for item in items:
    title = await item.query_selector_eval(".title", "el => el.textContent")
    price = await item.query_selector_eval(".price", "el => el.textContent")
    data.append({"title": title, "price": price})
```

## Usage Examples

### 示例 1: 自动化 + 人工接管混合流程

```
用户: "帮我登录某网站并抓取数据，有验证码"

执行:
1. verge-browser sandbox create --alias task-001
2. Playwright: 打开登录页，填写用户名密码
3. 检测到验证码:
   - 获取 session URL
   - 提示用户完成验证码
   - 等待用户确认
4. Playwright: 继续抓取数据
5. 保存结果到 /workspace
6. verge-browser sandbox delete task-001
```

### 示例 2: 可视化监控任务

```
用户: "监控某页面变化，需要可视化确认"

执行:
1. verge-browser sandbox create --alias monitor
2. 每小时:
   - verge-browser browser screenshot monitor --output ./$(date +%Y%m%d-%H%M).png
   - Playwright: 检查特定元素
3. 对比截图发现变化时通知用户
4. 提供 session URL 供用户查看实时状态
```

### 示例 3: 批量操作多个账号

```
用户: "需要依次登录 5 个账号执行相同操作"

执行:
1. for i in {1..5}:
   - verge-browser sandbox create --alias account-$i
   - 登录账号 $i
   - 执行操作
   - 截图保存证据
   - verge-browser sandbox delete account-$i
   - 间隔 30 秒防检测
```

### 示例 4: GUI 自动化测试

```
用户: "测试网页在不同分辨率下的显示效果"

执行:
1. 创建多个沙箱:
   - verge-browser sandbox create --alias mobile --width 375 --height 667
   - verge-browser sandbox create --alias tablet --width 768 --height 1024
   - verge-browser sandbox create --alias desktop --width 1920 --height 1080

2. 每个沙箱:
   - 打开目标页面
   - verge-browser browser screenshot {alias} --output ./{alias}.png

3. 对比截图，生成报告
4. 清理沙箱
```

## Configuration

### 环境变量

```bash
# API 服务地址
export VERGE_API_URL=http://localhost:8000

# 管理员 Token
export VERGE_ADMIN_AUTH_TOKEN=dev-admin-token

# Ticket 密钥 (用于 session URL 签名)
export VERGE_TICKET_SECRET=your-secret-key

# 沙箱存储路径
export VERGE_SANDBOX_BASE_DIR=/var/lib/verge-sandboxes
```

### Docker Compose 配置

```yaml
version: '3.8'
services:
  api:
    image: verge-browser-api
    environment:
      - VERGE_ADMIN_AUTH_TOKEN=dev-admin-token
      - VERGE_SANDBOX_BASE_DIR=/sandboxes
    ports:
      - "8000:8000"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - ./sandboxes:/sandboxes

  runtime-xvfb:
    image: verge-browser-runtime:xvfb
    # 用于 xvfb_vnc 运行时

  runtime-xpra:
    image: verge-browser-runtime:xpra
    # 用于 xpra 运行时
```

## Error Handling

### 常见错误处理

| 错误 | 处理策略 |
|-----|---------|
| `Sandbox not found` | 检查 alias 是否正确，使用 `sandbox list` 查看 |
| `Sandbox is paused` | 先执行 `sandbox resume` |
| `CDP connection timeout` | 沙箱可能还在启动，等待 5-10 秒重试 |
| `Screenshot failed` | 浏览器可能已崩溃，检查沙箱状态 |
| `Action execution failed` | 元素可能不存在，添加等待或重试逻辑 |

### 重试机制

```python
import asyncio
from playwright.async_api import async_playwright, TimeoutError as PlaywrightTimeout

async def with_retry(action, max_retries=3, delay=2):
    for i in range(max_retries):
        try:
            return await action()
        except PlaywrightTimeout:
            if i == max_retries - 1:
                raise
            print(f"Retry {i+1}/{max_retries}...")
            await asyncio.sleep(delay)
```

## Output Requirements

### 必须生成的文件

1. `./verge/{task-slug}/report.md` - 执行报告
2. `./verge/{task-slug}/screenshots/` - 截图证据
3. `./verge/{task-slug}/data/` - 提取的数据
4. `./verge/{task-slug}/logs/` - 操作日志

### 报告模板

```markdown
# {任务名称} 执行报告

## 执行摘要
- 沙箱 ID: {sandbox_id}
- 执行时间: {duration}
- 使用模式: {automation-only | hybrid}
- 人工介入: {次数}

## 操作记录
1. [时间] 创建沙箱
2. [时间] 访问 {url}
3. [时间] 截图: screenshot-001.png
4. [时间] 人工介入: 完成验证码
5. ...

## 数据提取
{提取的数据摘要}

## 截图证据
- screenshot-001.png: 登录页
- screenshot-002.png: 登录后首页
- ...

## 问题与解决
{遇到的问题及解决方案}
```

## Related Skills

### mofa-firecrawl
基于 Firecrawl CLI 的内容爬取，支持大规模网站爬取和搜索。

**区别:**
- mofa-firecrawl: 云端服务，无需自建，适合大规模爬取
- mofa-verge-browser: 本地 GUI 沙箱，可视化操作，适合需要人工介入的场景

**组合使用:**
```
mofa-firecrawl: 发现目标 URL 列表
      ↓
mofa-verge-browser: 逐个访问，可视化验证，人工处理复杂情况
      ↓
综合报告
```

### mofa-crawler
基于 Cloudflare Browser Rendering 的简单网页抓取。

**区别:**
- mofa-crawler: 简单 API 调用，快速抓取，无 GUI
- mofa-verge-browser: 完整 GUI 浏览器，支持复杂交互和人工接管

### mofa-crawlee-python
基于 Crawlee-Python 的爬虫框架。

**组合使用:**
- mofa-verge-browser: 处理需要 GUI 的复杂页面
- mofa-crawlee-python: 处理大规模结构化爬取

### mofa-research-2.0
深度研究管道。

**组合使用:**
```
mofa-research-2.0: 规划研究任务
      ↓
mofa-verge-browser: 可视化访问目标网站，处理复杂认证
      ↓
mofa-research-2.0: 分析提取的数据，生成报告
```
