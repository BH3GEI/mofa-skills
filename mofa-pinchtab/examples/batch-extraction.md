# 示例: 批量文章提取

## 场景
从多个博客文章 URL 中提取纯文本内容，用于 AI 分析。

## 执行步骤

### 1. 准备 URL 列表
```bash
# urls.txt
https://example.com/blog/post-1
https://example.com/blog/post-2
https://example.com/blog/post-3
```

### 2. 创建提取脚本

**batch_extract.sh:**
```bash
#!/bin/bash

INPUT_FILE="urls.txt"
OUTPUT_DIR="./extracted"
mkdir -p "$OUTPUT_DIR"

# 确保服务运行
if ! curl -s http://localhost:9867/health > /dev/null; then
    echo "Starting pinchtab daemon..."
    pinchtab daemon start
    sleep 2
fi

# 逐条处理
while IFS= read -r url; do
    filename=$(echo "$url" | sed 's/[^a-zA-Z0-9]/-/g')
    echo "Extracting: $url"

    # 导航
    pinchtab nav "$url" --wait 3000

    # 提取文本
    pinchtab text > "$OUTPUT_DIR/$filename.txt"

done < "$INPUT_FILE"

echo "Extraction complete! Files in $OUTPUT_DIR"
```

### 3. Python 并行版本

**batch_extract.py:**
```python
import asyncio
import aiohttp
import os

PINCHTAB_URL = "http://localhost:9867"
OUTPUT_DIR = "./extracted"

URLS = [
    "https://example.com/blog/post-1",
    "https://example.com/blog/post-2",
    "https://example.com/blog/post-3",
]

async def extract_url(http, url, index):
    session_name = f"extract-{index}"

    try:
        await http.post(f"{PINCHTAB_URL}/session", json={
            "name": session_name, "headless": True
        })

        base = f"{PINCHTAB_URL}/session/{session_name}"
        await http.post(f"{base}/navigate", json={"url": url, "wait": 3000})

        async with http.post(f"{base}/extract") as resp:
            text = await resp.text()

        filename = url.replace('/', '-').replace(':', '')
        with open(f"{OUTPUT_DIR}/{filename}.txt", 'w') as f:
            f.write(text)

        return {"url": url, "chars": len(text)}

    finally:
        await http.delete(f"{PINCHTAB_URL}/session/{session_name}")

asyncio.run(main())
```

## 预期输出

```
./pinchtab/batch-extract/
├── report.md
└── extracted/
    ├── https---example.com-blog-post-1.txt
    ├── https---example.com-blog-post-2.txt
    └── https---example.com-blog-post-3.txt
```
