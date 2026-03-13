# Verge Browser Playwright 任务模板

## 任务信息
- **沙箱别名**: {{ALIAS}}
- **CDP URL**: {{CDP_URL}}
- **目标网站**: {{TARGET_URL}}

## Python 代码模板

```python
import asyncio
from playwright.async_api import async_playwright

async def main():
    async with async_playwright() as p:
        # 通过 CDP 连接到 Verge Browser
        browser = await p.chromium.connect_over_cdp(
            ws_endpoint="{{CDP_URL}}"
        )

        # 创建新上下文
        context = await browser.new_context(
            viewport={"width": {{WIDTH}}, "height": {{HEIGHT}}}
        )

        # 创建新页面
        page = await context.new_page()

        # 导航到目标网站
        await page.goto("{{TARGET_URL}}")
        await page.wait_for_load_state("networkidle")

        # ===== 自动化操作 =====
        {{#each ACTIONS}}
        {{this}}
        {{/each}}

        # 截图保存证据
        await page.screenshot(path="/workspace/screenshot-001.png")

        # 提取数据
        {{#if EXTRACT_DATA}}
        data = await page.evaluate('''() => {
            // 自定义提取逻辑
            return {
                title: document.title,
                url: window.location.href,
                // 添加更多字段
            }
        }''')
        print(f"Extracted: {data}")
        {{/if}}

        # 清理
        await context.close()
        await browser.close()

if __name__ == "__main__":
    asyncio.run(main())
```

## 常见操作代码片段

### 点击元素
```python
await page.click("{{SELECTOR}}")
```

### 填写表单
```python
await page.fill("{{SELECTOR}}", "{{VALUE}}")
```

### 等待元素
```python
await page.wait_for_selector("{{SELECTOR}}", timeout=10000)
```

### 提取文本
```python
text = await page.inner_text("{{SELECTOR}}")
```

### 提取属性
```python
href = await page.get_attribute("{{SELECTOR}}", "href")
```

### 处理新页面
```python
async with context.expect_page() as new_page_info:
    await page.click("a[target='_blank']")  # 点击打开新页面的链接
new_page = await new_page_info.value
await new_page.wait_for_load_state()
```

## 人工介入点

```python
# 检测到需要人工处理的情况
{{#if HITL_POINTS}}
{{#each HITL_POINTS}}
# 检查 {{this.description}}
if await page.query_selector("{{this.selector}}"):
    print("需要人工介入: {{this.description}}")
    print(f"接管 URL: {{this.session_url}}")
    input("完成后按回车继续...")
{{/each}}
{{/if}}
```

## 执行检查清单

- [ ] CDP 连接成功
- [ ] 页面加载完成
- [ ] 自动化步骤执行成功
- [ ] 数据已提取
- [ ] 截图已保存
- [ ] 浏览器正常关闭
