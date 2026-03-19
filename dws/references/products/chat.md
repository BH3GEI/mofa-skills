# 会话与群聊 (chat) 命令参考

## 命令总览

### group (群组管理)

#### 创建群 — 当前登录用户自动成为群主
```
Usage:
  dws chat group create [flags]
Example:
  dws chat group create --name "Q1 项目冲刺群" --users userId1,userId2,userId3
Flags:
      --users string    成员 userId 列表，用户本身会自动加入，无需包含，逗号分隔 (必填)
      --name string     群名称 (必填)
```

#### 查看群成员列表 — 分页查询指定群聊的成员
```
Usage:
  dws chat group members [flags]
Example:
  dws chat group members --id <openconversation_id>
Flags:
      --cursor string   分页游标，首次从 0 开始
      --id string       群 ID / openconversation_id (必填)
```

#### 添加群成员 — 向指定群聊添加成员，需传入群 ID 与用户 ID 列表
```
Usage:
  dws chat group members add [flags]
Example:
  dws chat group members add --id <openconversation_id> --users userId1,userId2
Flags:
      --id string      群 ID / openconversation_id (必填)
      --users string   要添加的用户 userId 列表，逗号分隔 (必填)
```

#### 移除群成员 — 从指定群聊中移除成员，需传入群 ID 与待移除的用户 ID 列表
```
Usage:
  dws chat group members remove [flags]
Example:
  dws chat group members remove --id <openconversation_id> --users userId1,userId2
Flags:
      --id string      群 ID / openconversation_id (必填)
      --users string   要移除的用户 userId 列表，逗号分隔 (必填)
```

#### 将机器人添加到群中 — 将自定义机器人添加到当前用户有管理权限的群聊中，如果没有权限则会报错
```
Usage:
  dws chat group members add-bot [flags]
Example:
  dws chat group members add-bot --robot-code <robot-code> --id <openconversation_id>
Flags:
      --id string           群聊 openConversationId (必填)
      --robot-code string   机器人 Code (必填)
```

#### 更新群名称
```
Usage:
  dws chat group rename [flags]
Example:
  dws chat group rename --id <openconversation_id> --name "新群名"
Flags:
      --id string     群 ID / openconversation_id (必填)
      --name string   修改后的群名称 (必填)
```

### search (搜索会话)

#### 根据名称搜索会话列表
```
Usage:
  dws chat search [flags]
Example:
  dws chat search --query "项目冲刺"
Flags:
      --cursor string   分页游标 (首页留空)
      --query string    搜索关键词 (必填)
```

### message (会话消息管理)

#### 拉取会话消息内容 — 拉取指定群聊或单聊的会话消息内容

--group 指定群聊，--user 指定单聊用户，二者互斥。默认拉取给定时间之后的消息，--forward=false 拉之前的。hasMore=true 时用结果中的边界 createTime 作为下次 --time 翻页。
```
Usage:
  dws chat message list [flags]
Example:
  dws chat message list --group <openconversation_id> --time "2025-03-01 00:00:00"
  dws chat message list --user <userId> --time "2025-03-01 00:00:00" --limit 50
  dws chat message list --group <openconversation_id> --time "2025-03-01 00:00:00" --forward=false
Flags:
      --forward        true=拉给定时间之后的消息，false=拉给定时间之前的消息 (default true)
      --group string   群聊 openconversation_id（群聊时必填）
      --limit int      返回数量，不传则不限制
      --time string    开始时间，格式: yyyy-MM-dd HH:mm:ss (必填)
      --user string    单聊用户 userId（单聊时必填）

注意:
  - --group 与 --user 互斥，必须且只能指定其一
  - 翻页：hasMore=true 时，用结果中的边界 createTime 作为下次 --time
```

#### 以当前用户身份发送消息 — --group 群聊 / --user 单聊

--group 指定群聊 ID 发群消息；--user 指定用户 ID 发单聊，二者只能选其一，不能同时指定。消息内容为位置参数（恰好 1 个），支持 Markdown。可选 --title 作为消息标题。
```
Usage:
  dws chat message send [flags] <text>
Example:
  dws chat message send --group <openconversation_id> "hello"
  dws chat message send --user <userId> "请查收"
  dws chat message send --group <openconversation_id> --title "周报提醒" "请大家本周五前提交周报"
Flags:
      --group string   群聊 openconversation_id（群聊时必填）
      --user string    接收人 userId（单聊时必填）
      --title string   消息标题（可选，默认「消息」）

注意:
  - text 为位置参数（恰好 1 个），直接跟在 flags 后面，不需要 --text
  - --group 与 --user 互斥，必须且只能指定其一
```

#### 机器人发送消息（--group 群聊 / --users 单聊）

群聊：传 --group 指定群；单聊：传 --users 指定用户列表，二者只能选其一，不能同时指定。--text 支持 Markdown。
```
Usage:
  dws chat message send-by-bot [flags]
Example:
  dws chat message send-by-bot --robot-code <robot-code> --group <openconversation_id> --title "日报" --text "## 今日完成..."
  dws chat message send-by-bot --robot-code <robot-code> --users userId1,userId2 --title "提醒" --text "请提交周报"
Flags:
      --group string        群聊 openConversationId（群聊时必填）
      --robot-code string   机器人 Code (必填)
      --text string         消息内容 Markdown (必填)
      --title string        消息标题 (必填)
      --users string        用户 userId 列表，逗号分隔，最多20个（单聊时必填）

注意: --group 与 --users 互斥，必须且只能指定其一。
```

#### 机器人撤回消息（--group 群聊 / 不传为单聊）

