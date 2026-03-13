# Template: Simple Static Site Crawler

Use this template for scraping static HTML sites with BeautifulSoup.

## When to Use
- Static HTML pages
- Server-rendered content
- No JavaScript required
- Fast extraction needed

## Code Template

```python
import asyncio
from crawlee import Request
from crawlee.beautiful_soup_crawler import BeautifulSoupCrawler, BeautifulSoupCrawlingContext

async def main():
    crawler = BeautifulSoupCrawler(
        max_requests_per_crawl={{MAX_REQUESTS}},
        max_requests_per_second={{MAX_RPS}},
        request_handler_timeout={{TIMEOUT}},
    )

    @crawler.router.default_handler
    async def handler(context: BeautifulSoupCrawlingContext) -> None:
        context.log.info(f'Processing {context.request.url}')

        # Extract data
        {{EXTRACTION_LOGIC}}

        # Follow links (optional)
        {{LINK_FOLLOWING}}

    await crawler.run([{{START_URLS}}])
    await crawler.export_data('{{OUTPUT_FILE}}')

if __name__ == '__main__':
    asyncio.run(main())
```

## Extraction Logic Patterns

### Basic Text Extraction
```python
title = context.soup.find('h1').get_text(strip=True)
description = context.soup.select_one('.description').get_text(strip=True)
```

### List Extraction
```python
items = context.soup.select('.item')
for item in items:
    data = {
        'name': item.select_one('.name').get_text(strip=True),
        'price': item.select_one('.price').get_text(strip=True),
    }
    await context.push_data(data)
```

### Attribute Extraction
```python
data = {
    'url': context.request.url,
    'image': context.soup.select_one('img.product')['src'],
    'sku': context.soup.select_one('[data-sku]')['data-sku'],
}
await context.push_data(data)
```

## Link Following Patterns

### Pagination
```python
next_page = context.soup.select_one('a.next')
if next_page:
    await context.enqueue_links(selector='a.next')
```

### Detail Pages
```python
# On list page, enqueue all detail links
await context.enqueue_links(
    selector='.product-link',
    label='detail'
)

# Handle detail pages
@crawler.router.handler('detail')
async def detail_handler(context: BeautifulSoupCrawlingContext) -> None:
    # Extract detailed data
    pass
```
