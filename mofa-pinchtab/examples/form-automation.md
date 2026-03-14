# 示例: 表单自动化

## 场景
自动填写并提交网页表单。

## 执行步骤

### 1. 导航到表单页面
```bash
pinchtab nav "https://example.com/contact" --wait 3000
```

### 2. 获取表单结构
```bash
STRUCTURE=$(pinchtab snap -i -c)
echo "$STRUCTURE"
```

输出示例:
```
[ref-1] <input type="text" name="name" placeholder="Your name" />
[ref-2] <input type="email" name="email" placeholder="Your email" />
[ref-3] <textarea name="message" placeholder="Message"></textarea>
[ref-4] <button type="submit">Send</button>
```

### 3. 填写表单
```bash
# 填写姓名
pinchtab type @ref-1 "John Doe"

# 填写邮箱
pinchtab type @ref-2 "john@example.com"

# 填写消息
pinchtab type @ref-3 "Hello, this is a test message."
```

### 4. 提交表单
```bash
# 点击提交
pinchtab click @ref-4 --wait 3000

# 检查结果
pinchtab text
```

### 5. Python 完整脚本

**form_automation.py:**
```python
import requests
import time

PINCHTAB_URL = "http://localhost:9867"

# 表单数据
FORM_DATA = {
    "name": "John Doe",
    "email": "john@example.com",
    "message": "Test message"
}

# 导航
requests.post(f"{PINCHTAB_URL}/navigate", json={
    "url": "https://example.com/contact",
    "wait": 3000
})

# 获取结构
snapshot = requests.post(f"{PINCHTAB_URL}/snapshot", json={
    "interactive": True, "compact": True
}).json()

print("Form structure:", snapshot)

# 根据结构填写 (这里假设引用为 ref-1, ref-2, ref-3)
requests.post(f"{PINCHTAB_URL}/type", json={
    "ref": "ref-1", "text": FORM_DATA["name"]
})
requests.post(f"{PINCHTAB_URL}/type", json={
    "ref": "ref-2", "text": FORM_DATA["email"]
})
requests.post(f"{PINCHTAB_URL}/type", json={
    "ref": "ref-3", "text": FORM_DATA["message"]
})

# 提交
requests.post(f"{PINCHTAB_URL}/click", json={
    "ref": "ref-4", "wait": 3000
})

# 检查结果
result = requests.post(f"{PINCHTAB_URL}/extract").text
print("Result:", result)
```

## 预期输出

```
./pinchtab/form-automation/
├── report.md
└── log.txt
```
