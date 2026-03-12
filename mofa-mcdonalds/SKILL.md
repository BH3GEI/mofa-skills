---
name: mofa-mcdonalds
description: "McDonald's China ordering via MCP Server — 点餐、领券、积分兑换、活动日历"
triggers: ["麦当劳", "点餐", "麦麦", "mcd", "McDonald's"]
requires_env: MCD_MCP_TOKEN
always: false
---

# MOFA McDonald's

麦当劳中国 MCP 点餐助手 — 支持外送点餐、优惠券领取、积分兑换、活动查询。

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      MCDONALD'S ORDERING PIPELINE                           │
└─────────────────────────────────────────────────────────────────────────────┘

User Request
    ↓
┌─────────────────────────────────────────────────────────┐
│ Intent Parser                                           │
│ ────────────                                            │
│ • 点餐 → Ordering Module                                │
│ • 领券 → Coupon Module                                  │
│ • 积分 → Points Module                                  │
│ • 活动 → Calendar Module                                │
│ • 营养 → Nutrition Module                               │
└─────────────────────────┬───────────────────────────────┘
                          ↓
        ┌─────────────────┼─────────────────┐
        ↓                 ↓                 ↓
┌───────────────┐ ┌───────────────┐ ┌───────────────┐
│   Ordering    │ │   Coupons     │ │    Points     │
│   Module      │ │   Module      │ │   Module      │
├───────────────┤ ├───────────────┤ ├───────────────┤
│ 1. 获取地址   │ │ 1. 查可领券   │ │ 1. 查积分余额 │
│ 2. 查菜单     │ │ 2. 一键领券   │ │ 2. 积分商品   │
│ 3. 选餐品     │ │ 3. 我的优惠券 │ │ 3. 积分兑换   │
│ 4. 算价格     │ └───────────────┘ └───────────────┘
│ 5. 创建订单   │
│ 6. 支付链接   │
└───────┬───────┘
        ↓
┌───────────────┐
│  MCP Server   │
│  mcp.mcd.cn   │
└───────────────┘
```

## MCP Configuration

### 1. Get MCP Token

Visit https://mcp.mcd.cn to apply:

1. **Login** with phone number
2. Click **控制台** (Console) in top right
3. Click **激活** button to apply for MCP Token
4. Agree to service agreement
5. Copy your MCP Token

### 2. Configure Environment

Create `.env` file:

```bash
cp .env.example .env
```

Edit `.env`:
```
MCD_MCP_TOKEN=your_token_here
```

Or export directly:
```bash
export MCD_MCP_TOKEN="your_token_here"
```

### 3. MCP Client Config

```json
{
  "mcpServers": {
    "mcd-mcp": {
      "type": "streamablehttp",
      "url": "https://mcp.mcd.cn",
      "headers": {
        "Authorization": "Bearer $MCD_MCP_TOKEN"
      }
    }
  }
}
```

## Features

### 1. 点餐 Ordering

完整外卖下单流程：

```
获取配送地址 → 查询门店菜单 → 选择餐品 → 计算价格 → 创建订单 → 支付
```

**Tools:**
| Tool | Purpose |
|------|---------|
| `delivery-query-addresses` | 获取用户配送地址列表 |
| `delivery-create-address` | 新增配送地址 |
| `query-meals` | 查询门店可售餐品 |
| `query-meal-detail` | 查询餐品详情（套餐组成）|
| `calculate-price` | 计算订单价格 |
| `create-order` | 创建外送订单 |
| `query-order` | 查询订单详情/状态 |

**Example:**
```
User: "点个巨无霸套餐送到家里"

→ 1. delivery-query-addresses (获取地址)
→ 2. query-meals (查门店菜单)
→ 3. query-meal-detail (确认套餐内容)
→ 4. calculate-price (计算价格)
→ 5. create-order (创建订单)
→ 6. 返回支付链接
```

### 2. 领券 Coupons

麦麦省优惠券管理：

| Tool | Purpose |
|------|---------|
| `available-coupons` | 查询可领取的优惠券 |
| `auto-bind-coupons` | 一键领取所有可用券 |
| `query-my-coupons` | 查询我的优惠券 |
| `query-store-coupons` | 查询门店可用券 |

**Example:**
```
User: "帮我领券"
→ auto-bind-coupons → 领取成功 3 张券

User: "我有什么优惠券"
→ query-my-coupons → 显示可用券列表
```

### 3. 积分 Points

麦麦商城积分兑换：

| Tool | Purpose |
|------|---------|
| `query-my-account` | 查询积分余额 |
| `mall-points-products` | 积分兑换商品列表 |
| `mall-product-detail` | 商品详情 |
| `mall-create-order` | 积分兑换下单 |

**Example:**
```
User: "我有多少积分"
→ query-my-account → 可用积分: 7592

User: "积分能换什么"
→ mall-points-products → 商品列表
```

### 4. 活动日历 Calendar

| Tool | Purpose |
|------|---------|
| `campaign-calendar` | 查询当月营销活动 |

### 5. 营养信息 Nutrition

| Tool | Purpose |
|------|---------|
| `list-nutrition-foods` | 餐品营养成分（热量、蛋白质等）|

**Example:**
```
User: "巨无霸多少卡路里"
→ list-nutrition-foods → 巨无霸: 540 kcal
```

## Ordering Workflow

### Step 1: 获取配送地址

```bash
# Tool: delivery-query-addresses
# 返回用户保存的地址列表
```

如果没有地址，需要新增：
```bash
# Tool: delivery-create-address
# 参数: city, contactName, phone, address, addressDetail
```

### Step 2: 查询门店菜单

```bash
# Tool: query-meals
# 参数: storeCode, beCode (从地址获取)
```

### Step 3: 查询餐品详情

```bash
# Tool: query-meal-detail
# 参数: code, storeCode, beCode
```

### Step 4: 计算价格

```bash
# Tool: calculate-price
# 参数: storeCode, beCode, items[]
```

### Step 5: 创建订单

```bash
# Tool: create-order
# 参数: storeCode, beCode, items[]
# 返回: orderId, payH5Url (支付链接)
```

## Constraints

- **Rate Limit**: 600 requests/minute
- **MCP Version**: 2025-06-18 or earlier
- **Protocol**: Streamable HTTP only
- **Auth**: Bearer Token required

## Error Handling

| Code | Reason | Solution |
|------|--------|----------|
| 401 | Token invalid/expired | Check MCD_MCP_TOKEN |
| 429 | Rate limit exceeded | Slow down requests |

## Examples

See `examples/` directory:
- `order-workflow.md` - 完整点餐流程
- `coupon-workflow.md` - 领券用券流程
- `points-workflow.md` - 积分兑换流程
