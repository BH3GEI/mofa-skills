# Template: JavaScript SPA Crawler (Playwright)

Use this template for scraping JavaScript-heavy sites and SPAs.

## When to Use
- React/Vue/Angular apps
- Content loaded dynamically
- Infinite scroll
- User interactions needed

## Code Template

```python
import asyncio
from crawlee.playwright_crawler import PlaywrightCrawler, PlaywrightCrawlingContext

async def main():
    crawler = PlaywrightCrawler(
        headless={{HEADLESS}},
        browser_type='{{BROWSER}}',  # chromium | firefox | webkit
        max_requests_per_crawl={{MAX_REQUESTS}},
        request_handler_timeout={{TIMEOUT}},
    )

    @crawler.router.default_handler
    async def handler(context: PlaywrightCrawlingContext) -> None:
        context.log.info(f'Processing {context.request.url}')
        page = context.page

        # Wait for dynamic content
        {{WAIT_LOGIC}}

        # Interact with page if needed
        {{INTERACTION_LOGIC}}

        # Extract data
        {{EXTRACTION_LOGIC}}

    await crawler.run([{{START_URLS}}])
    await crawler.export_data('{{OUTPUT_FILE}}')

if __name__ == '__main__':
    asyncio.run(main())
```

## Wait Logic Patterns

### Wait for Selector
```python
await page.wait_for_selector('.product-grid')
```

### Wait for Network Idle
```python
await page.wait_for_load_state('networkidle')
```

### Custom Wait Condition
```python
await page.wait_for_function('''
    () => document.querySelectorAll('.item').length > 0
''')
```

## Interaction Patterns

### Click Element
```python
await page.click('.load-more-button')
await page.wait_for_timeout(2000)
```

### Fill Form
```python
await page.fill('input[name="search"]', 'query')
await page.press('input[name="search"]', 'Enter')
await page.wait_for_load_state('networkidle')
```

### Infinite Scroll
```python
await page.evaluate('''
    async () => {
        while (document.querySelector('.loading')) {
            window.scrollTo(0, document.body.scrollHeight);
            await new Promise(r => setTimeout(r, 1000));
        }
    }
''')
```

## Extraction Patterns

### Query Selector
```python
element = await page.query_selector('.price')
price = await element.text_content()
```

### Evaluate in Page
```python
data = await page.evaluate('''
    () => {
        return {
            title: document.querySelector('h1').innerText,
            price: document.querySelector('.price').innerText,
        }
    }
''')
await context.push_data(data)
```

### Query All Elements
```python
items = await page.query_selector_all('.product')
for item in items:
    name = await item.query_selector_eval('.name', 'el => el.textContent')
    price = await item.query_selector_eval('.price', 'el => el.textContent')
    await context.push_data({'name': name, 'price': price})
```
