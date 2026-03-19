# 意图路由指南

当用户请求难以判断归属哪个产品时，参考本指南。

## 易混淆场景快速对照表

| 用户说... | 真实意图 | 应该用 | 不要用 | 理由 |
|-----------|----------|--------|--------|------|
| "搜一下 OAuth2 接入文档" | 搜索开发文档 | `devdoc` | `doc search` | 搜索开放平台技术文档，不是钉钉内部内容 |
| "帮我建一个项目跟踪表" | 创建数据表格 | `aitable` | `doc` | 涉及结构化数据/行列操作，不是富文本文档 |
| "帮我写个项目周报" | 创建钉钉文档 | `doc` | `aitable` | 富文本内容创作，不是数据表 |
| "帮我记一下明天要做的事" | 创建个人待办 | `todo` | `doc` | 个人待办提醒，非文档内容 |
| "帮我把这个文件传到网盘" | 钉盘上传 | `drive` | `doc` | 文件存储/上传，不是文档内容编辑 |
| "帮我看看知识库里的文件" | 文档空间 | `doc` | `drive` | 钉钉文档知识库，不是钉盘文件管理 |
| "帮我预约一个视频会议" | 视频会议 | `conference` | `calendar` | 视频会议预约（含入会链接），不是日历日程 |
| "帮我建一个明天下午的日程" | 日历日程 | `calendar` | `conference` | 日历日程管理（可含参与者/会议室）|
| "通知群里的人都来开会" | 个人身份群发 | `chat message send` | `chat message send-by-bot` | 以个人身份向群发消息 |
| "让机器人每天推送日报" | 机器人定时推送 | `chat message send-by-bot` | `chat message send` | 需要机器人身份定期发送 |
| "CPU 超过 90% 自动告警" | Webhook 告警 | `chat message send-by-webhook` | `chat message send-by-bot` | 系统告警场景，需自定义 Webhook |
| "帮我看看收到的日报" | 日志收件箱 | `report` | `doc` | 钉钉日志系统（日报/周报），不是文档 |
| "帮我创建一个待办提醒" | 个人待办 | `todo` | `report` | 个人任务提醒，不是日志汇报 |
| "拉取一下上周项目群的聊天记录" | 拉取会话消息 | `chat message list` | — | 拉取指定群聊的消息列表 |

---

## 典型场景详解

### 1. aitable vs doc — 表格数据 vs 文档内容

**用 `aitable` 的场景**：
- "创建一个表格记录团队成员信息" — 结构化数据，有行列
- "在表格里加一列'状态'字段" — 字段/列操作
- "查一下表格里所有优先级为高的记录" — 数据筛选和查询
- "用项目管理模板建一个表" — 模板创建
- 用户提到"多维表"、"Base"、"数据表"、"记录"

**用 `doc` 的场景**：
- "帮我写个会议纪要" — 富文本内容创作
- "看一下这个文档链接的内容" — 阅读文档
- "在知识库创建一个文件夹" — 文档空间管理
- 用户提到"文档"、"知识库"、"写文档"

**判断关键**：有没有行列/字段/记录概念？有→ `aitable`；纯文本/Markdown → `doc`

---

### 2. devdoc vs doc search — 两种搜索

**用 `devdoc` 的场景**：
- "API 调用报错 403 怎么解决" — 开发调试问题
- "搜一下 OAuth2 接入文档" — 开放平台技术文档
- "CLI 命令出错了怎么办" — CLI 使用错误
- 用户提到"开发"、"API"、"调用错误"

**用 `doc search` 的场景**：
- "在我的文档里搜一下'项目方案'" — 搜索文档标题和内容
- 用户明确说"我的文档"、"知识库里搜"

**判断关键**：搜开发文档→ `devdoc`；搜用户自己的文档→ `doc search`

---

### 3. drive vs doc — 文件存储 vs 文档内容

