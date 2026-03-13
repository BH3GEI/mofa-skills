# 示例: 竞品信息抓取

## 场景
抓取竞品网站的定价、功能介绍和更新日志。

## 执行步骤

### 1. 关键页面抓取
```bash
# 定价页面
firecrawl "https://competitor.example.com/pricing" \
  --only-main-content \
  -o ./firecrawl/competitor/pricing.md

# 功能页面
firecrawl "https://competitor.example.com/features" \
  --only-main-content \
  -o ./firecrawl/competitor/features.md

# 更新日志
firecrawl "https://competitor.example.com/changelog" \
  --only-main-content \
  -o ./firecrawl/competitor/changelog.md
```

### 2. 搜索相关讨论
```bash
firecrawl search "competitor.example.com reviews 2026" \
  --limit 10 \
  --scrape \
  --scrape-formats markdown \
  --pretty \
  -o ./firecrawl/competitor/reviews.json
```

## 预期输出
```
./firecrawl/competitor/
├── report.md              # 竞品分析报告
├── pricing.md             # 定价信息
├── features.md            # 功能介绍
├── changelog.md           # 更新日志
└── reviews.json           # 用户评价
```
