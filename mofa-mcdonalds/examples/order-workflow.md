# Example: Complete Ordering Workflow

**User**: "我想点个巨无霸套餐送到家里"

---

## Step 1: Get Delivery Address

**Tool:** `delivery_query_addresses`

**Response:**
```json
{
  "addresses": [
    {
      "addressId": "1",
      "contactName": "张三",
      "phone": "152****6666",
      "fullAddress": "北京市朝阳区xxx小区 1栋2单元301",
      "storeCode": "12345",
      "storeName": "朝阳门店",
      "beCode": "12345"
    }
  ]
}
```

**Agent:** "找到您的地址：北京市朝阳区xxx小区，使用朝阳门店为您配送。"

---

## Step 2: Query Menu

**Tool:** `query_meals`
- storeCode: "12345"
- beCode: "12345"

**Response:** Menu categories with meals

---

## Step 3: Find 巨无霸套餐

Search in menu for "巨无霸" or similar items.

**Found:**
- 巨无霸套餐 (code: "900001")

---

## Step 4: Query Meal Detail (Optional)

**Tool:** `query_meal_detail`
- code: "900001"
- storeCode: "12345"
- beCode: "12345"

**Response:**
```json
{
  "code": "900001",
  "price": "45",
  "rounds": [
    { "name": "汉堡", "choices": [{ "name": "巨无霸" }] },
    { "name": "饮料", "choices": [{ "name": "中可乐" }] },
    { "name": "小食", "choices": [{ "name": "中薯条" }] }
  ]
}
```

---

## Step 5: Check Available Coupons

**Tool:** `query_store_coupons`
- storeCode: "12345"
- beCode: "12345"

**Found:** "外送满30减5券"

---

## Step 6: Calculate Price

**Tool:** `calculate_price`
```json
{
  "storeCode": "12345",
  "beCode": "12345",
  "items": [
    {
      "productCode": "900001",
      "quantity": 1,
      "couponId": "coupon_xxx",
      "couponCode": "DISCOUNT5"
    }
  ]
}
```

**Response:**
```json
{
  "productPrice": 4500,
  "deliveryPrice": 600,
  "discount": 500,
  "price": 4600
}
```

**Agent:** "订单金额：¥45，配送费：¥6，优惠：-¥5，应付：¥46"

---

## Step 7: Create Order

**Tool:** `create_order`
```json
{
  "storeCode": "12345",
  "beCode": "12345",
  "items": [
    {
      "productCode": "900001",
      "quantity": 1,
      "couponId": "coupon_xxx",
      "couponCode": "DISCOUNT5"
    }
  ]
}
```

**Response:**
```json
{
  "orderId": "1030938730000733964700499858",
  "payH5Url": "https://m.mcd.cn/mcp/scanToPay?orderId=...",
  "orderDetail": {
    "orderStatus": "待支付",
    "totalAmount": "46",
    "deliveryInfo": { ... }
  }
}
```

---

## Final Output

```
✅ 订单创建成功！

订单号: 1030938730000733964700499858
订单状态: 待支付

商品:
• 巨无霸套餐 x1 - ¥45
  - 巨无霸 x1
  - 中可乐 x1
  - 中薯条 x1

配送费: ¥6
优惠: -¥5
应付总额: ¥46

配送地址: 北京市朝阳区xxx小区 1栋2单元301
联系电话: 152****6666

📱 点击支付: https://m.mcd.cn/mcp/scanToPay?orderId=...
```
