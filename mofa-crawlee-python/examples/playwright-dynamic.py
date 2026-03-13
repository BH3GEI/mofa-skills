"""
Dynamic Content Scraper with Playwright
Demonstrates JavaScript-heavy site crawling

Usage:
    pip install 'crawlee[playwright]'
    playwright install
    python playwright-dynamic.py
"""
import asyncio
from datetime import timedelta
from crawlee.crawlers import PlaywrightCrawler, PlaywrightCrawlingContext


async def main() -> None:
    crawler = PlaywrightCrawler(
        headless=True,
        browser_type='chromium',
        max_requests_per_crawl=5,
        request_handler_timeout=timedelta(seconds=60),
    )

    @crawler.router.default_handler
    async def handler(context: PlaywrightCrawlingContext) -> None:
        """Handle dynamic pages."""
        context.log.info(f'Processing: {context.request.url}')
        page = context.page

        # Wait for content
        await page.wait_for_load_state('networkidle')

        # Get page title
        title = await page.title()

        # Extract data
        data = {
            'url': context.request.url,
            'title': title,
        }
        await context.push_data(data)

        context.log.info(f'Page title: {title}')

    await crawler.run([
        'https://httpbin.org/html',
    ])

    await crawler.export_data('dynamic_results.json')
    print("Playwright test completed!")


if __name__ == '__main__':
    asyncio.run(main())