群聊：传 --group 与 --keys；单聊：仅传 --keys。--keys 为发送时返回的 processQueryKey 列表，逗号分隔。
```
Usage:
  dws chat message recall-by-bot [flags]
Example:
  dws chat message recall-by-bot --robot-code <robot-code> --group <openconversation_id> --keys <process-query-key>
  dws chat message recall-by-bot --robot-code <robot-code> --keys key1,key2
Flags:
      --group string         群聊 openConversationId（群聊撤回时必填）
      --keys string         消息 processQueryKey 列表，逗号分隔 (必填)
      --robot-code string   机器人 Code (必填)
```

#### 自定义机器人 Webhook 发送群消息

@ 人时需在 --text 中包含 @userId 或 @手机号，否则 @ 不生效。
```
Usage:
  dws chat message send-by-webhook [flags]
Example:
  dws chat message send-by-webhook --token <webhook-token> --title "告警" --text "CPU 超 90%" --at-all
  dws chat message send-by-webhook --token <webhook-token> --title "test" --text "hi @034766" --at-users 034766
Flags:
      --at-all              @ 所有人
      --at-mobiles string   @ 指定手机号，逗号分隔
      --at-users string     @ 指定用户，逗号分隔（需在 text 中包含 @userId）
      --text string         消息内容 (必填)
      --title string        消息标题 (必填)
      --token string        Webhook Token (必填)
```

### bot (机器人管理)

#### 搜索我创建的机器人
```
Usage:
  dws chat bot search [flags]
Example:
  dws chat bot search --page 1
  dws chat bot search --page 1 --size 10 --name "日报"
Flags:
      --name string   按名称搜索
      --page int      页码，从1开始 (默认 1)
      --size int      每页条数 (默认 50)
```

## 意图判断

用户说"建群/创建群聊" → `chat group create`
用户说"搜索群/找群" → `chat search`
用户说"群成员/看群里有谁" → `chat group members`
用户说"拉人进群/加群成员" → `chat group members add`
用户说"踢人/移除群成员" → `chat group members remove`
用户说"加机器人到群" → `chat group members add-bot`
用户说"改群名" → `chat group rename`
用户说"聊天记录/会话消息/拉取会话" → `chat message list`
用户说"发群消息(以个人身份)" → `chat message send --group`
用户说"发单聊消息(以个人身份)" → `chat message send --user`
用户说"机器人发消息/机器人群发" → `chat message send-by-bot`
用户说"机器人撤回消息" → `chat message recall-by-bot`
用户说"Webhook 发消息/告警消息" → `chat message send-by-webhook`
用户说"查看我的机器人" → `chat bot search`

关键区分:
- `chat message send` — 以**当前用户**身份发消息（群聊或单聊），text 为位置参数
- `chat message send-by-bot` — 以**机器人**身份发消息（群聊或单聊），text 为 --text flag
- `chat message send-by-webhook` — 通过**自定义机器人 Webhook** 发群消息
- `chat message recall-by-bot` — 通过机器人撤回已发送的消息

## 核心工作流

```bash
# 1. 搜索群 — 提取 openconversation_id
dws chat search --query "项目冲刺" --format json

# 2. 拉取群消息
dws chat message list --group <openconversation_id> --time "2025-03-01 00:00:00" --format json

# 3. 以个人身份发送群消息
dws chat message send --group <openconversation_id> --title "周报提醒" "请大家本周五前提交周报" --format json

# 4. 以个人身份单聊
dws chat message send --user <userId> "你好" --format json

# 5. 机器人发群消息（Markdown）
dws chat message send-by-bot --robot-code <robot-code> \
  --group <openconversation_id> --title "日报" --text "## 今日完成..." --format json

# 6. 机器人单聊发消息
dws chat message send-by-bot --robot-code <robot-code> \
  --users userId1,userId2 --title "提醒" --text "请提交周报" --format json

# 7. Webhook 发告警
dws chat message send-by-webhook --token <webhook-token> \
  --title "告警" --text "CPU 超 90%" --at-all --format json
```

## 注意事项

- `--group` 为群聊会话 ID (openconversation_id)，可从群搜索或群聊信息中获取
- `chat message send` 的 text 是位置参数（恰好 1 个），非 flag；群聊用 `--group`，单聊用 `--user`，二者互斥
- `chat message list` 的 `--group` 和 `--user` 也互斥，必须且只能指定其一
- `--time` 格式: `yyyy-MM-dd HH:mm:ss`，为拉取消息的起始时间点；`--forward` 控制方向（默认 true，拉给定时间之后的消息），`--limit` 控制数量
- `chat search` 挂在 `chat` 下（非 `chat group` 下），路径为 `dws chat search`
- `send-by-bot` 群聊传 `--group`，单聊传 `--users`，二者互斥且必选其一
- `recall-by-bot` 群聊传 `--group` + `--keys`，单聊仅传 `--keys`（不传 `--group` 即为单聊撤回）
- `send-by-webhook` 支持 `--at-all`、`--at-mobiles`、`--at-users` 进行 @ 操作，但需在 `--text` 中包含 `@userId` 或 `@手机号` 才能生效

## 自动化脚本

| 脚本 | 场景 | 用法 |
|------|------|------|
| [chat_export_messages.py](../../scripts/chat_export_messages.py) | 导出群聊消息到 JSON 文件 | `python chat_export_messages.py --query "项目冲刺" --time "2026-03-10 00:00:00"` |
| [chat_history_with_user.py](../../scripts/chat_history_with_user.py) | 查询与某人的单聊聊天记录 | `python chat_history_with_user.py --name "张三" --time "2026-03-10 00:00:00"` |
