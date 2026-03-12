# Template: Stealth Mode (Protected Sites)

Use this template for sites with anti-bot protection.

## When to Use
- Cloudflare protected sites
- Sites with CAPTCHA
- Rate-limited sites
- Session-required sites

## Code Template

```python
import asyncio
from crawlee.playwright_crawler import PlaywrightCrawler, PlaywrightCrawlingContext
from crawlee.proxy_configuration import ProxyConfiguration

async def main():
    # Configure proxy rotation
    proxy_config = ProxyConfiguration(
        proxy_urls={{PROXY_URLS}}
    ) if {{USE_PROXY}} else None

    crawler = PlaywrightCrawler(
        headless=True,
        proxy_configuration=proxy_config,
        use_session_pool=True,
        session_pool_settings={
            'max_pool_size': {{SESSION_POOL_SIZE}},
            'persist_cookies_per_session': True,
        },
        max_requests_per_crawl={{MAX_REQUESTS}},
        request_handler_timeout={{TIMEOUT}},
    )

    @crawler.router.default_handler
    async def handler(context: PlaywrightCrawlingContext) -> None:
        page = context.page

        # Human-like delays
        {{DELAY_LOGIC}}

        # Natural scrolling
        {{SCROLL_LOGIC}}

        # Extract data
        {{EXTRACTION_LOGIC}}

    await crawler.run([{{START_URLS}}])
    await crawler.export_data('{{OUTPUT_FILE}}')

if __name__ == '__main__':
    asyncio.run(main())
```

## Stealth Techniques

### Random Delays
```python
import random

await page.wait_for_timeout(1000 + random.randint(500, 2000))
```

### Natural Scrolling
```python
await page.evaluate('''
    async () => {
        const delay = ms => new Promise(r => setTimeout(r, ms));
        for (let i = 0; i < 5; i++) {
            window.scrollBy(0, 300 + Math.random() * 200);
            await delay(500 + Math.random() * 1000);
        }
    }
''')
```

### Mouse Movements
```python
await page.mouse.move(100, 200)
await page.wait_for_timeout(200)
await page.mouse.move(150, 250)
```

## Proxy Configuration

### Rotating Proxies
```python
proxy_config = ProxyConfiguration(
    proxy_urls=[
        'http://user:pass@proxy1.example.com:8080',
        'http://user:pass@proxy2.example.com:8080',
        'http://user:pass@proxy3.example.com:8080',
    ]
)
```

### Proxy with Session
```python
# Each session maintains its own proxy
crawler = PlaywrightCrawler(
    use_session_pool=True,
    session_pool_settings={
        'max_pool_size': 10,
        'persist_cookies_per_session': True,
    },
    proxy_configuration=proxy_config,
)
```
