# PinchTab Interaction 任务模板

## 任务信息
- **元素引用**: {{REF}}
- **操作类型**: {{ACTION}}
- **输入文本**: {{TEXT}}

## 执行命令

### 点击元素
```bash
# 点击引用元素
pinchtab click @{{REF}}

# 点击后等待
pinchtab click @{{REF}} --wait 2000
```

### 输入文本
```bash
# 在输入框中输入
pinchtab type @{{REF}} "{{TEXT}}"

# 带延迟 (模拟人类输入)
pinchtab type @{{REF}} "{{TEXT}}" --delay 50
```

### 滚动页面
```bash
# 向下滚动
pinchtab scroll down

# 指定像素
pinchtab scroll down 500

# 向上滚动
pinchtab scroll up 300
```

### HTTP API 方式
```bash
# 点击
curl -X POST "{{PINCHTAB_URL}}/click" \
  -H "Content-Type: application/json" \
  -d '{
    "ref": "{{REF}}",
    "wait": 2000
  }'

# 输入
curl -X POST "{{PINCHTAB_URL}}/type" \
  -H "Content-Type: application/json" \
  -d '{
    "ref": "{{REF}}",
    "text": "{{TEXT}}",
    "delay": 50
  }'

# 滚动
curl -X POST "{{PINCHTAB_URL}}/scroll" \
  -H "Content-Type: application/json" \
  -d '{
    "direction": "down",
    "amount": 500
  }'
```

## 获取元素引用

```bash
# 1. 先获取快照
SNAPSHOT=$(pinchtab snap -i -c)
echo "$SNAPSHOT"

# 2. 输出示例:
# [ref-1] <input type="text" name="search" />
# [ref-2] <button>Search</button>
# [ref-3] <a href="/about">About</a>

# 3. 使用引用操作
pinchtab type @ref-1 "search query"
pinchtab click @ref-2
```

## 常见交互模式

### 表单填写
```bash
# 获取表单结构
pinchtab snap -i -c

# 填写各字段
pinchtab type @ref-name "John Doe"
pinchtab type @ref-email "john@example.com"
pinchtab type @ref-phone "1234567890"

# 提交
pinchtab click @ref-submit --wait 3000

# 检查结果
pinchtab text
```

### 分页浏览
```bash
#!/bin/bash
for i in {1..5}; do
  echo "Page $i"
  pinchtab text > "./content/page-$i.txt"

  # 获取结构找下一页按钮
  STRUCTURE=$(pinchtab snap -i -c)

  # 点击下一页
  if echo "$STRUCTURE" | grep -q "ref-next"; then
    pinchtab click @ref-next --wait 2000
  else
    break
  fi
done
```
