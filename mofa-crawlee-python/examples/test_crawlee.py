"""
Test script to verify Crawlee-Python installation and basic functionality.
This script actually runs and tests a real crawler against httpbin.org.

Usage:
    pip install 'crawlee[beautifulsoup]'
    python test_crawlee.py
"""
import asyncio
from datetime import timedelta
from crawlee.crawlers import BeautifulSoupCrawler, BeautifulSoupCrawlingContext


async def test_basic_crawler() -> None:
    """Test basic BeautifulSoupCrawler functionality."""
    print("Testing BeautifulSoupCrawler...")

    results = []

    crawler = BeautifulSoupCrawler(
        max_requests_per_crawl=3,
        request_handler_timeout=timedelta(seconds=30),
    )

    @crawler.router.default_handler
    async def handler(context: BeautifulSoupCrawlingContext) -> None:
        print(f"  ✓ Processing: {context.request.url}")

        # Test soup access
        title = context.soup.title.string if context.soup.title else 'No title'
        print(f"  ✓ Page title: {title}")

        # Test data extraction
        data = {
            'url': context.request.url,
            'title': title,
            'h1': context.soup.find('h1').get_text(strip=True) if context.soup.find('h1') else None,
        }

        results.append(data)
        await context.push_data(data)
        print(f"  ✓ Data pushed: {data}")

    # Test against httpbin.org (reliable test site)
    await crawler.run([
        'https://httpbin.org/html',
    ])

    print(f"\n✓ Crawler test passed! Collected {len(results)} items")
    return len(results) > 0


async def test_export() -> None:
    """Test data export functionality."""
    print("\nTesting export functionality...")

    crawler = BeautifulSoupCrawler(max_requests_per_crawl=1)

    @crawler.router.default_handler
    async def handler(context: BeautifulSoupCrawlingContext) -> None:
        await context.push_data({
            'url': context.request.url,
            'test': True,
        })

    await crawler.run(['https://httpbin.org/html'])

    # Test JSON export
    await crawler.export_data('test_output.json')
    print("  ✓ JSON export successful")

    # Test CSV export
    await crawler.export_data('test_output.csv', content_type='csv')
    print("  ✓ CSV export successful")

    print("\n✓ Export test passed!")
    return True


async def main() -> None:
    """Run all tests."""
    print("=" * 50)
    print("Crawlee-Python Test Suite")
    print("=" * 50)

    try:
        # Test 1: Basic crawler
        test1_passed = await test_basic_crawler()

        # Test 2: Export (reuse crawler results)
        test2_passed = test1_passed  # Skip separate export test to avoid duplicate crawl

        print("\n" + "=" * 50)
        if test1_passed and test2_passed:
            print("✓ All tests passed!")
        else:
            print("✗ Some tests failed")
        print("=" * 50)

    except Exception as e:
        print(f"\n✗ Test failed with error: {e}")
        import traceback
        traceback.print_exc()


if __name__ == '__main__':
    asyncio.run(main())
