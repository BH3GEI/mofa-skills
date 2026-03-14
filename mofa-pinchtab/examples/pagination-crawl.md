# 示例: 分页内容抓取

## 场景
抓取分页列表的内容，自动翻页直到最后一页。

## 执行步骤

### Bash 版本

**crawl_pagination.sh:**
```bash
#!/bin/bash

BASE_URL="https://example.com/items"
OUTPUT_DIR="./pages"
PAGE=1

mkdir -p "$OUTPUT_DIR"

# 第一页
pinchtab nav "${BASE_URL}?page=1" --wait 3000

while true; do
    echo "Processing page $PAGE..."

    # 提取内容
    pinchtab text > "$OUTPUT_DIR/page-${PAGE}.txt"

    # 截图
    pinchtab snap -o "$OUTPUT_DIR/page-${PAGE}.png"

    # 获取结构找下一页按钮
    STRUCTURE=$(pinchtab snap -i -c)

    # 检查是否有下一页
    if ! echo "$STRUCTURE" | grep -q "ref-next"; then
        echo "No more pages"
        break
    fi

    # 点击下一页
    pinchtab click @ref-next --wait 3000
    PAGE=$((PAGE + 1))

    # 限制最大页数
    if [ $PAGE -gt 10 ]; then
        echo "Reached max pages"
        break
    fi
done

echo "Crawled $PAGE pages"
```

### Python 版本

**crawl_pagination.py:**
```python
import requests
import time

PINCHTAB_URL = "http://localhost:9867"
OUTPUT_DIR = "./pages"
MAX_PAGES = 10

def crawl_pages(start_url):
    page = 1
    results = []

    # 导航到第一页
    requests.post(f"{PINCHTAB_URL}/navigate", json={
        "url": start_url, "wait": 3000
    })

    while page <= MAX_PAGES:
        print(f"Processing page {page}...")

        # 提取文本
        text = requests.post(f"{PINCHTAB_URL}/extract").text
        results.append({"page": page, "text": text})

        # 保存
        with open(f"{OUTPUT_DIR}/page-{page}.txt", "w") as f:
            f.write(text)

        # 获取结构
        snapshot = requests.post(f"{PINCHTAB_URL}/snapshot", json={
            "interactive": True, "compact": True
        }).json()

        # 查找下一页按钮
        has_next = False
        for item in snapshot.get("elements", []):
            if "next" in item.get("text", "").lower():
                has_next = True
                # 点击下一页
                requests.post(f"{PINCHTAB_URL}/click", json={
                    "ref": item["ref"], "wait": 3000
                })
                break

        if not has_next:
            print("No more pages")
            break

        page += 1
        time.sleep(1)

    return results

# 使用
results = crawl_pages("https://example.com/items?page=1")
print(f"Crawled {len(results)} pages")
```

## 预期输出

```
./pinchtab/pagination-crawl/
├── report.md
└── pages/
    ├── page-1.txt
    ├── page-1.png
    ├── page-2.txt
    ├── page-2.png
    └── ...
```
