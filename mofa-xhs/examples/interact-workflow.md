# Example: Interact Workflow

## Scenario 1: Like a Note

**User**: "给刚才那篇笔记点赞"

**Agent**:
```bash
# Step 1: Check login
xhs status
```

If not logged in:
```bash
xhs login
```

**Step 2: Like**
```bash
xhs like 65a1b2c3d4e5f6g7
```

**Output**:
```
✅ 已点赞笔记: 上海最值得一去的咖啡店
```

---

## Scenario 2: Unlike

**User**: "取消点赞"

**Agent**:
```bash
xhs like 65a1b2c3d4e5f6g7 --undo
```

---

## Scenario 3: Favorite

**User**: "收藏这个笔记"

**Agent**:
```bash
xhs favorite 65a1b2c3d4e5f6g7
```

---

## Scenario 4: Comment

**User**: "评论'很有用，收藏了'"

**Agent**:
```bash
xhs comment 65a1b2c3d4e5f6g7 "很有用，收藏了"
```

**Output**:
```
✅ 评论已发布
```

---

## Scenario 5: View Favorites

**User**: "我收藏了哪些笔记"

**Agent**:
```bash
xhs favorites --max 10
```

---

## Scenario 6: Delete Own Note

**User**: "删除我之前发的笔记"

**Agent**:
```bash
# Confirm first!
xhs delete 65a1b2c3d4e5f6g9
```

⚠️ **Warning**: This is irreversible!
