# Example: Weather API Search

**User Query**: "找个天气 API"

---

## Phase 1: Parse

**Intent**: Discovery (user wants options)
**Keywords**: weather
**No explicit filters**

---

## Phase 2: Search

```python
# Search strategy: fuzzy match on name + description
query = "weather"
matches = search_apis(query)
```

**Matching APIs (10 found):**

| Name | Description | Auth | HTTPS | CORS | Score |
|------|-------------|------|-------|------|-------|
| OpenWeatherMap | Weather data | apiKey | Yes | Yes | 0.95 |
| wttr.in | Weather in terminal | No | Yes | No | 0.90 |
| WeatherAPI | Weather data | apiKey | Yes | Yes | 0.85 |
| MetaWeather | Weather data | No | No | Yes | 0.80 |
| AccuWeather | Weather data | apiKey | Yes | No | 0.75 |

---

## Phase 3: Enhance

**Badges Applied:**
- OpenWeatherMap: [推荐] (apiKey, HTTPS, CORS)
- wttr.in: [免认证] (no auth, HTTPS)
- WeatherAPI: [前端友好] (HTTPS, CORS)
- MetaWeather: [免认证] (no auth, CORS)

---

## Phase 4: Output

**Results:**

| Badge | Name | Description | Auth | HTTPS | CORS |
|-------|------|-------------|------|-------|------|
| [推荐] | OpenWeatherMap | Weather data | apiKey | Yes | Yes |
| [免认证] | wttr.in | Weather in terminal | No | Yes | No |
| [前端友好] | WeatherAPI | Weather data | apiKey | Yes | Yes |
| [免认证] | MetaWeather | Weather data | No | No | Yes |

**Summary:** Found 10 weather APIs. 2 are [免认证] (no signup needed), 3 support full [前端友好] stack (HTTPS + CORS).

**Recommendations:**
- **Quick start**: wttr.in - no signup, simple API
- **Production**: OpenWeatherMap - reliable, comprehensive
