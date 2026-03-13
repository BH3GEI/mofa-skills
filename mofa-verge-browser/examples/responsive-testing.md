# 示例: 响应式页面测试

## 场景
测试网页在不同分辨率和设备下的显示效果。

## 执行步骤

### 1. 创建多个分辨率的沙箱

```bash
# 手机
verge-browser sandbox create --alias mobile --width 375 --height 667

# 平板
verge-browser sandbox create --alias tablet --width 768 --height 1024

# 桌面
verge-browser sandbox create --alias desktop --width 1920 --height 1080
```

### 2. 批量截图脚本

**capture_all.sh:**
```bash
#!/bin/bash

URL="https://example.com"
DEVICES=("mobile" "tablet" "desktop")

for device in "${DEVICES[@]}"; do
    echo "Capturing $device..."

    # 截图
    verge-browser browser screenshot $device \
        --output "./screenshots/$device-home.png"

done

echo "All screenshots captured!"
```

### 3. Playwright 详细测试

**responsive_test.py:**
```python
import asyncio
from playwright.async_api import async_playwright

DEVICES = [
    {"name": "mobile", "ws": "ws://localhost:8000/sandbox/mobile/cdp", "w": 375, "h": 667},
    {"name": "tablet", "ws": "ws://localhost:8000/sandbox/tablet/cdp", "w": 768, "h": 1024},
    {"name": "desktop", "ws": "ws://localhost:8000/sandbox/desktop/cdp", "w": 1920, "h": 1080},
]

async def test_device(device):
    async with async_playwright() as p:
        browser = await p.chromium.connect_over_cdp(device["ws"])

        # 设置视口
        context = await browser.new_context(
            viewport={"width": device["w"], "height": device["h"]}
        )
        page = await context.new_page()

        # 测试首页
        await page.goto("https://example.com")
        await page.screenshot(path=f"/workspace/{device['name']}-home.png")

        # 测试导航
        await page.click("nav a:first-child")
        await page.wait_for_load_state("networkidle")
        await page.screenshot(path=f"/workspace/{device['name']}-nav.png")

        # 测试表单
        await page.goto("https://example.com/contact")
        await page.screenshot(path=f"/workspace/{device['name']}-form.png")

        await browser.close()
        print(f"{device['name']} done")

async def main():
    await asyncio.gather(*[test_device(d) for d in DEVICES])

asyncio.run(main())
```

### 4. 生成对比报告

```python
# generate_report.py
from PIL import Image
import os

def create_comparison():
    images = []
    for device in ["mobile", "tablet", "desktop"]:
        path = f"./workspace/{device}-home.png"
        if os.path.exists(path):
            img = Image.open(path)
            images.append((device, img))

    # 创建对比图
    total_width = sum(img.width for _, img in images)
    max_height = max(img.height for _, img in images)

    comparison = Image.new('RGB', (total_width, max_height), 'white')

    x_offset = 0
    for name, img in images:
        comparison.paste(img, (x_offset, 0))
        x_offset += img.width

    comparison.save("./workspace/comparison.png")
    print("Comparison image saved!")

create_comparison()
```

### 5. 清理沙箱
```bash
verge-browser sandbox delete mobile
verge-browser sandbox delete tablet
verge-browser sandbox delete desktop
```

## 预期输出

```
./verge/responsive-test/
├── report.md
├── screenshots/
│   ├── mobile-home.png
│   ├── mobile-nav.png
│   ├── mobile-form.png
│   ├── tablet-home.png
│   ├── tablet-nav.png
│   ├── tablet-form.png
│   ├── desktop-home.png
│   ├── desktop-nav.png
│   └── desktop-form.png
└── comparison.png        # 对比图
```
