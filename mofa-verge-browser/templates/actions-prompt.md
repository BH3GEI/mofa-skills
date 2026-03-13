# Verge Browser Actions API 任务模板

## 任务信息
- **沙箱别名**: {{ALIAS}}
- **Actions 文件**: {{ACTIONS_FILE}}

## Actions JSON 格式

```json
{
  "actions": [
    {
      "type": "goto",
      "url": "{{TARGET_URL}}"
    },
    {
      "type": "wait",
      "ms": 2000
    },
    {{#each ACTIONS}}
    {
      "type": "{{this.type}}",
      {{#if this.selector}}
      "selector": "{{this.selector}}",
      {{/if}}
      {{#if this.x}}
      "x": {{this.x}},
      "y": {{this.y}},
      {{/if}}
      {{#if this.text}}
      "text": "{{this.text}}",
      {{/if}}
      {{#if this.url}}
      "url": "{{this.url}}",
      {{/if}}
      {{#if this.output}}
      "output": "{{this.output}}",
      {{/if}}
      {{#if this.direction}}
      "direction": "{{this.direction}}",
      "amount": {{this.amount}}
      {{/if}}
    }{{#unless @last}},{{/unless}}
    {{/each}}
  ]
}
```

## 执行命令

```bash
verge-browser browser actions {{ALIAS}} --input {{ACTIONS_FILE}}
```

## Action 类型参考

| 类型 | 参数 | 描述 |
|-----|------|------|
| `goto` | `url` (string) | 导航到指定 URL |
| `click` | `selector` (string) 或 `x`, `y` (number) | 点击元素或坐标 |
| `type` | `selector` (string), `text` (string) | 输入文本 |
| `scroll` | `selector` (string), `direction` ("up"\|"down"), `amount` (number) | 滚动 |
| `wait` | `ms` (number) 或 `selector` (string) | 等待 |
| `screenshot` | `output` (string) | 截图 |
| `download` | `url` (string), `output` (string) | 下载文件 |

## 完整示例

```json
{
  "actions": [
    {
      "type": "goto",
      "url": "https://example.com/login"
    },
    {
      "type": "wait",
      "ms": 2000
    },
    {
      "type": "type",
      "selector": "#username",
      "text": "myusername"
    },
    {
      "type": "type",
      "selector": "#password",
      "text": "mypassword"
    },
    {
      "type": "click",
      "selector": "#login-btn"
    },
    {
      "type": "wait",
      "ms": 3000
    },
    {
      "type": "screenshot",
      "output": "/workspace/after-login.png"
    },
    {
      "type": "scroll",
      "direction": "down",
      "amount": 500
    },
    {
      "type": "screenshot",
      "output": "/workspace/scrolled.png"
    }
  ]
}
```

## 执行流程

1. **创建 Actions 文件**
   ```bash
   cat > actions.json << 'EOF'
   {{ACTIONS_JSON}}
   EOF
   ```

2. **执行 Actions**
   ```bash
   verge-browser browser actions {{ALIAS}} --input actions.json
   ```

3. **检查结果**
   ```bash
   ls -la /workspace/
   ```

## 错误处理

```json
{
  "actions": [
    {
      "type": "goto",
      "url": "https://example.com"
    },
    {
      "type": "wait",
      "selector": ".content",
      "timeout": 5000
    },
    {
      "type": "click",
      "selector": ".btn-primary",
      "retry": 3
    }
  ],
  "on_error": "continue"  // continue | abort | screenshot
}
```
