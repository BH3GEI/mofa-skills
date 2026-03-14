# PinchTab Extraction 任务模板

## 任务信息
- **提取类型**: {{TYPE}}
- **选择器**: {{SELECTOR}}
- **会话**: {{SESSION}}

## 执行命令

### 纯文本提取 (推荐!)
```bash
# 提取整页文本 (最省 token)
pinchtab text

# 提取特定区域
pinchtab text --selector "{{SELECTOR}}"
```

### 页面快照
```bash
# 获取结构 (带元素引用)
pinchtab snap -i -c

# 完整快照
pinchtab snap

# 保存截图
pinchtab snap -o ./screenshots/{{TIMESTAMP}}.png
```

### HTTP API 方式
```bash
# 文本提取
curl -X POST "{{PINCHTAB_URL}}/extract" \
  -H "Content-Type: application/json" \
  -d '{
    "format": "text",
    "selector": "{{SELECTOR}}"
  }'

# 快照
curl -X POST "{{PINCHTAB_URL}}/snapshot" \
  -H "Content-Type: application/json" \
  -d '{
    "interactive": true,
    "compact": true
  }'
```

## 提取流程

```
导航到页面
    ↓
获取快照 (获取元素引用)
    ↓
提取文本内容
    ↓
保存结果
    ↓
[可选] 点击交互元素继续
```

## 输出处理

```bash
# 保存文本到文件
pinchtab text > ./content/page-{{ID}}.txt

# 同时保存结构和文本
pinchtab snap -i -c > ./content/page-{{ID}}-structure.txt
pinchtab text > ./content/page-{{ID}}-content.txt
```

## Token 效率提示

- `pinchtab text`: ~800 tokens/页
- `pinchtab snap -i -c`: ~1200 tokens/页
- 截图: ~5000-10000 tokens/页

**建议**: 优先使用 `text` 提取，需要交互信息时用 `snap -i -c`
