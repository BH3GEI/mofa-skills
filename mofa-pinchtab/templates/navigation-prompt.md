# PinchTab Navigation 任务模板

## 任务信息
- **目标 URL**: {{URL}}
- **等待时间**: {{WAIT}}ms
- **会话**: {{SESSION}}

## 执行命令

### CLI 方式
```bash
# 基础导航
pinchtab nav "{{URL}}"

# 带等待时间
pinchtab nav "{{URL}}" --wait {{WAIT}}

# 特定会话
pinchtab nav "{{URL}}" --session {{SESSION}} --wait {{WAIT}}
```

### HTTP API 方式
```bash
curl -X POST "{{PINCHTAB_URL}}/navigate" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "{{URL}}",
    "wait": {{WAIT}}
  }'
```

## 导航后检查清单

- [ ] 页面加载完成 (200 OK)
- [ ] URL 正确
- [ ] 标题可获取
- [ ] 无错误弹窗

## 获取页面信息

```bash
# 获取当前 URL
pinchtab url

# 获取页面标题
pinchtab title

# 获取完整信息
pinchtab info
```

## 输出示例

```json
{
  "url": "{{URL}}",
  "title": "Page Title",
  "status": "loaded",
  "load_time": 1200
}
```
