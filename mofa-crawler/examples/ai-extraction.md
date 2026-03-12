# Example: AI Structured Extraction

## Scenario

**User**: "爬取电商网站，提取所有商品名称和价格"

## Config

```json
{
  "url": "https://example-shop.com/products",
  "limit": 100,
  "formats": ["json"],
  "render": true,
  "ai_prompt": "Extract product name, price, original price if discounted, rating, and availability status"
}
```

## API Call

```bash
curl -X POST "https://api.cloudflare.com/client/v4/accounts/{id}/browser-rendering/crawl" \
  -H "Authorization: Bearer {token}" \
  -d '{
    "url": "https://example-shop.com/products",
    "limit": 100,
    "formats": ["json"],
    "render": true,
    "ai": {
      "prompt": "Extract product name, price, original price if discounted, rating, and availability"
    }
  }'
```

## Result

```json
{
  "pages": [
    {
      "url": "https://example-shop.com/products/1",
      "extractedData": {
        "products": [
          {
            "name": "Wireless Headphones",
            "price": "$79.99",
            "originalPrice": "$99.99",
            "rating": 4.5,
            "available": true
          }
        ]
      }
    }
  ]
}
```

## Post-Processing

```bash
# Convert to CSV for analysis
jq -r '.pages[].extractedData.products[] | [.name, .price, .rating] | @csv' result.json > products.csv
```
