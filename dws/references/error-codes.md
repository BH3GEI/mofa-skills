# 错误码说明

## 通用错误 (所有产品)

### 认证失败 (401 / token expired)
**原因**: Token 过期或未登录
**解决**:
```bash
dws auth status    # 检查状态
dws auth login     # 重新登录
```

### 请求超时
**原因**: 网络慢或服务端响应慢
**解决**: 增加超时时间 `--timeout 60` 重试

### 网络连接失败
**原因**: 无法连接 MCP Server
**解决**:
```bash
dws aitable base list --format json  # 用最简命令验证连接
```

---

## aitable 专用错误

> 以下内容针对新版 schema：`baseId / tableId / fieldId / recordId`。

### 参数体系写错
**现象**
- 还在用旧参数：`dentryUuid` / `sheetIdOrName` / `--doc` / `--sheet`（这些都已废弃）
- 接口直接报参数缺失 / 无效请求

**原因**
- MCP server 已升级到新 schema，但脚本或命令没跟上

**解决**
- Base 级：用 `--base-id`（对应 `baseId`）
- Table 级：用 `--table-id`（对应 `tableId`）
- Field 级：用 `--field-id`（对应 `fieldId`）
- Record 级：用 `--record-ids`（对应 `recordId`）

---

### 参数名大小写或命名风格错误
**现象**
- 参数看起来传了，但服务端像没收到
- 报字段缺失 / 资源不存在

**原因**
- CLI flag 使用 kebab-case（`--base-id`），JSON 内使用 camelCase（`baseId`）

**正确示例**
```bash
dws aitable base get --base-id <BASE_ID> --format json
dws aitable table update --base-id <BASE_ID> --table-id <TABLE_ID> --name '新表名' --format json
```

**错误示例**
```bash
dws aitable base get --baseId <BASE_ID>     # 错: flag 不是 camelCase
dws aitable base get --doc <UUID>           # 错: 旧参数
```

---

### 查询记录时单选 / 多选过滤无结果
**现象**
- 明明记录存在，但 `record query --filters` 查不出来

**原因**
- 对 `singleSelect / multipleSelect` 字段做过滤时，建议传 **option id**

**解决**
1. 先 `dws aitable field get --base-id <BASE_ID> --table-id <TABLE_ID> --format json` 读取字段完整配置
2. 找到 options 里的 id
3. 在 filters 里传 id

---

### create / update 记录写入失败
**常见原因**
- `cells` 的 key 用了字段名，不是 `fieldId`
- `url` 字段直接传字符串（应传 `{"text":"显示文本","link":"https://..."}`）
- `richText` 字段直接传字符串（应传 `{"markdown":"**加粗**"}`）
- `group` 字段写成 `openConversationId`（应传 `[{"cid":"xxx"}]`）
- 单次超过 100 条

**解决**
- 先用 `dws aitable table get --base-id <BASE_ID> --format json` 拿字段目录
- 必要时 `dws aitable field get` 获取完整配置

---

### update_field 更新单选/多选后历史数据异常
**现象**
- 更新选项后，已有单元格显示错乱或丢值

**原因**
- 更新 `options` 时没有传完整列表
- 已有 option 没保留原 `id`

**解决**
- 先 `dws aitable field get` 取完整配置
- 更新时传完整 options 列表，已有项保留原 `id`

---

### delete_table 失败：cannot delete the last table
**原因**: 该表是 Base 中最后一张表
**解决**: 先新建一张表再删旧表；或改用 `dws aitable base delete`

---

### create_fields / create_table 某些字段类型失败
**已知边界**
- `formula` 当前可能 `not supported yet`
- 关联字段可能因下游主键约束失败

**建议**
- 复杂字段拆开单独创建
- 先建基础结构，再逐项补复杂字段

---

## Agent 错误处理规则 (严格遵守)

1. 遇到任何错误，加 `--verbose` 重试一次
2. 仍然失败，把完整错误信息报告给用户，不要自行尝试替代方案
3. 禁止: 打开浏览器、构造 HTTP 请求、使用 curl
4. 禁止: 编造不存在的命令或参数
5. 禁止: 跳过错误继续执行后续操作

## 推荐排查顺序

### 先确认 ID 链路
1. `dws aitable base list` / `base search` → 拿 `baseId`
2. `dws aitable base get --base-id <ID>` → 拿 `tableId`
3. `dws aitable table get --base-id <ID>` → 拿 `fieldId`
4. `dws aitable record query` / 结果对象 → 拿 `recordId`

别跳步，别猜 ID。

### 再确认 payload 结构
- 新增/更新记录：看 `cells`（key 为 fieldId）
- 新增字段：看 `--fields` JSON 数组
- 更新字段：看 `--config`
- 查询过滤：看 `--filters`

### 最后确认批量上限
- 字段批量：15
- table / field 详情批量：10
- record 批量：100

## 调试命令模板

```bash
# 看 Base
dws aitable base list --format json
dws aitable base get --base-id <BASE_ID> --format json

# 看 Table / Field
dws aitable table get --base-id <BASE_ID> --table-ids <TABLE_ID> --format json
dws aitable field get --base-id <BASE_ID> --table-id <TABLE_ID> --format json

# 查记录
dws aitable record query --base-id <BASE_ID> --table-id <TABLE_ID> --limit 10 --format json

# 新增记录
dws aitable record create --base-id <BASE_ID> --table-id <TABLE_ID> \
  --records '[{"cells":{"fldXXX":"值"}}]' --format json
```

## 一句话原则

- **别再用旧 schema。**
- **别猜 ID。**
- **复杂参数一律用 JSON。**
- **先读结构，再写数据。**

