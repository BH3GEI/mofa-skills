# 单命令产品合集

以下产品命令较少，合并参考。

---

## devdoc — 开放平台文档

### 搜索开放平台文档
```
Usage:
  dws devdoc article search [flags]
Example:
  dws devdoc article search --keyword "OAuth2 接入" --page 1 --size 10
Flags:
      --keyword string   搜索关键词 (必填)
      --page string      页码 (默认 1)
      --size string      每页数量 (默认 10)
```

---

## conference — 视频会议
### 创建预约会议
```
Usage:
  dws conference meeting create [flags]
Example:
  dws conference meeting create --title "产品评审会" \
    --start 2026-03-11T14:00:00+08:00 --end 2026-03-11T15:00:00+08:00
Flags:
      --end string     结束时间 ISO-8601 格式 (必填)
      --start string   开始时间 ISO-8601 格式 (必填)
      --title string   会议标题 (必填)
```

注意: 不会自动关联日历日程。

关键区分: conference(视频会议预约) vs calendar event(日历日程)

---

## live — 直播

### 查看我的直播列表
```
Usage:
  dws live stream list [flags]
Example:
  dws live stream list
```

---

## 意图判断

- 用户说"开发文档/API 文档/接口文档" → `devdoc article search`
- 用户说"预约会议/视频会议" → `conference meeting create`
- 用户说"直播/我的直播" → `live stream list`

## 上下文传递表

| 操作 | 从返回中提取 | 用于 |
|------|-------------|------|
| `devdoc article search` | 文档链接 | 直接展示给用户 |
| `conference meeting create` | 会议 ID、入会链接 | 分享给参会者 |
