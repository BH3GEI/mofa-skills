# Example: Post Workflow

## Scenario 1: Publish Note with Image

**User**: "我想发一篇笔记"

**Agent**: "请提供：
1. 标题（必填）
2. 图片路径（可选）
3. 正文内容（可选）"

**User**: "标题是'今天的咖啡时光'，图片在 ~/photos/coffee.jpg，正文写'周末的下午，一杯手冲'"

**Agent**:
```bash
# Step 1: Check login
xhs status

# Step 2: Publish
xhs post "今天的咖啡时光" \
  --image ~/photos/coffee.jpg \
  --content "周末的下午，一杯手冲"
```

**Output**:
```
✅ 发布成功！

标题: 今天的咖啡时光
Note ID: 65a1b2c3d4e5f6g10
链接: https://www.xiaohongshu.com/discovery/item/65a1b2c3d4e5f6g10
```

---

## Scenario 2: Multi-Image Post

**User**: "发一篇带3张图的笔记，标题是'旅行日记'"

**Agent**:
```bash
xhs post "旅行日记" \
  --image ~/photos/trip1.jpg \
  --image ~/photos/trip2.jpg \
  --image ~/photos/trip3.jpg \
  --content "美好的旅行回忆"
```

---

## Scenario 3: Text-Only Post

**User**: "发一篇纯文字笔记"

**Agent**:
```bash
xhs post "今日感悟" --content "生活的美好在于发现"
```

---

## Scenario 4: Check User Profile

**User**: "查看我的资料"

**Agent**:
```bash
xhs whoami
```

---

## Scenario 5: Logout

**User**: "退出登录"

**Agent**:
```bash
xhs logout
```

**Output**:
```
✅ 已退出登录
```
