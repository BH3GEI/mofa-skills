# Firecrawl Browser 任务模板

## 任务信息
- **任务类型**: browser
- **执行时间**: {{TIMESTAMP}}

## 会话管理

### 启动会话
```bash
firecrawl browser launch-session \
  {{#if TTL}}--ttl {{TTL}}{{/if}} \
  {{#if TTL_INACTIVITY}}--ttl-inactivity {{TTL_INACTIVITY}}{{/if}} \
  {{#if PROFILE}}--profile {{PROFILE}}{{/if}} \
  {{#if STREAM}}--stream{{/if}}
```

### 执行命令序列

{{#each COMMANDS}}
```bash
firecrawl browser execute "{{this}}"
```
{{/each}}

### 关闭会话
```bash
firecrawl browser close
```

## Python/Node 代码执行

### Python (Playwright)
```bash
firecrawl browser execute --python '
{{PYTHON_CODE}}
'
```

### Node.js (Playwright)
```bash
firecrawl browser execute --node '
{{NODE_CODE}}
'
```

## 常见操作

| 操作 | 命令 |
|-----|------|
| 打开页面 | `open <url>` |
| 获取快照 | `snapshot` |
| 点击元素 | `click @<ref>` |
| 填写输入 | `fill @<ref> '<text>'` |
| 抓取内容 | `scrape` |
| 返回 | `back` |
| 前进 | `forward` |
| 刷新 | `reload` |

## 输出处理

1. 保存 snapshot 结果
2. 保存 scrape 内容
3. 记录操作日志
