# Firecrawl Crawl 任务模板

## 任务信息
- **目标 URL**: {{URL}}
- **任务类型**: crawl
- **执行时间**: {{TIMESTAMP}}

## 执行命令

```bash
firecrawl crawl "{{URL}}" \
  {{#if LIMIT}}--limit {{LIMIT}}{{/if}} \
  {{#if MAX_DEPTH}}--max-depth {{MAX_DEPTH}}{{/if}} \
  {{#if INCLUDE_PATHS}}--include-paths {{INCLUDE_PATHS}}{{/if}} \
  {{#if EXCLUDE_PATHS}}--exclude-paths {{EXCLUDE_PATHS}}{{/if}} \
  {{#if ALLOW_SUBDOMAINS}}--allow-subdomains{{/if}} \
  {{#if DELAY}}--delay {{DELAY}}{{/if}} \
  --wait \
  {{#if PROGRESS}}--progress{{/if}} \
  -o "{{OUTPUT_PATH}}"
```

## 爬取范围

- **包含路径**: {{INCLUDE_PATHS}}
- **排除路径**: {{EXCLUDE_PATHS}}
- **最大页面数**: {{LIMIT}}
- **最大深度**: {{MAX_DEPTH}}

## 进度跟踪

```
正在爬取: {{URL}}
├── 已发现: {discovered} 个 URL
├── 已抓取: {completed} 个页面
├── 失败: {failed} 个
└── 剩余: {remaining} 个
```

## 结果处理

1. 解析 crawl-results.json
2. 提取每个页面的内容
3. 生成页面索引
4. 统计信息汇总

## 输出文件

```
{{OUTPUT_DIR}}/
├── crawl-results.json     # 原始爬取结果
├── index.json             # 页面索引
├── content/               # 页面内容
│   ├── page-001.md
│   └── ...
└── stats.json             # 统计信息
```
