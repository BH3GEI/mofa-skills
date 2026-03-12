# Example: Frontend-Friendly API Discovery

**User Query**: "前端能直接调用的 API，免认证 HTTPS"

---

## Phase 1: Parse

**Intent**: Filtered discovery
**Requirements**:
- HTTPS: true (secure)
- CORS: yes (browser can call directly)
- Auth: none (no backend proxy needed)

---

## Phase 2: Search

```python
filters = {
    "https": true,
    "cors": "yes",
    "auth": "none"
}
results = search_apis(filters=filters, limit=15)
```

**Cross-category matches found:**

### Entertainment
- Cat Facts - Daily cat facts
- Dog Facts - Random dog facts
- Studio Ghibli - Ghibli film resources

### Development
- HTTP Cat - Cat pictures for HTTP status codes
- HTTP Dog - Dog pictures for HTTP status codes

### Data
- JSONPlaceholder - Fake data for testing
- UUID Generator - Generate UUIDs

### Geolocation
- IPify - IP address API
- IPinfo - IP geolocation

---

## Phase 3: Enhance

All results get [推荐] badge (no-auth + HTTPS + CORS).

---

## Phase 4: Output

**Frontend-Ready APIs (15 shown of 200+):**

| Category | Name | Description |
|----------|------|-------------|
| Entertainment | Cat Facts | Daily cat facts [推荐] |
| Entertainment | Dog Facts | Random dog facts [推荐] |
| Development | HTTP Cat | HTTP status cats [推荐] |
| Development | JSONPlaceholder | Fake REST API [推荐] |
| Geolocation | IPify | Simple IP API [推荐] |

**Summary:** 200+ APIs are frontend-ready (no-auth + HTTPS + CORS). Popular categories: Entertainment, Development Tools, Test Data.

**Quick Start:** JSONPlaceholder for testing, HTTP Cat for fun error pages.
