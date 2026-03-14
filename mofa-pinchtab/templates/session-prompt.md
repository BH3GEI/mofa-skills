# PinchTab Session 任务模板

## 任务信息
- **会话名称**: {{SESSION_NAME}}
- **模式**: {{MODE}}
- **Headless**: {{HEADLESS}}

## 启动服务

### 守护进程模式 (推荐)
```bash
# 安装
pinchtab daemon install

# 启动
pinchtab daemon start

# 查看状态
pinchtab daemon status

# 停止
pinchtab daemon stop
```

### 服务器模式
```bash
# 前台运行
pinchtab server

# 指定端口
pinchtab server --port 9867
```

### 桥接模式
```bash
# 单实例
pinchtab bridge

# 有头模式 (可见)
pinchtab bridge --headed
```

## 多会话管理

### 创建会话
```bash
curl -X POST "{{PINCHTAB_URL}}/session" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "{{SESSION_NAME}}",
    "headless": {{HEADLESS}},
    "proxy": null
  }'
```

### 使用会话
```bash
# CLI 中指定会话
pinchtab nav "https://example.com" --session {{SESSION_NAME}}
pinchtab text --session {{SESSION_NAME}}

# API 中使用会话端点
curl -X POST "{{PINCHTAB_URL}}/session/{{SESSION_NAME}}/navigate" \
  -d '{"url": "https://example.com"}'
```

### 列出会话
```bash
curl "{{PINCHTAB_URL}}/sessions"
```

### 删除会话
```bash
curl -X DELETE "{{PINCHTAB_URL}}/session/{{SESSION_NAME}}"
```

## 并行会话示例

```python
# parallel_sessions.py
import asyncio
import aiohttp

async def create_session(http, name):
    """创建新会话"""
    await http.post("http://localhost:9867/session", json={
        "name": name,
        "headless": True
    })

async def process_in_session(session_name, url):
    """在会话中处理任务"""
    async with aiohttp.ClientSession() as http:
        base = f"http://localhost:9867/session/{session_name}"

        await http.post(f"{base}/navigate", json={"url": url})

        async with http.post(f"{base}/extract") as resp:
            return await resp.text()

async def delete_session(http, name):
    """删除会话"""
    await http.delete(f"http://localhost:9867/session/{name}")

# 使用示例
async def main():
    urls = ["https://site1.com", "https://site2.com", "https://site3.com"]

    async with aiohttp.ClientSession() as http:
        # 创建会话
        for i, url in enumerate(urls):
            await create_session(http, f"task-{i}")

        # 并行处理
        tasks = [
            process_in_session(f"task-{i}", url)
            for i, url in enumerate(urls)
        ]
        results = await asyncio.gather(*tasks)

        # 清理
        for i in range(len(urls)):
            await delete_session(http, f"task-{i}")

    return results

asyncio.run(main())
```
