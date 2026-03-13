---
name: mofa-amazon-mcp
description: "Amazon shopping via MCP with L402 payments. Search and buy products through AI assistant. Triggers: amazon, buy, purchase, shopping, product search, 购买, 亚马逊"
requires_bins: uv, uvx
requires_env: FEWSATS_API_KEY
always: false
---

# MOFA Amazon MCP

通过 [Amazon MCP](https://github.com/Fewsats/amazon-mcp) 和 [Fewsats](https://fewsats.com) 支付基础设施，实现 AI 助手的 Amazon 商品搜索与购买。

## 架构

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        MOFA AMAZON MCP PIPELINE                             │
└─────────────────────────────────────────────────────────────────────────────┘

Phase 1: 搜索商品
─────────────────
用户请求 → 提取搜索关键词
                          ↓
              ┌─────────────────────┐
              │  Amazon MCP Server  │
              │  - search_products  │
              │  - 返回商品列表      │
              └─────────────────────┘
                          ↓
              ┌─────────────────────┐
              │  展示结果给用户      │
              │  - 商品名称/价格     │
              │  - 评分/评论数       │
              │  - 商品详情链接      │
              └─────────────────────┘

Phase 2: 确认购买 (Human-in-the-loop)
───────────────────────────────────────
用户确认 → 展示商品详情
                          ↓
              ┌─────────────────────┐
              │  用户确认价格/运费   │
              │  预算检查           │
              └─────────────────────┘
                          ↓
              ┌─────────────────────┐
              │  Fewsats MCP        │
              │  L402 支付流程      │
              └─────────────────────┘

Phase 3: 完成购买
─────────────────
                          ↓
              ┌─────────────────────┐
              │  Amazon MCP         │
              │  - purchase_product │
              │  - 下单/支付        │
              └─────────────────────┘
                          ↓
              ┌─────────────────────┐
              │  返回购买凭证        │
              │  - 订单号           │
              │  - 预计送达时间      │
              └─────────────────────┘
```

## ⚠️ Onboarding 流程（首次使用必须执行）

**CRITICAL**: 当用户首次使用 Amazon 购物功能时，**必须**先完成以下配置。检测到 `FEWSATS_API_KEY` 未设置时，**立即**引导用户 onboarding，不要直接执行搜索/购买。

### 自动检测流程

```
用户: "帮我买无线耳机"
    ↓
AI 检测: FEWSATS_API_KEY 存在?
    ↓ NO
引导 Onboarding:
┌─────────────────────────────────────────┐
│  🔧 首次使用 Amazon 购物需要配置         │
│                                         │
│  1. 注册 Fewsats 账号                   │
│     👉 https://fewsats.com              │
│                                         │
│  2. 获取 API Key                        │
│     Dashboard → API Keys → Create       │
│                                         │
│  3. 选择配置方式：                       │
│                                         │
│     方式 A: 环境变量（推荐）             │
│     export FEWSATS_API_KEY="xxx"        │
│                                         │
│     方式 B: Claude Desktop 配置         │
│     编辑 ~/Library/Application Support/ │
│     Claude/claude_desktop_config.json   │
│                                         │
│  配置完成后告诉我，我就能帮你购物了！     │
└─────────────────────────────────────────┘
```

### 详细配置步骤

#### 1. 安装 UV

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

#### 2. 注册 Fewsats 并获取 API Key

1. 访问 [Fewsats](https://fewsats.com) 注册账号
2. 完成邮箱验证
3. 进入 Dashboard → API Keys → "Create New Key"
4. **复制 Key**（只显示一次！）

#### 3. 配置方式（二选一）

**方式 A: 环境变量（全局可用）**

```bash
# 添加到 ~/.zshrc 或 ~/.bashrc
export FEWSATS_API_KEY="fs_live_xxxxxxxxxx"

# 立即生效
source ~/.zshrc
```

**方式 B: Claude Desktop 配置（仅 Claude 可用）**

编辑 `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "Amazon": {
      "command": "uvx",
      "args": ["amazon-mcp"]
    },
    "Fewsats": {
      "command": "env",
      "args": [
        "FEWSATS_API_KEY=fs_live_xxxxxxxxxx",
        "uvx",
        "fewsats-mcp"
      ]
    }
  }
}
```

#### 4. 验证配置

```bash
# 测试环境变量
echo $FEWSATS_API_KEY

