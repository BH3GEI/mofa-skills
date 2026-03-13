# 示例: 结构化数据提取

## 场景
从电商网站提取商品信息（名称、价格、图片等）。

## 执行步骤

### 1. 创建沙箱
```bash
verge-browser sandbox create --alias scraper --width 1440 --height 900
```

### 2. 数据提取脚本

**extract_products.py:**
```python
import asyncio
import json
from playwright.async_api import async_playwright

async def extract_products():
    async with async_playwright() as p:
        browser = await p.chromium.connect_over_cdp(
            "ws://localhost:8000/sandbox/scraper/cdp"
        )
        page = await browser.new_page()

        # 访问目标网站
        await page.goto("https://example.com/products")
        await page.wait_for_load_state("networkidle")

        products = []
        page_num = 1

        while True:
            print(f"Extracting page {page_num}...")

            # 等待商品加载
            await page.wait_for_selector(".product-card")

            # 提取当前页商品
            items = await page.query_selector_all(".product-card")

            for item in items:
                try:
                    name = await item.query_selector_eval(
                        ".product-name", "el => el.textContent.trim()"
                    )
                    price = await item.query_selector_eval(
                        ".product-price", "el => el.textContent.trim()"
                    )
                    image = await item.query_selector_eval(
                        ".product-image img", "el => el.src"
                    )
                    link = await item.query_selector_eval(
                        "a", "el => el.href"
                    )

                    products.append({
                        "name": name,
                        "price": price,
                        "image": image,
                        "link": link
                    })
                except Exception as e:
                    print(f"Error extracting item: {e}")

            # 截图记录
            await page.screenshot(
                path=f"/workspace/page-{page_num:03d}.png",
                full_page=True
            )

            # 检查是否有下一页
            next_btn = await page.query_selector(".pagination .next")
            if not next_btn:
                break

            is_disabled = await next_btn.evaluate("el => el.disabled")
            if is_disabled:
                break

            # 点击下一页
            await next_btn.click()
            await page.wait_for_load_state("networkidle")
            page_num += 1

        # 保存数据
        with open("/workspace/products.json", "w") as f:
            json.dump(products, f, indent=2, ensure_ascii=False)

        print(f"Extracted {len(products)} products")
        await browser.close()

asyncio.run(extract_products())
```

### 3. Actions API 方式 (简单场景)

**extract_actions.json:**
```json
{
  "actions": [
    {
      "type": "goto",
      "url": "https://example.com/products"
    },
    {
      "type": "wait",
      "ms": 3000
    },
    {
      "type": "screenshot",
      "output": "/workspace/products-page.png"
    },
    {
      "type": "scroll",
      "direction": "down",
      "amount": 1000
    },
    {
      "type": "screenshot",
      "output": "/workspace/products-scrolled.png"
    }
  ]
}
```

```bash
verge-browser browser actions scraper --input extract_actions.json
```

### 4. 数据清洗和导出

**process_data.py:**
```python
import json
import csv
import re

# 读取提取的数据
with open("/workspace/products.json") as f:
    products = json.load(f)

# 清洗价格数据
for p in products:
    # 提取数字价格
    price_match = re.search(r'[\d,]+\.?\d*', p['price'])
    if price_match:
        p['price_numeric'] = float(price_match.group().replace(',', ''))
    else:
        p['price_numeric'] = None

# 保存为 CSV
with open("/workspace/products.csv", "w", newline="") as f:
    writer = csv.DictWriter(f, fieldnames=["name", "price", "price_numeric", "image", "link"])
    writer.writeheader()
    writer.writerows(products)

# 生成统计
prices = [p['price_numeric'] for p in products if p['price_numeric']]
stats = {
    "total": len(products),
    "valid_prices": len(prices),
    "avg_price": sum(prices) / len(prices) if prices else None,
    "min_price": min(prices) if prices else None,
    "max_price": max(prices) if prices else None
}

with open("/workspace/stats.json", "w") as f:
    json.dump(stats, f, indent=2)

print(f"Processed {stats['total']} products")
print(f"Average price: ${stats['avg_price']:.2f}")
```

### 5. 清理
```bash
verge-browser sandbox delete scraper
```

## 预期输出

```
./verge/data-extraction/
├── report.md
├── products.json           # 原始数据
├── products.csv            # CSV 格式
├── stats.json              # 统计信息
└── screenshots/
    ├── page-001.png
    ├── page-002.png
    └── ...
```
