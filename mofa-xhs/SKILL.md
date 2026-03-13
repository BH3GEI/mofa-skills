---
name: mofa-xhs
description: "Xiaohongshu (小红书) integration — search notes, browse profiles, like/comment/publish via xhs-cli"
triggers: ["小红书", "xhs", "xiaohongshu", "rednote", "笔记"]
requires_bins: xhs
install_hint: "uv tool install xhs-cli"
always: false
---

# MOFA XHS (小红书)

Xiaohongshu (小红书, RedNote) integration via xhs-cli.

## Onboarding / 开始使用

### 前置要求

1. **安装 xhs-cli**
   ```bash
   # 使用 uv (推荐)
   uv tool install xhs-cli

   # 或使用 pipx
   pipx install xhs-cli

   # 或使用 pip
   pip install xhs-cli
   ```

2. **小红书账号**
   - 需要在 https://www.xiaohongshu.com 注册并登录过
   - 建议使用 Chrome 浏览器登录过小红书网页版

3. **认证登录** (三选一)

   **方式 A: 自动提取 Chrome cookies** (推荐)
   ```bash
   # 确保 Chrome 已登录小红书
   xhs login
   ```

   **方式 B: 手动提供 Cookie**
   ```bash
   # 从浏览器开发者工具复制 Cookie
   xhs login --cookie "a1=xxx; web_session=xxx; ..."
   ```

   **方式 C: 使用已保存的 Cookie 文件**
   ```bash
   # Cookie 将自动保存在 ~/.xhs/cookies.json
   xhs status
   ```

4. **验证安装**
   ```bash
   xhs status
   ```
   预期输出应显示 "Logged in as: [username]"

### 快速开始

```bash
# 搜索笔记
xhs search "咖啡" --limit 10

# 查看笔记详情
xhs read "https://www.xiaohongshu.com/discovery/item/xxx"

# 查看用户信息
xhs user "用户名"
```

### 故障排除

| 问题 | 解决方案 |
|-----|---------|
| `xhs: command not found` | 确保 `~/.local/bin` 在 PATH 中，或重新安装 |
| `Not logged in` | 运行 `xhs login`，确保 Chrome 已登录小红书 |
| `Cookie expired` | 重新运行 `xhs login` 刷新 Cookie |
| `Rate limited` | 降低请求频率，添加延迟参数 |
| 搜索无结果 | 检查关键词，尝试用中文关键词 |

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     XIAOHONGSHU (小红书) SKILL                              │
└─────────────────────────────────────────────────────────────────────────────┘

User Request
    ↓
┌─────────────────────────────────────────────────────────┐
│ Intent Parser                                           │
│ ────────────                                            │
│ • 搜索笔记 → Search Module                              │
│ • 查看详情 → Read Module                                │
│ • 用户资料 → User Module                                │
│ • 点赞评论 → Interact Module                            │
│ • 发布笔记 → Post Module                                │
└─────────────────────────┬───────────────────────────────┘
                          ↓
              ┌─────────────────────┐
              │   Auth Check        │
              │   xhs status        │
              └──────────┬──────────┘
                         ↓
            ┌──────────────────────┐
            │  xhs-cli             │
            │  (command wrapper)   │
            └──────────┬───────────┘
                       ↓
            ┌──────────────────────┐
            │  Xiaohongshu         │
            │  Platform            │
            └──────────────────────┘
```

## Features

### 1. Search (搜索)

```bash
# Search notes
xhs search "关键词"
xhs search "咖啡" --json

# Search topics/hashtags
xhs topics "旅行"
```

### 2. Read (查看)

```bash
# Read note details
xhs read <note_id>
xhs read <note_id> --comments

# Browse feed
xhs feed
```

### 3. User Profile (用户)

```bash
# User info
xhs user <user_id>

# User's posts
xhs user-posts <user_id>

# Followers/Following
xhs followers <user_id>
xhs following <user_id>
```

### 4. Interactions (互动)

```bash
# Like / Unlike
xhs like <note_id>
xhs like <note_id> --undo

# Favorite / Unfavorite
xhs favorite <note_id>
xhs favorite <note_id> --undo

# Comment
xhs comment <note_id> "评论内容"

# Delete own note
xhs delete <note_id>
```

### 5. Post (发布)

```bash
# Publish note
xhs post "标题" --image photo1.jpg --image photo2.jpg --content "正文"
```

### 6. Favorites (收藏)

```bash
xhs favorites
xhs favorites --max 10
```

### 7. Account (账户)

```bash
xhs status
xhs whoami
xhs logout
```

## JSON Output

All commands support `--json` for machine-readable output:

```bash
xhs search "咖啡" --json | jq '.[0].id'
xhs whoami --json | jq '.userInfo.userId'
xhs read <note_id> --json | jq '.title'
```

## Common Patterns

```bash
# Get your user ID
xhs whoami --json | jq -r '.userInfo.userId'

# Search and extract note IDs
xhs search "topic" --json | jq -r '.[].id'

# Check login before action
xhs status && xhs like <note_id>
```

## Error Handling

- Exit code 0 on success, non-zero on failure
- Error messages prefixed with ❌
- Login-required commands show clear instructions
- `xsec_token` auto-resolved from cache

## Safety Notes

- Do not share raw cookie values in chat logs
- Prefer `xhs login` over manual cookie input
- Re-login via `xhs login` if auth fails

## Real-World Testing Notes

Based on actual testing (2026-03-13, macOS, xhs-cli v0.1.4):

### Installation

```bash
uv tool install xhs-cli
```

**Note:** First run will auto-download ~306MB Camoufox browser (anti-fingerprint Firefox). Takes 1-3 minutes depending on network.

### Without Login (Anonymous)

| Command | Status | Notes |
|---------|--------|-------|
| `xhs feed` | ✅ Works | Returns 35 recommended items |
| `xhs search` | ❌ Requires login | Returns empty `[]` |
| `xhs topics` | ❌ Requires login | Returns empty |
| `xhs read` | ⚠️ Partial | Returns metadata but content may be empty |
| `xhs status` | ✅ Works | Shows "Not logged in" |

### After Login

Login required for full functionality:
- Search notes
- Like/Comment/Favorite
- View user profiles
- Post new notes
- Access favorites list

### Common Issues

1. **Camoufox download fails**
   - Check network connectivity to GitHub
   - Manual download: https://github.com/daijro/camoufox/releases

2. **Login cookie extraction fails**
   - Ensure Chrome is installed and has logged into xiaohongshu.com
   - Alternative: Use `xhs login` without Chrome to get QR code

3. **Search returns empty**
   - Normal if not logged in
   - After login, should return results

4. **Rate limiting**
   - Xiaohongshu has strict rate limits
   - Add delays between requests
   - Avoid rapid-fire operations

### Quick Test After Installation

```bash
# Test without login
xhs feed

# Should see table output with ~35 recommended notes
# If this works, installation is successful
```
