# Firecrawl Scrape 任务模板

## 任务信息
- **目标 URL**: {{URL}}
- **任务类型**: scrape
- **执行时间**: {{TIMESTAMP}}

## 执行命令

```bash
firecrawl "{{URL}}" \
  --format {{FORMAT}} \
  {{#if ONLY_MAIN_CONTENT}}--only-main-content{{/if}} \
  {{#if WAIT_FOR}}--wait-for {{WAIT_FOR}}{{/if}} \
  {{#if SCREENSHOT}}--screenshot{{/if}} \
  -o "{{OUTPUT_PATH}}"
```

## 内容提取清单

- [ ] 页面标题
- [ ] 正文内容
- [ ] 发布时间/修改时间
- [ ] 作者信息
- [ ] 标签/分类
- [ ] 相关链接
- [ ] 图片资源

## 输出处理

1. 保存原始内容到: `{{RAW_OUTPUT_PATH}}`
2. 提取结构化数据到: `{{STRUCTURED_OUTPUT_PATH}}`
3. 生成内容摘要

## 质量标准

- 内容完整性: > 80%
- 格式正确性: markdown 无乱码
- 元数据完整: 至少包含标题
