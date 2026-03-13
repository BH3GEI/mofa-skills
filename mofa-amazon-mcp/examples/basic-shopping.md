# Example: Basic Shopping Session

**User Request**: "买一副无线耳机，预算 200 美元以内"

---

## Phase 1: Intent Parsing

**Analysis:**
- Intent: Purchase
- Product: Wireless headphones
- Budget: $200 USD
- Priority: Price + Quality balance

---

## Phase 2: Search

```
MCP Call: search_products
Query: "wireless headphones under $200"
Max results: 10
```

**Results:**

| # | Product | Price | Rating | Prime |
|---|---------|-------|--------|-------|
| 1 | Sony WH-CH720N | $148 | ★★★★☆ (3,421) | ✓ |
| 2 | Audio-Technica ATH-M50xBT2 | $169 | ★★★★★ (8,932) | ✓ |
| 3 | JBL Tune 760NC | $99 | ★★★★☆ (2,156) | ✓ |
| 4 | Sennheiser HD 450BT | $149 | ★★★★☆ (1,876) | ✓ |
| 5 | Anker Soundcore Q45 | $99 | ★★★★★ (12,433) | ✓ |

---

## Phase 3: User Selection

**User**: "第2个 Audio-Technica 那个"

**Product Details:**
- **Audio-Technica ATH-M50xBT2** - $169.00
- Rating: ★★★★★ (8,932 reviews)
- Features: Studio sound, 50hr battery, Bluetooth 5.0
- In stock, Prime delivery

---

## Phase 4: Budget Check & Confirmation

```
Budget Check:
- Product price: $169.00
- Tax (est): $13.52
- Shipping: FREE (Prime)
- TOTAL: $182.52
- Budget remaining after: $17.48
- Approval threshold: $100 (requires confirmation)
```

**HITL**: "购买总计 $182.52，确认吗？"

**User**: "确认"

---

## Phase 5: Payment & Purchase

```
MCP Call: Fewsats (L402 payment)
Amount: $182.52
Status: Approved

MCP Call: purchase_product
Product: ATH-M50xBT2
Payment: Fewsats token
```

---

## Phase 6: Confirmation

```markdown
## Purchase Complete

| Item | Value |
|------|-------|
| Order ID | 112-3581321-4231113 |
| Product | Audio-Technica ATH-M50xBT2 |
| Total | $182.52 |
| Payment | Fewsats L402 |
| Delivery | Mar 15-16 (Prime) |
| Tracking | Available tomorrow |

**Receipt**: Saved to purchase history
```

---

## Key Points

- Budget-aware search
- Clear presentation of options
- Explicit confirmation for over-threshold purchase
- Full price transparency (tax included)
- Order tracking provided
