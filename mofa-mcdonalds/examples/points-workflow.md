# Example: Points Workflow

## Scenario 1: Check Points Balance

**User**: "我有多少积分"

**Tool:** `query_my_account`

**Response:**
```json
{
  "availablePoint": "7592",
  "accumulativePoint": "141760",
  "frozenPoint": "30",
  "currentMouthExpirePoint": "0",
  "nextMouthExpirePoint": "200"
}
```

**Agent Output:**
```
💰 我的积分

可用积分: 7,592 ⭐
累计积分: 141,760
冻结积分: 30

⚠️ 下个月有 200 积分即将过期

🎁 积分商城有 15 件商品可兑换
说"积分能换什么"查看
```

---

## Scenario 2: Browse Points Products

**User**: "积分能换什么"

**Tool:** `mall_points_products`

**Response:**
```json
{
  "products": [
    {
      "spuName": "中杯拿铁/美式 500积分",
      "spuId": 542,
      "skuId": 10997,
      "point": "500",
      "spuImage": "https://img.mcd.cn/gallery/xxx.png"
    },
    {
      "spuName": "麦旋风 800积分",
      "spuId": 543,
      "skuId": 10998,
      "point": "800",
      "spuImage": "https://img.mcd.cn/gallery/xxx.png"
    }
  ]
}
```

**Agent Output:**
```
🎁 积分兑换商品 (15件)

1. 【中杯拿铁/美式】
   积分: 500 ⭐
   有效期: 2026-02-02 至 2026-04-30
   [图片]

2. 【麦旋风】
   积分: 800 ⭐
   有效期: 2026-02-02 至 2026-04-30
   [图片]

💡 说"兑换第1个"或商品名兑换
   你当前有 7,592 积分，可兑换任意商品
```

---

## Scenario 3: Exchange Product

**User**: "兑换第一个"

**Tool:** `mall_create_order`
- skuId: 10997
- count: 1

**Response:**
```json
{
  "orderId": "ECS1202144786604392448",
  "coupons": [
    {
      "couponId": "2BCEFFB7E1CEA6BD32A43C45A1CE80B3",
      "couponCode": "MCDD6E08N9100KC050F087",
      "orderItemStatus": 1
    }
  ],
  "orderStatus": 30,
  "status": 1
}
```

**Agent Output:**
```
🎉 兑换成功！

订单号: ECS1202144786604392448
商品: 中杯拿铁/美式
消耗积分: 500 ⭐
剩余积分: 7,092 ⭐

券码: MCDD6E08N9100KC050F087
状态: 已发放

📅 有效期: 2026-02-02 至 2026-04-30
💡 可在"我的优惠券"中查看
```

---

## Scenario 4: Product Detail

**User**: "看看拿铁券详情"

**Tool:** `mall_product_detail`
- spuId: 542

**Response:**
```json
{
  "spuName": "中杯拿铁/美式 500积分",
  "points": "500",
  "images": ["https://img.mcd.cn/gallery/xxx.png"],
  "detail": "中杯拿铁或美式咖啡任选其一",
  "upDate": "2026-02-02 00:00:00",
  "downDate": "2026-04-30 23:59:59"
}
```

**Agent Output:**
```
🎁 积分商品详情

【中杯拿铁/美式 500积分】

积分: 500 ⭐
有效期: 2026-02-02 至 2026-04-30

详情:
中杯拿铁或美式咖啡任选其一

[图片]

💡 说"兑换这个"立即兑换
```
