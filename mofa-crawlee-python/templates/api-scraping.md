# Template: API Scraping (HttpCrawler)

Use this template for scraping REST APIs and JSON endpoints.

## When to Use
- REST APIs
- JSON endpoints
- Fast data retrieval
- No HTML parsing needed

## Code Template

```python
import asyncio
from crawlee.http_crawler import HttpCrawler, HttpCrawlingContext

async def main():
    crawler = HttpCrawler(
        max_requests_per_crawl={{MAX_REQUESTS}},
        max_requests_per_second={{MAX_RPS}},
        request_handler_timeout={{TIMEOUT}},
    )

    @crawler.router.default_handler
    async def handler(context: HttpCrawlingContext) -> None:
        context.log.info(f'API: {context.request.url}')

        # Parse JSON response
        json_data = context.response.json()

        # Extract items
        {{EXTRACTION_LOGIC}}

        # Handle pagination
        {{PAGINATION_LOGIC}}

    await crawler.run([{{START_URLS}}])
    await crawler.export_data('{{OUTPUT_FILE}}')

if __name__ == '__main__':
    asyncio.run(main())
```

## Extraction Patterns

### List Response
```python
for item in json_data.get('items', []):
    await context.push_data({
        'id': item['id'],
        'name': item['name'],
        'created_at': item['createdAt'],
    })
```

### Nested Data
```python
for product in json_data['data']['products']:
    await context.push_data({
        'sku': product['sku'],
        'name': product['name'],
        'price': product['pricing']['current'],
        'currency': product['pricing']['currency'],
        'in_stock': product['inventory']['available'] > 0,
    })
```

## Pagination Patterns

### Page-Based
```python
page = json_data['page']
if page < json_data['total_pages']:
    next_page = page + 1
    await context.add_requests([
        f'https://api.example.com/items?page={next_page}'
    ])
```

### Offset-Based
```python
offset = json_data['offset']
limit = json_data['limit']
if offset + limit < json_data['total']:
    next_offset = offset + limit
    await context.add_requests([
        f'https://api.example.com/items?offset={next_offset}&limit={limit}'
    ])
```

### Cursor-Based
```python
next_cursor = json_data.get('next_cursor')
if next_cursor:
    await context.add_requests([
        f'https://api.example.com/items?cursor={next_cursor}'
    ])
```

## Authentication Patterns

### API Key in Header
```python
from crawlee import Request

await crawler.run([
    Request(
        url='https://api.example.com/data',
        headers={'X-API-Key': 'your-api-key'}
    )
])
```

### Bearer Token
```python
Request(
    url='https://api.example.com/data',
    headers={'Authorization': 'Bearer your-token'}
)
```
