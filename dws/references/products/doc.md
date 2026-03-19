# 文档 (doc) 命令参考

## 命令总览

### 搜索文档
```
Usage:
  dws doc search [flags]
Example:
  dws doc search --keyword "会议纪要"
  dws doc search
Flags:
      --keyword string   搜索关键词 (不传则返回最近访问)
```

### 遍历文件列表
```
Usage:
  dws doc list [flags]
Example:
  dws doc list
  dws doc list --folder <FOLDER_ID>
  dws doc list --workspace <WS_ID> --page-size 20
Flags:
      --folder string       文件夹 ID 或 URL
      --workspace string    知识库 ID
      --page-size int       每页数量 (默认 50，最大 50)
      --page-token string   分页游标 (从上次结果的 nextPageToken 获取)
```

### 获取文档元信息
```
Usage:
  dws doc info [flags]
Example:
  dws doc info --node <DOC_ID>
  dws doc info --node "https://alidocs.dingtalk.com/i/nodes/<DOC_UUID>"
Flags:
      --node string   文档 ID 或 URL (必填)
```

### 读取文档内容
```
Usage:
  dws doc read [flags]
Example:
  dws doc read --node <DOC_ID>
  dws doc read --node "https://alidocs.dingtalk.com/i/nodes/<DOC_UUID>"
Flags:
      --node string   文档 ID 或 URL (必填)
```

### 创建文档
```
Usage:
  dws doc create [flags]
Example:
  dws doc create --name "项目周报"
  dws doc create --name "Q1 总结" --markdown "# Q1 总结" --folder <FOLDER_ID>
  dws doc create --name "知识库文档" --workspace <WS_ID>
Flags:
      --name string        文档名称 (必填)
      --folder string      目标文件夹 ID 或 URL
      --workspace string   目标知识库 ID
      --markdown string    文档初始 Markdown 内容
```

### 更新文档内容
```
Usage:
  dws doc update [flags]
Example:
  dws doc update --node <DOC_ID> --markdown "# 追加内容" --mode append
  dws doc update --node <DOC_ID> --markdown "# 完整替换" --mode overwrite
Flags:
      --node string       文档 ID 或 URL (必填)
      --markdown string   Markdown 内容 (必填)
      --mode string       更新模式: overwrite=覆盖, append=追加 (默认 append)
```

### 创建文件夹
```
Usage:
  dws doc folder create [flags]
Example:
  dws doc folder create --name "项目资料"
  dws doc folder create --name "子文件夹" --folder <PARENT_FOLDER_ID>
Flags:
      --name string        文件夹名称 (必填)
      --folder string      父文件夹 ID 或 URL
      --workspace string   目标知识库 ID
```

### 查询块元素
```
Usage:
  dws doc block list [flags]
Example:
  dws doc block list --node <DOC_ID>
  dws doc block list --node <DOC_ID> --start-index 0 --end-index 5
  dws doc block list --node <DOC_ID> --block-type heading
Flags:
      --node string         文档 ID 或 URL (必填)
      --start-index int     起始位置 (从 0 开始)
      --end-index int       终止位置 (含)
      --block-type string   按块类型过滤
```

### 插入块元素
```
Usage:
  dws doc block insert [flags]
Example:
  dws doc block insert --node <DOC_ID> --text "这是一段文字"
  dws doc block insert --node <DOC_ID> --heading "二级标题" --level 2
  dws doc block insert --node <DOC_ID> --element '{"blockType":"paragraph","paragraph":{"text":"内容"}}'
  dws doc block insert --node <DOC_ID> --text "在此处之前插入" --ref-block <BLOCK_ID> --where before
Flags:
      --node string        文档 ID 或 URL (必填)
      --text string        快捷: 段落文本内容
      --heading string     快捷: 标题文本
      --level int          标题级别 1-6 (配合 --heading，默认 1)
      --element string     块元素 JSON (高级)
      --index int          参照位置索引 (从 0 开始)
      --where string       插入方向: before / after (默认 after)
      --ref-block string   参照块 ID (优先级高于 --index)
```

### 更新块元素
```
Usage:
  dws doc block update [flags]
Example:
  dws doc block update --node <DOC_ID> --block-id <BLOCK_ID> --text "新内容"
  dws doc block update --node <DOC_ID> --block-id <BLOCK_ID> --element '{"blockType":"heading","heading":{"text":"新标题","level":1}}'
Flags:
      --node string        文档 ID 或 URL (必填)
      --block-id string    目标块 ID (必填)
      --text string        快捷: 段落文本内容
      --heading string     快捷: 标题文本
      --level int          标题级别 1-6 (配合 --heading，默认 1)
      --element string     块元素 JSON (高级)
```

