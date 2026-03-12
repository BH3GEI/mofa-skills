---
name: mofa-crawlee-python
description: "Teach AI to use Crawlee-Python for robust web scraping and crawling. Triggers: crawlee, web scraping, python crawler, apify, crawler"
requires_bins: python, pip
always: false
---

# MOFA Crawlee-Python

A comprehensive skill for teaching AI to use [Crawlee-Python](https://github.com/apify/crawlee-python) - Apify's robust web scraping and browser automation library.

## What is Crawlee-Python?

Crawlee is a battle-tested web scraping library that provides:

- **Declarative API**: Write crawlers with clean, Pythonic syntax
- **Automatic scaling**: Parallel crawling with configurable concurrency
- **Resilient crawling**: Automatic retries, error handling, session management
- **Multiple crawler types**: BeautifulSoup, Parsel, Playwright (browser automation)
- **Data export**: JSON, CSV, Parquet, SQL databases
- **Proxy rotation**: Built-in proxy management
- **Request routing**: URL pattern matching for different handlers
- **Data pipelines**: Structured data extraction and validation

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      MOFA CRAWLEE-PYTHON PIPELINE                           │
└─────────────────────────────────────────────────────────────────────────────┘

Phase 1: REQUIREMENTS ANALYSIS
───────────────────────────────
User Request → Extract crawling requirements
                          ↓
              ┌─────────────────────┐
              │  Identify:          │
              │  - Target URLs      │
              │  - Data to extract  │
              │  - Output format    │
              │  - Scale/depth      │
              │  - Anti-bot needs   │
              └─────────────────────┘

Phase 2: CRAWLER SELECTION
────────────────────────────
                         ┌─────────────┐
                    ┌────┤ Static HTML ├────┐
                    │    └─────────────┘    │
                    │    ┌─────────────┐    │
              Choose├────┤  JavaScript ├────┤ Based on
              Type  │    └─────────────┘    │ Target
                    │    ┌─────────────┐    │
                    ├────┤    Browser  ├────┤
                    │    └─────────────┘    │
                    │    ┌─────────────┐    │
                    └────┤    Router   ├────┘
                         └─────────────┘

Phase 3: CODE GENERATION
────────────────────────
Generate Crawlee code:
┌─────────────────────────────────────────────────────────┐
│  • Import statements                                    │
│  • Request handlers                                     │
│  • Data extraction logic                                │
│  • Error handling                                       │
│  • Export configuration                                 │
│  • Best practices (delays, user-agent, etc.)           │
└─────────────────────────────────────────────────────────┘

Phase 4: VALIDATION
───────────────────
Review code for:
- Correct Crawlee API usage
- Error handling completeness
- Rate limiting & politeness
- Data structure correctness

Phase 5: EXECUTION GUIDE
────────────────────────
Provide run instructions:
- Installation commands
- Environment setup
- Execution steps
- Output inspection
```

## Quick Start

### Installation

```bash
# Requires Python 3.10+

# Basic installation
pip install crawlee

# With BeautifulSoup support
pip install 'crawlee[beautifulsoup]'

# With Parsel (XPath/CSS) support
pip install 'crawlee[parsel]'

# With browser automation
pip install 'crawlee[playwright]'
playwright install

# Full installation
pip install 'crawlee[all]'

# CLI quick start
uvx 'crawlee[cli]' create my-crawler
```

### Basic Crawler Types

| Crawler | Use Case | Best For |
|---------|----------|----------|
| `BeautifulSoupCrawler` | Static HTML | Simple sites, speed |
| `ParselCrawler` | XPath/CSS extraction | Complex selectors |
| `PlaywrightCrawler` | Browser automation | SPAs, JS-heavy sites |
| `AdaptivePlaywrightCrawler` | Smart hybrid | Auto-switch HTTP/browser |
| `HttpCrawler` | Raw HTTP requests | API scraping, speed |

### Crawler Selection Guide

```
Target site analysis:
│
├─ Uses JavaScript rendering? (React, Vue, Angular)
│  ├─ YES → PlaywrightCrawler
│  │        ├─ Very large site? → AdaptivePlaywrightCrawler
│  │        └─ Anti-bot protection? → Stealth mode + proxy
│  │
│  └─ NO → Extraction complexity?
│         ├─ Simple CSS selectors → BeautifulSoupCrawler (fastest)
│         ├─ Complex XPath/CSS → ParselCrawler
│         └─ JSON/API endpoints → HttpCrawler
│
└─ Unknown? → Start with BeautifulSoupCrawler, fallback to Playwright if fails
```

## Phase 1: Requirements Analysis

Analyze the user's scraping needs:

```json
{
  "role": "user",
  "content": "Analyze this web scraping request and extract requirements.\n\nRequest: {{REQUEST}}\n\nExtract:\n1. Target URLs or URL patterns\n2. Specific data fields to extract\n3. Crawling depth (single page, site-wide, etc.)\n4. Output format preference\n5. Scale requirements\n6. Anti-detection needs\n7. Rate limiting requirements\n\nRespond with structured requirements."
}
```

## Phase 2: Crawler Selection Decision Tree

```
Target uses JavaScript? (React, Vue, SPA)
├── YES → Use PlaywrightCrawler
│         └── Need stealth? → Enable stealth mode + proxy
│
└── NO → Complexity of extraction?
    ├── Simple selectors → BeautifulSoupCrawler (fastest)
    ├── Complex XPath/CSS → ParselCrawler
    └── API endpoints → HttpCrawler
```

## Phase 3: Code Generation Patterns

### Pattern 1: Simple Static Site (BeautifulSoup)

```python
import asyncio
from datetime import timedelta
from crawlee import Request
from crawlee.crawlers import BeautifulSoupCrawler, BeautifulSoupCrawlingContext

async def main() -> None:
    crawler = BeautifulSoupCrawler(
        max_requests_per_crawl=50,
        request_handler_timeout=timedelta(seconds=30),
    )

    @crawler.router.default_handler
    async def handler(context: BeautifulSoupCrawlingContext) -> None:
        context.log.info(f'Processing {context.request.url} ...')

        # Extract data using BeautifulSoup
        title = context.soup.title.string if context.soup.title else None
        items = context.soup.select('.item')

        for item in items:
            data = {
                'url': context.request.url,
                'title': title,
                'name': item.select_one('.name').get_text(strip=True),
                'price': item.select_one('.price').get_text(strip=True),
            }
            await context.push_data(data)

        # Follow all links (filtered by pattern)
        await context.enqueue_links(
            include=['/products/*', '/items/*'],
            exclude=['/cart/*', '/login/*'],
        )

    await crawler.run([
        'https://example.com/products'
    ])

    # Export results
    await crawler.export_data('results.json')
    await crawler.export_data('results.csv', content_type='csv')

if __name__ == '__main__':
    asyncio.run(main())
```

### Pattern 2: JavaScript-Heavy Site (Playwright)

```python
import asyncio
from datetime import timedelta
from crawlee.crawlers import PlaywrightCrawler, PlaywrightCrawlingContext

async def main() -> None:
    crawler = PlaywrightCrawler(
        headless=True,
        browser_type='chromium',  # chromium | firefox | webkit
        max_requests_per_crawl=20,
        request_handler_timeout=timedelta(seconds=60),
    )

    @crawler.router.default_handler
    async def handler(context: PlaywrightCrawlingContext) -> None:
        context.log.info(f'Processing {context.request.url} ...')
        page = context.page

        # Wait for dynamic content
        await page.wait_for_selector('.product-grid')

        # Scroll to load more (infinite scroll)
        await page.evaluate('''
            async () => {
                while (document.querySelector('.load-more')) {
                    document.querySelector('.load-more').click();
                    await new Promise(r => setTimeout(r, 1000));
                }
            }
        ''')

        # Extract data using Playwright
        title = await page.title()
        products = await page.query_selector_all('.product')

        for product in products:
            data = {
                'url': context.request.url,
                'page_title': title,
                'name': await product.query_selector_eval('.name', 'el => el.textContent'),
                'price': await product.query_selector_eval('.price', 'el => el.textContent'),
            }
            await context.push_data(data)

        # Follow links
        await context.enqueue_links()

    await crawler.run(['https://spa-example.com'])
    await crawler.export_data('products.json')

if __name__ == '__main__':
    asyncio.run(main())
```

### Pattern 3: Multi-Route Crawler

```python
import asyncio
from datetime import timedelta
from crawlee import Request
from crawlee.crawlers import BeautifulSoupCrawler, BeautifulSoupCrawlingContext

async def main() -> None:
    crawler = BeautifulSoupCrawler(
        max_requests_per_crawl=100,
    )

    # Handler for product list pages
    @crawler.router.handler('list')
    async def list_handler(context: BeautifulSoupCrawlingContext) -> None:
        context.log.info(f'List page: {context.request.url}')

        # Enqueue all product links with 'product' label
        await context.enqueue_links(
            selector='.product-link',
            label='product'
        )

        # Enqueue pagination with 'list' label
        next_page = context.soup.select_one('a.pagination-next')
        if next_page and 'disabled' not in next_page.get('class', []):
            await context.enqueue_links(
                selector='a.pagination-next',
                label='list'
            )

    # Handler for product detail pages
    @crawler.router.handler('product')
    async def product_handler(context: BeautifulSoupCrawlingContext) -> None:
        context.log.info(f'Product page: {context.request.url}')

        data = {
            'url': context.request.url,
            'title': context.soup.select_one('h1.title').get_text(strip=True) if context.soup.select_one('h1.title') else None,
            'price': context.soup.select_one('.price').get_text(strip=True) if context.soup.select_one('.price') else None,
            'description': context.soup.select_one('.description').get_text(strip=True) if context.soup.select_one('.description') else None,
            'images': [img['src'] for img in context.soup.select('.gallery img') if img.get('src')],
        }
        await context.push_data(data)

    await crawler.run([
        Request(url='https://example.com/category/page/1', label='list')
    ])

    await crawler.export_data('products.json')
    await crawler.export_data('products.csv', content_type='csv')

if __name__ == '__main__':
    asyncio.run(main())
```

### Pattern 4: API Scraping (HttpCrawler)

```python
import asyncio
from datetime import timedelta
from crawlee.crawlers import HttpCrawler, HttpCrawlingContext

async def main() -> None:
    crawler = HttpCrawler(
        max_requests_per_crawl=1000,
        request_handler_timeout=timedelta(seconds=30),
    )

    @crawler.router.default_handler
    async def handler(context: HttpCrawlingContext) -> None:
        context.log.info(f'API: {context.request.url}')

        # Parse JSON response
        json_data = context.response.json()

        # Extract items
        for item in json_data.get('items', []):
            await context.push_data({
                'id': item['id'],
                'name': item['name'],
                'created_at': item.get('createdAt'),
            })

        # Handle pagination
        if json_data.get('has_more') or json_data.get('hasMore'):
            next_page = json_data.get('page', 1) + 1
            await context.add_requests([
                f'https://api.example.com/items?page={next_page}'
            ])

    await crawler.run(['https://api.example.com/items?page=1'])
    await crawler.export_data('api_data.json')

if __name__ == '__main__':
    asyncio.run(main())
```

### Pattern 5: ParselCrawler (XPath/CSS Selectors)

```python
import asyncio
from datetime import timedelta
from crawlee.crawlers import ParselCrawler, ParselCrawlingContext

async def main() -> None:
    crawler = ParselCrawler(
        max_requests_per_crawl=50,
        request_handler_timeout=timedelta(seconds=30),
    )

    @crawler.router.default_handler
    async def handler(context: ParselCrawlingContext) -> None:
        context.log.info(f'Processing {context.request.url} ...')

        # Extract using XPath
        title = context.selector.xpath('//title/text()').get()

        # Extract using CSS selectors
        items = context.selector.css('.product-item')

        for item in items:
            data = {
                'url': context.request.url,
                'title': title,
                # XPath relative to item
                'name': item.xpath('.//h2/text()').get(),
                # CSS selector relative to item
                'price': item.css('.price::text').get(),
                'link': item.css('a::attr(href)').get(),
            }
            await context.push_data(data)

        # Follow links
        await context.enqueue_links()

    await crawler.run(['https://example.com/products'])
    await crawler.export_data('results.json')

if __name__ == '__main__':
    asyncio.run(main())
```

### Pattern 6: AdaptivePlaywrightCrawler (Smart Hybrid)

```python
import asyncio
from datetime import timedelta
from crawlee.crawlers import AdaptivePlaywrightCrawler, AdaptivePlaywrightCrawlingContext

async def main() -> None:
    """
    AdaptivePlaywrightCrawler automatically chooses between HTTP and browser
    based on whether the page needs JavaScript rendering.
    """
    crawler = AdaptivePlaywrightCrawler(
        max_requests_per_crawl=100,
        request_handler_timeout=timedelta(seconds=60),
    )

    @crawler.router.default_handler
    async def handler(context: AdaptivePlaywrightCrawlingContext) -> None:
        context.log.info(f'Processing {context.request.url} ...')

        # Use the response directly (HTTP if static, browser if JS needed)
        if context.http_response:
            # HTTP response available - use BeautifulSoup
            title = context.soup.title.string if context.soup.title else None
            context.log.info('Used HTTP crawler')
        else:
            # Browser was needed
            title = await context.page.title()
            context.log.info('Used Playwright crawler')

        # Extract data
        data = {
            'url': context.request.url,
            'title': title,
        }
        await context.push_data(data)

        # Follow links
        await context.enqueue_links()

    await crawler.run(['https://example.com'])
    await crawler.export_data('results.json')

if __name__ == '__main__':
    asyncio.run(main())
```

### Pattern 7: Stealth Mode (Protected Sites)

```python
import asyncio
from datetime import timedelta
from crawlee.crawlers import PlaywrightCrawler, PlaywrightCrawlingContext
from crawlee.proxy_configuration import ProxyConfiguration

async def main() -> None:
    # Configure proxy rotation (optional)
    proxy_config = ProxyConfiguration(
        proxy_urls=[
            'http://user:pass@proxy1.example.com:8080',
            'http://user:pass@proxy2.example.com:8080',
        ]
    )

    crawler = PlaywrightCrawler(
        headless=True,
        browser_type='chromium',
        proxy_configuration=proxy_config,
        max_requests_per_crawl=20,
        request_handler_timeout=timedelta(seconds=120),
        # Additional browser options for stealth
        browser_launch_options={
            'args': [
                '--disable-blink-features=AutomationControlled',
                '--disable-web-security',
            ]
        },
    )

    @crawler.router.default_handler
    async def handler(context: PlaywrightCrawlingContext) -> None:
        page = context.page

        # Random delays between actions
        await page.wait_for_timeout(2000 + int(asyncio.get_event_loop().time() * 1000) % 3000)

        # Natural scrolling behavior
        await page.evaluate('''
            async () => {
                const delay = ms => new Promise(r => setTimeout(r, ms));
                for (let i = 0; i < 3; i++) {
                    window.scrollBy(0, 500);
                    await delay(500 + Math.random() * 1000);
                }
            }
        ''')

        # Extract data
        title = await page.title()
        await context.push_data({
            'url': context.request.url,
            'title': title,
        })

        # Follow links with delay
        await page.wait_for_timeout(1000)
        await context.enqueue_links()

    await crawler.run(['https://protected-site.com'])
    await crawler.export_data('results.json')

if __name__ == '__main__':
    asyncio.run(main())
```

## Phase 4: Common Extraction Patterns

### Extracting Tables

```python
async def extract_table(context: BeautifulSoupCrawlingContext):
    table = context.soup.select_one('table.data')
    headers = [th.get_text(strip=True) for th in table.select('th')]

    for row in table.select('tr')[1:]:
        cells = row.select('td')
        data = {headers[i]: cells[i].get_text(strip=True)
                for i in range(len(headers))}
        await context.push_data(data)
```

### Handling Infinite Scroll

```python
async def handle_infinite_scroll(context: PlaywrightCrawlingContext):
    page = context.page

    previous_height = 0
    while True:
        # Scroll to bottom
        await page.evaluate('window.scrollTo(0, document.body.scrollHeight)')
        await page.wait_for_timeout(2000)

        current_height = await page.evaluate('document.body.scrollHeight')
        if current_height == previous_height:
            break
        previous_height = current_height
```

### Extracting Structured Data (JSON-LD)

```python
import json

async def extract_jsonld(context: BeautifulSoupCrawlingContext):
    scripts = context.soup.find_all('script', type='application/ld+json')
    for script in scripts:
        data = json.loads(script.string)
        if data.get('@type') == 'Product':
            await context.push_data({
                'name': data.get('name'),
                'description': data.get('description'),
                'price': data.get('offers', {}).get('price'),
            })
```

## Phase 5: Configuration Reference

### Crawler Configuration Options

```python
BeautifulSoupCrawler(
    # Request limits
    max_requests_per_crawl=100,
    max_requests_per_second=10,

    # Timeouts
    request_handler_timeout=30,
    navigation_timeout=30,

    # Concurrency
    min_concurrency=1,
    max_concurrency=10,

    # Retry settings
    max_request_retries=3,
    retry_on_blocked=True,

    # Proxy
    proxy_configuration=proxy_config,

    # Sessions
    use_session_pool=True,
    persist_cookies_per_session=True,
)
```

### Request Options

```python
Request(
    url='https://example.com',
    label='product',
    user_data={
        'page_num': 1,
        'category': 'electronics'
    },
    headers={
        'Accept-Language': 'en-US',
    },
)
```

## Phase 6: Error Handling Best Practices

```python
from crawlee import Request
from crawlee.errors import SessionError

@crawler.router.default_handler
async def handler(context: BeautifulSoupCrawlingContext) -> None:
    try:
        # Extraction logic
        element = context.soup.select_one('.price')
        if not element:
            context.log.warning(f'Price not found on {context.request.url}')
            return

        price = element.get_text(strip=True)

    except Exception as e:
        context.log.error(f'Error processing {context.request.url}: {e}')
        # Requeue for retry if needed
        raise
```

## Output Requirements

When user requests a crawler, provide:

1. **Complete Python code** with imports and main block
2. **Installation commands** for dependencies
3. **Usage instructions** with example run command
4. **Output format** explanation
5. **Rate limiting notes** (be polite to target sites)

### File Organization

```
project/
├── crawler.py          # Main crawler code
├── requirements.txt    # Dependencies
└── output/            # Generated data
    ├── data.json
    └── data.csv
```

## Integration with Other Skills

### With mofa-research-2.0
```
mofa-research-2.0: Research target site structure
         ↓
mofa-crawlee-python: Build specialized crawler
         ↓
mofa-research-2.0: Analyze crawled data
```

### With mofa-data-analysis
```
mofa-crawlee-python: Crawl data → CSV/JSON
         ↓
mofa-data-analysis: Clean, analyze, visualize
```

## Examples

See `examples/` directory for complete working examples:
- `ecommerce-products.py` - Product scraping
- `news-aggregator.py` - News article extraction
- `social-media.py` - Social media data collection
- `api-crawler.py` - REST API scraping

## Resources

- [Crawlee Python Docs](https://crawlee.dev/python/)
- [GitHub Repository](https://github.com/apify/crawlee-python)
- [API Reference](https://crawlee.dev/python/api)
- [Examples Gallery](https://crawlee.dev/python/docs/examples)