**用 `drive` 的场景**：
- "把这个 PDF 传到钉盘" — 上传文件
- "下载那个 Excel 附件" — 下载文件
- "看一下钉盘根目录有什么文件" — 浏览文件列表
- 用户提到"钉盘"、"网盘"、"上传"、"下载"

**用 `doc` 的场景**：
- "读一下这个文档的内容" — 读取文档 Markdown
- "帮我写入一段话到文档里" — 编辑文档内容
- "在知识库里搜索会议纪要" — 搜索文档
- 用户提到"文档内容"、"知识库"

**判断关键**：文件存储/传输→ `drive`；文档内容读写→ `doc`

---

### 4. conference vs calendar — 视频会议 vs 日历日程

**用 `conference` 的场景**：
- "帮我预约一个视频会议" — 需要入会链接
- 用户明确说"视频会议"

**用 `calendar` 的场景**：
- "明天下午安排个会" — 日程管理（可含会议室）
- "帮我约几个人开会" — 创建日程 + 添加参与者
- "看看下午有没有空闲会议室" — 会议室管理
- "帮我查一下同事有空吗" — 闲忙查询
- 用户提到"日程"、"会议室"、"约会"

**判断关键**：仅需视频会议入会链接→ `conference`；日程/参与者/会议室管理→ `calendar`

---

### 5. chat 内部 — 三种消息发送方式

**用 `chat message send` 的场景**：
- "帮我在群里发个消息提醒大家" — **个人身份**发群消息

**用 `chat message send-by-bot` 的场景**：
- "让机器人在群里发一条通知" — **机器人身份**发消息
- "给张三发一条机器人单聊消息" — 机器人单聊

**用 `chat message send-by-webhook` 的场景**：
- "通过 Webhook 发告警到群里" — 自定义机器人 Webhook
- 用户有 Webhook Token

**判断关键**：个人发→ `send`；机器人发→ `send-by-bot`；有 Webhook Token→ `send-by-webhook`

---

### 6. report vs doc vs todo — 日志 vs 文档 vs 待办

**用 `report` 的场景**：
- "帮我看看收到的日报" — 日志收件箱
- "有什么日志模版" — 查看模版
- "看看这个日志的已读统计" — 阅读状态
- "我发过的日志有哪些" — 已发送列表 (`report sent`)
- 用户提到"日报"、"周报"、"日志"

**用 `doc` 的场景**：
- "帮我写个项目总结文档" — 长文本创作

**用 `todo` 的场景**：
- "记一下这周要做的事" — 个人任务管理

**判断关键**：钉钉日志系统(日报/周报模版)→ `report`；文档创作→ `doc`；任务清单→ `todo`

---

---

## 跨产品工作流路由

以下场景需要多个产品配合完成，注意上下文传递顺序。

### 发邮件给同事（contact → mail）

用户说"给张三发封邮件"，但只知道名字不知道邮箱地址：

```bash
# 1. 通过通讯录搜索用户 — 提取 email 字段
dws contact user search --keyword "张三" --format json

# 2. 用搜索到的邮箱地址作为收件人发送邮件
dws mail mailbox list --format json  # 获取发件人邮箱
dws mail message send --from my@company.com --to zhangsan@company.com \
  --subject "周报" --body "内容" --format json
```

### 创建日程并邀请同事（contact → calendar）

用户说"约张三明天下午开会"：

```bash
# 1. 搜索同事 userId
dws contact user search --keyword "张三" --format json

# 2. 创建日程
dws calendar event create --title "会议" \
  --start "2026-03-15T14:00:00+08:00" --end "2026-03-15T15:00:00+08:00" --format json

# 3. 添加参与者
dws calendar participant add --event <EVENT_ID> --users <USER_ID> --format json
```

### 创建待办并指派（contact → todo）

用户说"给张三建个待办"：

```bash
# 1. 搜索同事 userId
dws contact user search --keyword "张三" --format json

# 2. 创建待办
dws todo task create --title "任务内容" --executors <USER_ID> --format json
```
