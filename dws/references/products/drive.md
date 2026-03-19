# 钉盘 (drive) 命令参考

## 命令总览

### 获取文件/文件夹列表
```
Usage:
  dws drive list [flags]
Example:
  dws drive list --max 20
  dws drive list --max 20 --parent-id <dentryUuid> --order-by name --order asc
Flags:
      --max string          每页返回数量，默认 20，最大 100 (必填)
      --next-token string   分页游标，首次不传 (可选)
      --order string        排序方向: asc|desc，默认 desc (可选)
      --order-by string     排序字段: createTime|modifyTime|name (可选)
      --parent-id string    父节点 ID (dentryUuid)，不传则列出空间根目录 (可选)
      --space-id string     空间 ID，不传则使用「我的文件」对应 spaceId (可选)
      --thumbnail           是否返回缩略图信息 (可选)
```

### 获取文件元数据信息
```
Usage:
  dws drive info [flags]
Example:
  dws drive info --file-id <dentryUuid>
Flags:
      --file-id string    节点 ID (dentryUuid) (必填)
      --space-id string   节点所属空间 ID (可选)
```

### 获取文件下载链接
```
Usage:
  dws drive download [flags]
Example:
  dws drive download --file-id <dentryUuid>
Flags:
      --file-id string    文件 ID (dentryUuid) (必填)
      --space-id string   文件所属空间 ID (可选)
```

### 创建文件夹
```
Usage:
  dws drive mkdir [flags]
Example:
  dws drive mkdir --name "项目资料"
  dws drive mkdir --name "子目录" --parent-id <dentryUuid>
Flags:
      --name string        文件夹名称，最长 50 字符 (必填)
      --parent-id string   父节点 ID (dentryUuid)，不传则在空间根目录下创建 (可选)
      --space-id string    目标空间 ID，不传则使用「我的文件」 (可选)
```

### 获取文件上传信息
```
Usage:
  dws drive upload-info [flags]
Example:
  dws drive upload-info --file-name "报告.pdf" --file-size 102400
  dws drive upload-info --file-name "readme.txt" --file-size 1024 --parent-id <dentryUuid>
Flags:
      --file-name string   文件名，须包含扩展名，如 报告.pdf (必填)
      --file-size string   文件大小（字节） (必填)
      --mime-type string   文件 MIME 类型，如 application/pdf，不传则自动推断 (可选)
      --parent-id string   父节点 ID (dentryUuid)，不传则上传到空间根目录 (可选)
      --space-id string    目标空间 ID，不传则使用「我的文件」 (可选)
```

### 提交文件上传
```
Usage:
  dws drive commit [flags]
Example:
  dws drive commit --file-name "报告.pdf" --file-size 102400 --upload-id <uploadId>
  dws drive commit --file-name "readme.txt" --file-size 1024 --upload-id <uploadId> [--space-id <spaceId>] [--parent-id <parentId>]
Flags:
      --file-name string   文件名（含扩展名），须与 get_upload_info 时一致 (必填)
      --file-size string   文件大小（字节），须与 get_upload_info 时一致 (必填)
      --upload-id string   上传 ID，来自 get_upload_info 返回的 uploadId (必填)
      --space-id string    空间 ID，不传则使用「我的文件」 (可选)
      --parent-id string   父节点 ID (dentryUuid)，不传则提交到根目录 (可选)
```

## 意图判断

用户说"我的文件/钉盘/网盘/云盘" → `list`
用户说"文件详情/文件信息" → `info`
用户说"下载文件" → `download`
用户说"新建文件夹/创建目录" → `mkdir`
用户说"获取上传信息/准备上传" → `upload-info`
用户说"上传文件/提交上传" → `commit`

关键区分: drive(钉盘文件管理) vs doc(文档内容读写)

## 核心工作流

```bash
# 1. 浏览「我的文件」根目录
dws drive list --max 20 --format json

# 2. 进入子目录 — 提取 dentryUuid 作为 parent-id
dws drive list --max 20 --parent-id <dentryUuid> --format json

# 3. 查看文件元数据
dws drive info --file-id <dentryUuid> --format json

# 4. 获取下载链接
dws drive download --file-id <dentryUuid> --format json

# 5. 创建文件夹
dws drive mkdir --name "项目资料" --format json

# 6. 上传文件（需先通过 upload-info 获取 uploadId）
dws drive commit --file-name "报告.pdf" --file-size 102400 --upload-id <uploadId> --format json
```

## 上下文传递表

| 操作 | 从返回中提取 | 用于 |
|------|-------------|------|
| `list` | `dentryUuid` | info / download / mkdir / list 的 --file-id 或 --parent-id |
| `list` | `spaceId` | info / download / mkdir / commit 的 --space-id |
| `mkdir` | `dentryUuid` | list 的 --parent-id |

## 注意事项

- 不传 `--space-id` 时默认使用「我的文件」空间
- 不传 `--parent-id` 时默认操作空间根目录
- `--order-by` 支持: `createTime`、`modifyTime`、`name`
- `commit` 是上传流程的最后一步，需先调用 `get_upload_info` 获取上传凭证
- `--file-name` 必须包含扩展名（如 `report.pdf`）

## 自动化脚本

| 脚本 | 场景 | 用法 |
|------|------|------|
| [drive_tree_list.py](../../scripts/drive_tree_list.py) | 递归列出钉盘目录树结构 | `python drive_tree_list.py --depth 2` |
