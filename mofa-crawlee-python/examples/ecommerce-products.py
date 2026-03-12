"""
E-commerce Product Scraper Example
Demonstrates multi-route crawling with BeautifulSoupCrawler

Usage:
    pip install 'crawlee[beautifulsoup]'
    python ecommerce-products.py
"""
import asyncio
from datetime import timedelta
from crawlee import Request
from crawlee.crawlers import BeautifulSoupCrawler, BeautifulSoupCrawlingContext


async def main() -> None:
    crawler = BeautifulSoupCrawler(
        max_requests_per_crawl=50,
        request_handler_timeout=timedelta(seconds=30),
    )

    @crawler.router.handler('category')
    async def category_handler(context: BeautifulSoupCrawlingContext) -> None:
        """Handle category listing pages."""
        context.log.info(f'Processing category: {context.request.url}')

        # Extract category name
        category = context.soup.select_one('h1.category-title')
        category_name = category.get_text(strip=True) if category else 'Unknown'

        # Enqueue all product detail pages
        await context.enqueue_links(
            selector='.product-card a',
            label='product',
            user_data={'category': category_name}
        )

        # Handle pagination
        next_link = context.soup.select_one('a.pagination-next')
        if next_link and 'disabled' not in next_link.get('class', []):
            await context.enqueue_links(
                selector='a.pagination-next',
                label='category'
            )

    @crawler.router.handler('product')
    async def product_handler(context: BeautifulSoupCrawlingContext) -> None:
        """Handle product detail pages."""
        context.log.info(f'Processing product: {context.request.url}')

        # Extract with fallbacks
        title_elem = (
            context.soup.select_one('h1.product-title') or
            context.soup.select_one('h1')
        )

        price_elem = context.soup.select_one('.price')
        description_elem = context.soup.select_one('.description')

        # Get all images
        images = [img['src'] for img in context.soup.select('.gallery img') if img.get('src')]

        # Get SKU
        sku_elem = context.soup.select_one('[data-sku]')
        sku = sku_elem['data-sku'] if sku_elem else None

        product_data = {
            'url': context.request.url,
            'category': context.request.user_data.get('category', 'Unknown'),
            'title': title_elem.get_text(strip=True) if title_elem else 'N/A',
            'price': price_elem.get_text(strip=True) if price_elem else None,
            'description': description_elem.get_text(strip=True) if description_elem else None,
            'sku': sku,
            'images': images,
            'image_count': len(images),
        }

        await context.push_data(product_data)

    # Start crawling
    start_urls = [
        Request(url='https://example-shop.com/electronics', label='category'),
    ]

    await crawler.run(start_urls)

    # Export results
    await crawler.export_data('products.json')
    await crawler.export_data('products.csv', content_type='csv')

    context.log.info('Crawl completed!')


if __name__ == '__main__':
    asyncio.run(main())
