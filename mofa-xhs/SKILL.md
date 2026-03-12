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

## Prerequisites

Install xhs-cli:

```bash
uv tool install xhs-cli
# Or: pipx install xhs-cli
```

## Authentication

xhs-cli requires valid cookies to function.

```bash
# Check login status
xhs status

# Login (auto-extract Chrome cookies)
xhs login

# Or provide cookies manually
xhs login --cookie "a1=..."
```

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
