# Template: Error Handling Best Practices

Common error handling patterns for robust crawlers.

## Basic Error Handling

```python
@crawler.router.default_handler
async def handler(context: BeautifulSoupCrawlingContext) -> None:
    try:
        # Main extraction logic
        title = context.soup.find('h1').get_text(strip=True)

    except AttributeError as e:
        context.log.warning(f'Missing element on {context.request.url}: {e}')
        return

    except Exception as e:
        context.log.error(f'Error processing {context.request.url}: {e}')
        raise  # Re-raise to trigger retry
```

## Graceful Degradation

```python
@crawler.router.default_handler
async def handler(context: BeautifulSoupCrawlingContext) -> None:
    # Try primary selector, fall back to alternatives
    title_elem = (
        context.soup.select_one('h1.title') or
        context.soup.select_one('h1') or
        context.soup.select_one('.product-name')
    )

    title = title_elem.get_text(strip=True) if title_elem else 'N/A'

    # Try to get price, but don't fail if missing
    price_elem = context.soup.select_one('.price')
    price = price_elem.get_text(strip=True) if price_elem else None

    await context.push_data({
        'url': context.request.url,
        'title': title,
        'price': price,
    })
```

## Conditional Retries

```python
from crawlee.errors import SessionError

@crawler.router.default_handler
async def handler(context: BeautifulSoupCrawlingContext) -> None:
    try:
        # Check for blocking
        if context.soup.select_one('.blocked-message'):
            context.log.warning('Detected blocking, rotating session')
            raise SessionError('Blocked by target site')

        # Normal extraction
        {{EXTRACTION_LOGIC}}

    except SessionError:
        # Force session rotation and retry
        raise

    except Exception as e:
        context.log.error(f'Unexpected error: {e}')
        raise
```

## Validation

```python
async def validate_data(data: dict) -> bool:
    """Validate extracted data before pushing."""
    required_fields = ['title', 'url']
    for field in required_fields:
        if not data.get(field):
            return False
    return True

@crawler.router.default_handler
async def handler(context: BeautifulSoupCrawlingContext) -> None:
    data = {
        'url': context.request.url,
        'title': context.soup.select_one('h1').get_text(strip=True),
        'price': context.soup.select_one('.price').get_text(strip=True),
    }

    if await validate_data(data):
        await context.push_data(data)
    else:
        context.log.warning(f'Invalid data on {context.request.url}')
```