# 测试 Amazon MCP
uvx amazon-mcp --help
```

### Onboarding 完成检查清单

| 检查项 | 状态 |
|--------|------|
| UV 已安装 | ☐ |
| Fewsats 账号已注册 | ☐ |
| API Key 已获取 | ☐ |
| API Key 已配置（环境变量或 Claude Desktop） | ☐ |
| 配置已验证 | ☐ |

**只有所有检查项完成，才能开始购物！**

## 使用流程

### 场景 1: 搜索商品

```
用户: 帮我找一下无线耳机

AI: 正在搜索 Amazon 上的无线耳机...

┌─────────────────────────────────────────┐
│  搜索结果 (Top 5)                        │
├─────────────────────────────────────────┤
│ 1. Sony WH-1000XM5 - $348.00            │
│    ★★★★★ (12,345 评论)                   │
│    降噪耳机，30小时续航                   │
│                                         │
│ 2. AirPods Pro 2 - $249.00              │
│    ★★★★★ (89,234 评论)                   │
│    Apple 芯片，主动降噪                   │
│    ...                                  │
└─────────────────────────────────────────┘

需要查看某个商品的详情或购买吗？
```

### 场景 2: 购买商品

```
用户: 购买第1个 Sony 耳机

AI: 确认购买信息:

┌─────────────────────────────────────────┐
│  商品: Sony WH-1000XM5                  │
│  价格: $348.00                          │
│  运费: 免费 (Prime)                      │
│  预计送达: 2天内                         │
│  预算限额: $500/月                       │
├─────────────────────────────────────────┤
│  [确认购买]  [取消]                      │
└─────────────────────────────────────────┘

确认后将通过 Fewsats 进行 L402 支付...
```

## MCP 工具调用

### 搜索商品

```json
{
  "tool": "search_products",
  "server": "Amazon",
  "params": {
    "query": "wireless headphones",
    "max_results": 10
  }
}
```

### 获取商品详情

```json
{
  "tool": "get_product_details",
  "server": "Amazon",
  "params": {
    "product_id": "B09XS7JWHH"
  }
}
```

### 购买商品

```json
{
  "tool": "purchase_product",
  "server": "Amazon",
  "params": {
    "product_id": "B09XS7JWHH",
    "quantity": 1
  }
}
```

## 支付流程 (L402)

```
购买请求
    ↓
Amazon MCP → 生成 L402 发票
    ↓
Fewsats MCP → 检查预算/限额
    ↓
用户确认 (如超过阈值)
    ↓
Fewsats MCP → 执行支付
    ↓
Amazon MCP → 完成订单
    ↓
返回订单确认
```

## 安全与预算控制

Fewsats 提供多层安全控制:

| 功能 | 说明 |
|------|------|
| 预算限额 | 设置月度/单次购买上限 |
| 审批阈值 | 超过金额需手动确认 |
| 购买历史 | 完整的交易记录 |
| 手动审核 | 可选的每笔交易确认 |

## 配置说明

见 `config.toml` 可配置:

- `max_search_results`: 默认搜索结果数
- `budget_limit`: 预算限额
- `approval_threshold`: 需要确认的最低金额
- `default_shipping_address`: 默认配送地址 (可选)

## 故障排除

### uvx 命令未找到

```bash
# 确保 UV 已正确安装并添加到 PATH
source ~/.bashrc  # 或 ~/.zshrc
which uvx
```

### Fewsats API Key 无效

- 检查环境变量是否正确设置
- 在 Fewsats Dashboard 确认 API Key 状态
- 尝试重新生成 API Key

### 搜索无结果

- 尝试使用英文关键词
- 检查网络连接
- 确认 Amazon MCP 服务器正常运行

## 相关 Skill

- **mofa-research-2.0**: 商品调研与比价
- **mofa-xhs**: 小红书商品评价参考
