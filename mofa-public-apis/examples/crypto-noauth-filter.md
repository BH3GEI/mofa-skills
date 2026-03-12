# Example: Cryptocurrency APIs (No Auth Required)

**User Query**: "加密货币 API，最好免认证"

---

## Phase 1: Parse

**Intent**: Filtered search
**Category**: Cryptocurrency
**Filter**: auth="none"

---

## Phase 2: Search

```python
filters = {
    "category": "Cryptocurrency",
    "auth": "none"
}
results = search_apis(filters=filters)
```

**Matching APIs (5 found):**

| Name | Description | HTTPS | CORS |
|------|-------------|-------|------|
| CoinGecko | Cryptocurrency data | Yes | Yes |
| Coinpaprika | Cryptocurrency data | Yes | Yes |
| Blockchair | Blockchain data | Yes | Yes |
| BraveNewCoin | Crypto assets | Yes | No |

---

## Phase 3: Enhance

**Badges Applied:**
- CoinGecko: [推荐] [免认证]
- Coinpaprika: [推荐] [免认证]
- Blockchair: [推荐] [免认证]
- BraveNewCoin: [免认证]

---

## Phase 4: Output

**Results:**

| Badge | Name | Description | HTTPS | CORS |
|-------|------|-------------|-------|------|
| [推荐] | CoinGecko | Cryptocurrency data | Yes | Yes |
| [推荐] | Coinpaprika | Cryptocurrency data | Yes | Yes |
| [推荐] | Blockchair | Blockchain data | Yes | Yes |
| [免认证] | BraveNewCoin | Crypto assets | Yes | No |

**Summary:** Found 4 no-auth cryptocurrency APIs. 3 are fully [推荐] (HTTPS + CORS ready).

**Top Pick:** CoinGecko - most comprehensive, no signup, generous rate limits.
