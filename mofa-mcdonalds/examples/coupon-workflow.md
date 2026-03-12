# Example: Coupon Workflow

## Scenario 1: Auto-Claim Coupons

**User**: "帮我领券"

**Tool:** `auto_bind_coupons`

**Response:**
```json
{
  "total": 3,
  "success": 3,
  "failed": 0,
  "coupons": [
    {
      "name": "9.9元麦乐鸡",
      "couponId": "xxx1",
      "couponCode": "MCD..."
    },
    {
      "name": "11.9元麦辣鸡翅",
      "couponId": "xxx2",
      "couponCode": "MCD..."
    },
    {
      "name": "满30减5",
      "couponId": "xxx3",
      "couponCode": "MCD..."
    }
  ]
}
```

**Agent Output:**
```
🎉 领券成功！

总计: 3张优惠券
成功: 3张 ✅

成功领取:
• 9.9元麦乐鸡
• 11.9元麦辣鸡翅
• 满30减5

说"我有什么券"查看全部优惠券
```

---

## Scenario 2: Check My Coupons

**User**: "我有什么优惠券"

**Tool:** `query_my_coupons`

**Response:**
```json
{
  "coupons": [
    {
      "title": "9.9元麦乐鸡",
      "price": "9.9",
      "validPeriod": "2025-12-01 00:00-2026-01-31 23:59",
      "restrictions": ["到店专用"]
    },
    {
      "title": "满30减5",
      "price": "5",
      "validPeriod": "2025-12-01 00:00-2025-12-31 23:59",
      "restrictions": ["外送专用"]
    }
  ]
}
```

**Agent Output:**
```
🎫 我的优惠券 (2张可用)

## 9.9元麦乐鸡
• 优惠: ¥9.9 (原价¥14)
• 有效期: 2025-12-01 至 2026-01-31
• 使用限制: 到店专用
• [图片]

## 满30减5
• 优惠: ¥5
• 有效期: 2025-12-01 至 2025-12-31
• 使用限制: 外送专用
• [图片]

💡 外送点餐时可自动使用符合条件的优惠券
```

---

## Scenario 3: Check Available Coupons

**User**: "有什么券可以领"

**Tool:** `available_coupons`

**Response:** Shows list of claimable coupons

**Agent Output:**
```
🎫 可领取优惠券 (5张)

1. 【9.9元麦乐鸡】
   状态: 未领取
   [图片]

2. 【11.9元麦辣鸡翅】
   状态: 未领取
   [图片]

3. 【北非蛋风味麦满分】
   状态: 不可领取 (已领完)
   [图片]

💡 说"一键领券"领取所有可领的券
```