### 删除块元素
```
Usage:
  dws doc block delete [flags]
Example:
  dws doc block delete --node <DOC_ID> --block-id <BLOCK_ID> --yes
Flags:
      --node string        文档 ID 或 URL (必填)
      --block-id string    目标块 ID (必填)
```

## 意图判断

用户说"找文档/搜文档/最近文档":
- 搜索 → `search`
- 浏览 → `list`

用户说"看文档/读内容/文档内容":
- 读取 → `read` (需文档 ID 或 URL)
- 元信息 → `info`

用户说"写文档/创建文档":
- 新建 → `create`
- 追加内容 → `update --mode append`
- 覆盖替换 → `update --mode overwrite`

用户说"建文件夹/新建目录":
- 创建 → `folder create`

用户说"编辑块/改段落/插入标题/删除块":
- 查看结构 → `block list`
- 插入 → `block insert`
- 修改 → `block update`
- 删除 → `block delete`

关键区分: doc(文档编辑/阅读) vs aitable(数据表格操作) vs drive(钉盘文件管理)

## 核心工作流

```bash
# ── 工作流 1: 浏览并阅读文档 ──

# 1. 浏览我的文档根目录
dws doc list --format json

# 2. 浏览子文件夹
dws doc list --folder <FOLDER_ID> --format json

# 3. 获取文档元信息 (标题、类型、权限)
dws doc info --node <DOC_ID> --format json

# 4. 读取文档内容 (Markdown 格式)
dws doc read --node <DOC_ID> --format json

# ── 工作流 2: 创建文档并写入内容 ──

# 1. (可选) 创建文件夹 — 提取 nodeId
dws doc folder create --name "项目资料" --format json

# 2. 创建文档 — 提取 nodeId
dws doc create --name "项目周报" --folder <FOLDER_ID> --format json

# 3. 写入内容 (追加模式)
dws doc update --node <DOC_ID> --markdown "# 本周总结\n\n- 完成了 A\n- 推进了 B" --mode append --format json

# ── 工作流 3: 一步创建带内容的文档 ──

dws doc create --name "会议纪要" --markdown "# 会议纪要\n\n## 议题\n\n1. ..." --format json

# ── 工作流 4: 块级精细编辑 ──

# 1. 查看文档块结构 — 获取 blockId
dws doc block list --node <DOC_ID> --format json

# 2. 在文档末尾插入段落
dws doc block insert --node <DOC_ID> --text "新增内容"

# 3. 在指定块之前插入标题
dws doc block insert --node <DOC_ID> --heading "新章节" --level 2 --ref-block <BLOCK_ID> --where before

# 4. 更新某个块的内容
dws doc block update --node <DOC_ID> --block-id <BLOCK_ID> --text "修改后的内容"

# 5. 删除块
dws doc block delete --node <DOC_ID> --block-id <BLOCK_ID> --yes
```

## 上下文传递表

| 操作 | 从返回中提取 | 用于 |
|------|-------------|------|
| `list` | `nodes[].nodeId` | read / info / update / block 操作的 --node |
| `list` | folder 类型的 `nodeId` | list 的 --folder, create 的 --folder |
| `search` | 文档 `nodeId` / URL | read / info / update 的 --node |
| `create` | `nodeId` | update / block 操作的 --node |
| `folder create` | `nodeId` | create / list 的 --folder |
| `block list` | `blockId` | block insert 的 --ref-block, block update/delete 的 --block-id |

## nodeId 双格式说明

所有 `--node` 参数同时支持两种格式，系统自动识别：
- **文档 ID**: 32 位字母数字字符串 (dentryUuid)
- **文档 URL**: `https://alidocs.dingtalk.com/i/nodes/{dentryUuid}`

`--folder` 参数同样支持文件夹 URL 或 ID。

## 注意事项

- `update --mode overwrite` 会**清空原内容后重写**，⚠️ 谨慎使用；默认 `--mode append` (追加) 更安全
- `read` 返回 Markdown 格式的文档内容，仅限有"下载"权限的文档
- `create` 不传 `--folder` 和 `--workspace` 时，默认创建在"我的文档"根目录
- `block list/insert/update/delete` 是块级精细编辑，适合结构化修改；简单内容追加建议用 `update --mode append`
- `block insert` 优先使用 `--text` 或 `--heading` 快捷方式；复杂块类型 (table, callout 等) 使用 `--element` JSON
- 块类型包括: paragraph, heading, blockquote, callout, columns, orderedList, unorderedList, table, sheet, attachment, slot
- 关键区分: doc(文档编辑/阅读) vs aitable(数据表格操作) vs drive(钉盘文件管理)

## 自动化脚本

| 脚本 | 场景 | 用法 |
|------|------|------|
| [doc_create_and_write.py](../../scripts/doc_create_and_write.py) | 创建文档并写入 Markdown 内容 | `python doc_create_and_write.py --name "周报" --content "# 本周总结"` |
