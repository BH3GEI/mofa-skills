"""
News Aggregator Example
Demonstrates content extraction and JSON-LD structured data parsing

Usage:
    pip install 'crawlee[beautifulsoup]'
    python news-aggregator.py
"""
import asyncio
import json
from datetime import datetime, timedelta
from crawlee import Request
from crawlee.crawlers import BeautifulSoupCrawler, BeautifulSoupCrawlingContext


async def main() -> None:
    crawler = BeautifulSoupCrawler(
        max_requests_per_crawl=20,
        request_handler_timeout=timedelta(seconds=30),
    )

    @crawler.router.handler('homepage')
    async def homepage_handler(context: BeautifulSoupCrawlingContext) -> None:
        """Process homepage - enqueue links."""
        context.log.info(f'Homepage: {context.request.url}')

        await context.enqueue_links(
            selector='nav a, .article-link',
            label='article'
        )

    @crawler.router.handler('article')
    async def article_handler(context: BeautifulSoupCrawlingContext) -> None:
        """Extract article content."""
        context.log.info(f'Article: {context.request.url}')

        # Try JSON-LD first
        article_data = extract_json_ld(context)

        if not article_data:
            article_data = extract_from_html(context)

        if article_data.get('title'):
            await context.push_data(article_data)

    def extract_json_ld(context: BeautifulSoupCrawlingContext) -> dict:
        """Extract from JSON-LD structured data."""
        scripts = context.soup.find_all('script', type='application/ld+json')

        for script in scripts:
            try:
                data = json.loads(script.string)
                items = data if isinstance(data, list) else [data]

                for item in items:
                    if item.get('@type') in ['NewsArticle', 'Article']:
                        return {
                            'url': context.request.url,
                            'title': item.get('headline'),
                            'author': item.get('author', {}).get('name') if isinstance(item.get('author'), dict) else item.get('author'),
                            'published_date': item.get('datePublished'),
                            'description': item.get('description'),
                        }
            except (json.JSONDecodeError, AttributeError, TypeError):
                continue

        return {}

    def extract_from_html(context: BeautifulSoupCrawlingContext) -> dict:
        """Extract from HTML as fallback."""
        title = context.soup.select_one('h1')
        author = context.soup.select_one('.author, .byline')
        date_elem = context.soup.select_one('time[datetime]')

        return {
            'url': context.request.url,
            'title': title.get_text(strip=True) if title else None,
            'author': author.get_text(strip=True) if author else None,
            'published_date': date_elem['datetime'] if date_elem else None,
            'crawled_at': datetime.now().isoformat(),
        }

    await crawler.run([
        Request(url='https://httpbin.org/html', label='article'),
    ])

    await crawler.export_data('articles.json')
    print("News aggregator test completed!")


if __name__ == '__main__':
    asyncio.run(main())
