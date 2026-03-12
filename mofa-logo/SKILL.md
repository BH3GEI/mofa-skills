---
name: mofa-logo
description: "AI-powered logo generation with Claude Opus 4.6 - minimalist, mascot, emblem, wordmark styles with SVG output"
triggers: ["logo", "标志", "Logo", "brand identity", "品牌设计"]
requires_model: claude-opus-4-6
always: false
---

# MOFA Logo Maker

基于 Claude Opus 4.6 的专业 Logo 生成器。经过实测，Opus 4.6 在 Logo 设计上效果最佳。

## Why Opus 4.6?

经过多轮测试对比：
- **Opus 4.6**: 设计感和专业性最强，理解品牌调性准确
- **Sonnet 4.6**: 速度较快但创意略逊
- **其他模型**: 常出现设计 cliché（过度使用渐变、阴影等）

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         MOFA LOGO MAKER PIPELINE                            │
└─────────────────────────────────────────────────────────────────────────────┘

User Request
    ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 1: ENTRY                                          │
│ ────────────                                            │
│ • 解析品牌信息 (名称、行业、调性)                       │
│ • 确定设计风格                                          │
│ • 设置约束条件                                          │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 2: STYLE SELECTION                                │
│ ────────────────────────                                │
│ • minimalist (极简)                                     │
│ • mascot (吉祥物)                                       │
│ • emblem (徽章)                                         │
│ • wordmark (文字标)                                     │
│ • lettermark (字母标)                                   │
│ • abstract (抽象图形)                                   │
│ • vintage (复古)                                        │
│ • modern (现代)                                         │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 3: PROMPT ENGINEERING                             │
│ ───────────────────────────                             │
│ • 注入设计原则 (Scalable, Memorable, Timeless)          │
│ • 色彩理论                                              │
│ • 构图规则                                              │
│ • 技术规范                                              │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 4: GENERATION (Opus 4.6)                          │
│ ───────────────────────────────                         │
│ • 生成多个概念方案                                      │
│ • SVG 代码 + 视觉描述                                   │
│ • 色彩方案                                              │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 5: REFINEMENT                                     │
│ ─────────────────                                       │
│ • 评估各方案                                            │
│ • 选择最优概念                                          │
│ • 迭代优化                                              │
└─────────────────────────┬───────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ Phase 6: OUTPUT                                         │
│ ─────────────                                           │
│ • SVG 代码                                              │
│ • PNG 渲染说明                                          │
│ • 使用规范                                              │
│ • 品牌应用示例                                          │
└─────────────────────────────────────────────────────────┘
```

## Usage

### Basic Logo Request

```
User: "给我设计一个咖啡店的 Logo"

Agent:
1. Gather brand info
   - Name: ?
   - Style preference: ?
   - Color preference: ?

2. Generate with Opus 4.6
   - 3-4 concepts
   - Different styles (minimalist, vintage, modern)

3. Present with rationale
```

### Advanced Brief

```
User: "科技初创公司，名字叫 ByteFlow，做 AI 基础设施，要现代简洁的风格"

Input:
- Brand: ByteFlow
- Industry: AI Infrastructure
- Style: Modern, Minimalist
- Values: Innovation, Reliability, Speed

Output:
- 3 logo concepts
- SVG code for each
- Color palette
- Usage guidelines
```

## Design Principles

Every logo design follows these principles:

1. **Scalable** - Works at 16x16 favicon or 100ft billboard
2. **Memorable** - Distinctive enough to recall after 5 seconds
3. **Timeless** - Avoids trendy gradients, shadows, effects
4. **Appropriate** - Matches brand personality and industry
5. **Simple** - Clean, uncluttered, focused
6. **Versatile** - Works in color, monochrome, reversed

## Styles

### Minimalist
- Clean lines, geometric shapes
- Maximum 2-3 elements
- Generous whitespace

### Mascot
- Character-based illustration
- Friendly and approachable
- Memorable personality

### Emblem
- Badge or seal format
- Traditional and authoritative
- Detailed and ornate

### Wordmark
- Typography-focused
- Custom letterforms
- Strong brand name

### Lettermark
- Initials or monogram
- Abbreviated brand name
- Highly compact

### Abstract
- Non-representational shapes
- Conceptual meaning
- Unique and distinctive

## Output Format

Each concept includes:
1. Concept Name
2. Design Description
3. Rationale
4. SVG Code
5. Color Palette (hex codes)
6. Usage Guidelines

## Tools

| Tool | Description |
|------|-------------|
| `generate_logo` | Main generation with Opus 4.6 |
| `refine_concept` | Iterate on selected concept |
| `create_variations` | Generate style variations |
| `export_svg` | Output optimized SVG code |
| `create_style_guide` | Generate brand guidelines |
