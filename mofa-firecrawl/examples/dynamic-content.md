# 示例: 动态内容抓取

## 场景
抓取需要 JavaScript 渲染或登录后才能访问的内容。

## 执行步骤

### 1. 启动浏览器会话
```bash
firecrawl browser launch-session --ttl 900
```

### 2. 执行登录流程
```bash
# 打开登录页
firecrawl browser execute "open https://app.example.com/login"

# 填写表单
firecrawl browser execute "fill @email 'user@example.com'"
firecrawl browser execute "fill @password 'your-password'"

# 点击登录
firecrawl browser execute "click @login-button"
```

### 3. 导航到目标页面
```bash
firecrawl browser execute "open https://app.example.com/dashboard"
firecrawl browser execute "scrape" -o ./firecrawl/dynamic/dashboard.md
```

### 4. 处理分页或懒加载
```bash
# Python 脚本翻页
firecrawl browser execute --python '
await page.goto("https://app.example.com/items")
all_items = []

while True:
    # 获取当前页内容
    items = await page.query_selector_all(".item")
    for item in items:
        text = await item.text_content()
        all_items.append(text)

    # 点击下一页
    next_btn = await page.query_selector(".next-page")
    if not next_btn or not await next_btn.is_visible():
        break
    await next_btn.click()
    await page.wait_for_timeout(2000)

print(f"Total items: {len(all_items)}")
'
```

### 5. 关闭会话
```bash
firecrawl browser close
```

## 预期输出
```
./firecrawl/dynamic/
├── report.md              # 抓取报告
├── dashboard.md           # 仪表板内容
└── paginated-data.json    # 分页数据
```
