# Example: Compare and Buy

**User Request**: "帮我比较几款机械键盘，然后买一个适合写代码的"

---

## Phase 1: Intent Parsing

**Analysis:**
- Intent: Compare → Purchase
- Product: Mechanical keyboards
- Use case: Programming/coding
- Criteria: Tactile feedback, durability, noise level

---

## Phase 2: Search & Compare

```
MCP Call: search_products
Query: "mechanical keyboard programming tactile"
Max results: 15
```

**Filtered Results (Coding-focused):**

| # | Product | Switch | Price | Rating | Noise |
|---|---------|--------|-------|--------|-------|
| 1 | Keychron K8 Pro | Gateron G Pro Brown | $89 | ★★★★★ (2,100) | Medium |
| 2 | HHKB Hybrid Type-S | Topre Silent | $329 | ★★★★★ (1,432) | Quiet |
| 3 | Keychron Q1 Pro | Gateron G Pro Red | $169 | ★★★★☆ (892) | Medium |
| 4 | Logitech MX Mechanical | Tactile Quiet | $169 | ★★★★☆ (3,400) | Quiet |
| 5 | Ducky One 3 | Cherry MX Brown | $119 | ★★★★★ (5,600) | Medium |

---

## Phase 3: Comparison Summary

```markdown
## 编程键盘对比

### 性价比之选: Keychron K8 Pro ($89)
- 热插拔轴体
- Mac/Win 双模
- 蓝牙/有线
- 适合: 预算有限，想要定制

### 静音之选: Logitech MX Mechanical ($169)
- 罗技生态
- 低噪设计
- 智能背光
- 适合: 办公室/共享空间

### 顶级手感: HHKB Hybrid Type-S ($329)
- Topre 静电容轴
- 程序员经典
- 60% 紧凑布局
- 适合: 追求极致，预算充足
```

---

## Phase 4: User Decision

**User**: "Keychron K8 Pro 看起来不错，买那个"

---

## Phase 5: Budget Check

```
Budget Check:
- Price: $89.00
- Tax: $7.12
- Shipping: FREE
- Total: $96.12
- Under approval_threshold ($100): AUTO-APPROVE
```

---

## Phase 6: Purchase

```markdown
## Purchase Complete

| Item | Value |
|------|-------|
| Order ID | 114-7896543-2109876 |
| Product | Keychron K8 Pro |
| Total | $96.12 |
| Delivery | Mar 14 (Prime) |
```

---

## Key Points

- Multiple relevant options presented
- Clear categorization by use case
- Auto-approval for under-threshold purchase
- Quick execution
