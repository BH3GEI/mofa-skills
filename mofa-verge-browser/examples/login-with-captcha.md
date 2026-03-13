# 示例: 登录需要验证码的网站

## 场景
需要登录某网站，但遇到了图形验证码，需要人工介入处理。

## 执行步骤

### 1. 创建沙箱
```bash
verge-browser sandbox create \
  --alias login-task \
  --width 1440 \
  --height 900
```

### 2. 自动化到验证码出现

**playwright_login.py:**
```python
import asyncio
from playwright.async_api import async_playwright

async def main():
    async with async_playwright() as p:
        browser = await p.chromium.connect_over_cdp(
            "ws://localhost:8000/sandbox/login-task/cdp"
        )
        page = await browser.new_page()

        # 访问登录页
        await page.goto("https://example.com/login")

        # 填写用户名密码
        await page.fill("#username", "myuser@example.com")
        await page.fill("#password", "mypassword")

        # 点击登录
        await page.click("#login-btn")

        # 等待验证码出现
        try:
            await page.wait_for_selector(".captcha-image", timeout=5000)
            print("检测到验证码，需要人工介入")

            # 获取接管 URL
            import subprocess
            result = subprocess.run(
                ["verge-browser", "sandbox", "session", "login-task"],
                capture_output=True, text=True
            )
            session_url = result.stdout.strip()

            print(f"请在浏览器中打开: {session_url}")
            print("完成验证码后按回车继续...")
            input()

        except:
            print("未检测到验证码，继续执行")

        # 继续后续操作
        await page.goto("https://example.com/dashboard")
        await page.screenshot(path="/workspace/dashboard.png")

        await browser.close()

asyncio.run(main())
```

### 3. 获取人工接管 URL (备用)
```bash
# 如果需要在浏览器中手动操作
verge-browser sandbox session login-task
```

### 4. 验证结果
```bash
# 检查截图
ls -la /workspace/dashboard.png

# 查看沙箱状态
verge-browser sandbox status login-task
```

### 5. 清理
```bash
verge-browser sandbox delete login-task
```

## 预期输出

```
./verge/login-task/
├── report.md              # 执行报告
├── screenshots/
│   ├── before-login.png   # 登录前
│   └── dashboard.png      # 登录后
└── logs/
    └── actions.log        # 操作日志
```
