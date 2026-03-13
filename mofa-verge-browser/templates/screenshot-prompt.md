# Verge Browser 截图任务模板

## 任务信息
- **沙箱别名**: {{ALIAS}}
- **输出路径**: {{OUTPUT_PATH}}
- **截图类型**: {{TYPE}}

## 执行命令

### 基础截图
```bash
verge-browser browser screenshot {{ALIAS}} --output {{OUTPUT_PATH}}
```

### 区域截图
```bash
verge-browser browser screenshot {{ALIAS}} \
  --x {{X}} \
  --y {{Y}} \
  --width {{WIDTH}} \
  --height {{HEIGHT}} \
  --output {{OUTPUT_PATH}}
```

### 完整窗口截图
```bash
verge-browser browser screenshot {{ALIAS}} \
  --full-window \
  --output {{OUTPUT_PATH}}
```

## 截图策略

### 1. 关键节点自动截图

```bash
# 访问前
verge-browser browser screenshot {{ALIAS}} --output ./screenshots/01-before-goto.png

# 操作后
verge-browser browser screenshot {{ALIAS}} --output ./screenshots/02-after-login.png

# 完成时
verge-browser browser screenshot {{ALIAS}} --output ./screenshots/03-final.png
```

### 2. 对比截图

```bash
#!/bin/bash
# 连续截图用于对比

for i in {1..5}; do
  verge-browser browser screenshot {{ALIAS}} \
    --output "./screenshots/step-$(printf %03d $i).png"
  sleep 2
done
```

### 3. 全页面截图 (通过 Playwright)

```python
# full_page_screenshot.py
import asyncio
from playwright.async_api import async_playwright

async def main():
    async with async_playwright() as p:
        browser = await p.chromium.connect_over_cdp("{{CDP_URL}}")
        page = await browser.new_page()
        await page.goto("{{URL}}")

        # 全页面截图
        await page.screenshot(
            path="{{OUTPUT_PATH}}",
            full_page=True
        )

        await browser.close()

asyncio.run(main())
```

## 截图检查清单

- [ ] 截图文件已生成
- [ ] 截图内容符合预期
- [ ] 分辨率正确
- [ ] 关键信息可见
- [ ] 文件大小合理 (非空文件)

## 截图分析

```python
from PIL import Image
import os

def analyze_screenshot(path):
    img = Image.open(path)
    return {
        "size": os.path.getsize(path),
        "dimensions": img.size,
        "mode": img.mode,
        "format": img.format
    }

info = analyze_screenshot("{{OUTPUT_PATH}}")
print(f"截图信息: {info}")
```
