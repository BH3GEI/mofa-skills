# Example: Search Workflow

## Scenario 1: Basic Search

**User**: "搜索咖啡相关笔记"

**Agent**:
```bash
xhs search "咖啡" --json
```

**Output**:
```json
[
  {
    "id": "65a1b2c3d4e5f6g7",
    "title": "上海最值得一去的咖啡店",
    "author": "咖啡爱好者小王",
    "likes": 12500,
    "favorites": 2341,
    "abstract": "推荐5家上海的精品咖啡店..."
  },
  {
    "id": "65a1b2c3d4e5f6g8",
    "title": "手冲咖啡入门指南",
    "author": "咖啡师阿明",
    "likes": 8900,
    "favorites": 1200,
    "abstract": "从豆子到冲泡，手把手教你..."
  }
]
```

**Agent Response**:
```
📕 搜索到 10 条咖啡相关笔记

1. 【上海最值得一去的咖啡店】
   作者: 咖啡爱好者小王
   ❤️ 12.5k  ⭐ 2.3k
   摘要: 推荐5家上海的精品咖啡店...
   ID: 65a1b2c3d4e5f6g7

2. 【手冲咖啡入门指南】
   作者: 咖啡师阿明
   ❤️ 8.9k  ⭐ 1.2k
   摘要: 从豆子到冲泡，手把手教你...
   ID: 65a1b2c3d4e5f6g8

查看更多请输入数字 (1-10)，或说"搜索其他关键词"
```

---

## Scenario 2: Read Note Details

**User**: "看看第一个"

**Agent**:
```bash
xhs read 65a1b2c3d4e5f6g7 --comments
```

**Output**: Full note content with images and comments

**Agent Response**:
```
📖 笔记详情

标题: 上海最值得一去的咖啡店
作者: 咖啡爱好者小王
发布时间: 2024-01-15

【内容】
推荐5家上海的精品咖啡店：

1. % Arabica
   - 地址: 外滩...
   - 特色: 落地窗看外滩

2. 星巴克臻选
   - 地址: 南京西路...
   - 特色: 烘焙工坊

...

❤️ 12.5k  ⭐ 2.3k  💬 345

【热门评论】
- @用户A: 收藏了！周末就去
- @用户B: 第一家确实不错
```

---

## Scenario 3: Topic Search

**User**: "搜索旅行话题"

**Agent**:
```bash
xhs topics "旅行"
```

---

## Scenario 4: Browse Feed

**User**: "看看推荐内容"

**Agent**:
```bash
xhs feed --json
```
