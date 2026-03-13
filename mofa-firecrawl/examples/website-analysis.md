# 示例: 网站内容分析

## 场景
分析一个技术博客网站的内容结构和主题分布。

## 执行步骤

### 1. URL 发现
```bash
firecrawl map "https://techblog.example.com" \
  --include-subdomains \
  --json \
  --pretty \
  -o ./firecrawl/techblog/urls.json
```

### 2. 整站爬取
```bash
firecrawl crawl "https://techblog.example.com" \
  --include-paths /blog,/tutorials \
  --exclude-paths /admin,/author \
  --limit 200 \
  --max-depth 2 \
  --wait \
  --progress \
  -o ./firecrawl/techblog/crawl-results.json
```

### 3. 内容分析
处理爬取结果，生成:
- 主题分类统计
- 发布时间分布
- 热门标签分析
- 作者贡献度

## 预期输出
```
./firecrawl/techblog/
├── report.md              # 分析报告
├── urls.json              # URL 列表
├── crawl-results.json     # 爬取结果
└── analysis/
    ├── topics.json        # 主题分布
    ├── timeline.json      # 时间线
    └── authors.json       # 作者统计
```
