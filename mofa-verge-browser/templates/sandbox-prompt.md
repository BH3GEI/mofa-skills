# Verge Browser Sandbox 任务模板

## 任务信息
- **沙箱别名**: {{ALIAS}}
- **分辨率**: {{WIDTH}}x{{HEIGHT}}
- **运行时**: {{RUNTIME}}
- **创建时间**: {{TIMESTAMP}}

## 执行命令

### 创建沙箱
```bash
verge-browser sandbox create \
  --alias {{ALIAS}} \
  --width {{WIDTH}} \
  --height {{HEIGHT}} \
  {{#if RUNTIME}}--runtime {{RUNTIME}}{{/if}} \
  {{#if JSON}}--json{{/if}}
```

### 获取 Session URL (人工接管)
```bash
verge-browser sandbox session {{ALIAS}}
```

### 沙箱生命周期管理
```bash
# 查看状态
verge-browser sandbox status {{ALIAS}}

# 暂停
verge-browser sandbox pause {{ALIAS}}

# 恢复
verge-browser sandbox resume {{ALIAS}}

# 删除
verge-browser sandbox delete {{ALIAS}}
```

## 沙箱配置

- **Websocket (CDP)**: `ws://{{API_URL}}/sandbox/{{SANDBOX_ID}}/cdp`
- **noVNC URL**: `http://{{API_URL}}/sandbox/{{SANDBOX_ID}}/vnc`
- **工作目录**: `/workspace`

## 检查清单

- [ ] 沙箱创建成功
- [ ] CDP WebSocket 可连接
- [ ] Session URL 可访问
- [ ] /workspace 可读写
- [ ] 浏览器正常启动

## 输出

```json
{
  "sandbox_id": "{{SANDBOX_ID}}",
  "alias": "{{ALIAS}}",
  "status": "running",
  "cdp_url": "ws://{{API_URL}}/sandbox/{{SANDBOX_ID}}/cdp",
  "vnc_url": "http://{{API_URL}}/sandbox/{{SANDBOX_ID}}/vnc",
  "created_at": "{{TIMESTAMP}}"
}
```
