# 邮箱 (mail) 命令参考

## 命令总览

### 查询可用邮箱地址
```
Usage:
  dws mail mailbox list [flags]
Example:
  dws mail mailbox list
```

### 搜索邮件 (KQL 语法)
```
Usage:
  dws mail message search [flags]
Example:
  dws mail message search --email user@company.com --query "subject:\"周报\"" --size 20
  dws mail message search --email user@company.com --query "from:alice AND date>2025-06-01T00:00:00Z" --size 10
Flags:
      --cursor string   分页游标 (首页留空)
      --email string    搜索目标邮箱地址 (必填)
      --query string    KQL 查询表达式 (必填)
      --size string     每页返回数量 1-100 (默认 20)
```

KQL 查询字段: date, size, tag, folderId, isRead, hasAttachments, subject, attachname, body, from, to
常用文件夹 ID: 1=已发送, 2=收件箱, 3=垃圾邮件, 5=草稿, 6=已删除

### 查看邮件完整内容
```
Usage:
  dws mail message get [flags]
Example:
  dws mail message get --email user@company.com --id <messageId>
Flags:
      --email string   邮件所属邮箱地址 (必填)
      --id string      邮件 ID (必填)
```

### 发送邮件
```
Usage:
  dws mail message send [flags]
Example:
  dws mail message send --from user@company.com --to colleague@company.com \
    --subject "周报" --body "本周完成任务A和任务B"
Flags:
      --body string      邮件正文 (必填)
      --cc string        抄送人列表
      --from string      发件人邮箱 (必填)
      --subject string   邮件标题 (必填)
      --to string        收件人列表 (必填)
```

## 意图判断

用户说"我的邮箱/邮箱地址" → `mailbox list`
用户说"找邮件/搜邮件/查邮件" → `message search`
用户说"看邮件/打开邮件/邮件内容" → 先 `message search` 获取 messageId，再 `message get`
用户说"发邮件/写邮件" → 先 `mailbox list` 获取发件地址，再 `message send`
用户说"给(某人名字)发邮件" → 先 `contact user search` 获取收件人邮箱，再 `message send`

## 核心工作流

```bash
# 1. 查看可用邮箱 — 提取邮箱地址
dws mail mailbox list --format json

# 2. 搜索邮件 — 提取 messageId
dws mail message search --email user@company.com \
  --query "subject:\"周报\" AND date>2025-06-01T00:00:00Z" --size 10 --format json

# 3. 查看邮件详情
dws mail message get --email user@company.com --id <messageId> --format json

# 4. 发送邮件
dws mail message send --from user@company.com --to colleague@company.com \
  --subject "周报" --body "本周完成…" --format json
```

## 上下文传递表

| 操作 | 从返回中提取 | 用于 |
|------|-------------|------|
| `mailbox list` | 邮箱地址 | message search/get/send 的 --email/--from |
| `message search` | `messageId` | message get 的 --id |
| `contact user search` | 用户邮箱 (email) | message send 的 --to/--cc (跨产品) |

## 注意事项

- `mailbox list` 返回用户所有邮箱（含个人和企业），每条记录包含邮箱地址、账号类型、所属企业。选择邮箱时优先匹配用户当前所在企业的企业邮箱；若有多个可选，向用户确认后再操作
- `message search` 返回邮件 ID 和元信息（不含正文），需 `message get` 获取完整内容
- KQL 查询支持 AND/OR/NOT 组合，字段值含空格时需用双引号
- `--cc` 抄送人支持多人，逗号分隔
- 收件人邮箱获取：用户只知道同事名字时，先通过 `dws contact user search --keyword "名字"` 查询，从返回中提取 orgAuthEmail 字段

## 自动化脚本

| 脚本 | 场景 | 用法 |
|------|------|------|
| [mail_unread_summary.py](../../scripts/mail_unread_summary.py) | 查询今天未读邮件汇总 | `python mail_unread_summary.py` |
| [mail_send_with_cc.py](../../scripts/mail_send_with_cc.py) | 发送带抄送的邮件（自动获取发件地址） | `python mail_send_with_cc.py --to a@b.com --cc c@d.com --subject "周报" --body "内容"` |

