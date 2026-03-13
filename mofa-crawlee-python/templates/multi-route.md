# Template: Multi-Route Crawler

Use this template for complex crawls with different handlers for different page types.

## When to Use
- Different page layouts
- Category + Detail pages
- Pagination handling
- Complex site structures

## Code Template

```python
import asyncio
from crawlee import Request
from crawlee.beautiful_soup_crawler import BeautifulSoupCrawler, BeautifulSoupCrawlingContext

async def main():
    crawler = BeautifulSoupCrawler(
        max_requests_per_crawl={{MAX_REQUESTS}},
        request_handler_timeout={{TIMEOUT}},
    )

    # Handler for listing pages
    @crawler.router.handler('list')
    async def list_handler(context: BeautifulSoupCrawlingContext) -> None:
        context.log.info(f'List page: {context.request.url}')

        {{LIST_EXTRACTION}}

        # Enqueue detail pages
        await context.enqueue_links(
            selector='{{DETAIL_SELECTOR}}',
            label='detail'
        )

        # Enqueue next page
        {{PAGINATION_LOGIC}}

    # Handler for detail pages
    @crawler.router.handler('detail')
    async def detail_handler(context: BeautifulSoupCrawlingContext) -> None:
        context.log.info(f'Detail page: {context.request.url}')

        {{DETAIL_EXTRACTION}}

    # Start with labeled requests
    await crawler.run([
        Request(url=url, label='list') for url in {{START_URLS}}
    ])

    await crawler.export_data('{{OUTPUT_FILE}}')

if __name__ == '__main__':
    asyncio.run(main())
```

## Route Patterns

### E-commerce Pattern
```python
@crawler.router.handler('category')
async def category_handler(context: BeautifulSoupCrawlingContext) -> None:
    # Extract category info
    # Enqueue products
    await context.enqueue_links(selector='.product-card a', label='product')
    # Enqueue subcategories
    await context.enqueue_links(selector='.subcategory a', label='category')
    # Enqueue pagination
    await context.enqueue_links(selector='a.next', label='category')

@crawler.router.handler('product')
async def product_handler(context: BeautifulSoupCrawlingContext) -> None:
    # Extract product details
    await context.push_data({
        'url': context.request.url,
        'title': context.soup.select_one('h1').get_text(strip=True),
        'price': context.soup.select_one('.price').get_text(strip=True),
    })
```

### News Site Pattern
```python
@crawler.router.handler('homepage')
async def homepage_handler(context: BeautifulSoupCrawlingContext) -> None:
    # Enqueue category pages
    await context.enqueue_links(selector='nav.categories a', label='category')
    # Enqueue featured articles
    await context.enqueue_links(selector='.featured-article a', label='article')

@crawler.router.handler('category')
async def category_handler(context: BeautifulSoupCrawlingContext) -> None:
    # Enqueue articles
    await context.enqueue_links(selector='.article-preview a', label='article')
    # Pagination
    await context.enqueue_links(selector='a.next-page', label='category')

@crawler.router.handler('article')
async def article_handler(context: BeautifulSoupCrawlingContext) -> None:
    # Extract article content
    await context.push_data({
        'url': context.request.url,
        'title': context.soup.select_one('h1.article-title').get_text(strip=True),
        'author': context.soup.select_one('.author').get_text(strip=True),
        'date': context.soup.select_one('.publish-date')['datetime'],
        'content': context.soup.select_one('.article-body').get_text(strip=True),
    })
```
