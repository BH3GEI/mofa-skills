---
name: mofa-infographic
description: "AI-generated infographics and visual posters. Triggers: infographic, poster, 信息图, 海报, data poster, visual summary, mofa infographic. Generates multi-section infographic via Gemini with optional Qwen-Edit refinement, stitched into a single tall image."
requires_bins: mofa
requires_env: GEMINI_API_KEY
---

# mofa-infographic

CLI: `mofa infographic` | Styles: `mofa-infographic/styles/*.toml` | Config: `mofa/config.json`

## Quick Start

```bash
mofa infographic --style cyberpunk-neon --out poster.png --work-dir /tmp/sections -i sections.json
```

## Interaction Guide

Before generating, gather preferences interactively. On Telegram, use inline keyboard buttons:

1. **Topic** — What data/story should the infographic present?
2. **Style** — Recommend based on content:
   - Tech / AI / data → `cyberpunk-neon`
   - Reports / articles / longform → `editorial`
   - Business / consulting / clean → `clean-light`
   - Comparisons / multi-topic summaries → `multi-panel`
3. **Number of sections** — Typically 3-5 (header, 2-3 content, footer)
4. **Resolution** — Default 2K; suggest 4K for print
5. **API mode** — `rt` (fast, default) or `batch` (50% cheaper, slower)
6. **API key** — Check if GEMINI_API_KEY is configured. If not, ask the user to provide it.

Present a section plan (section descriptions + variants) for confirmation before generating.

**Telegram inline keyboard example:**
```json
message(content="Choose an infographic style:", metadata={"inline_keyboard": [
  [{"text": "赛博朋克 cyberpunk-neon", "callback_data": "style:cyberpunk-neon"}, {"text": "杂志 editorial", "callback_data": "style:editorial"}],
  [{"text": "简约 clean-light", "callback_data": "style:clean-light"}, {"text": "多版块 multi-panel", "callback_data": "style:multi-panel"}]
]})
```
User's button press arrives as `[callback] style:cyberpunk-neon`.

## Styles (4)

| User says | `--style` | Theme | Best For |
|-----------|-----------|-------|----------|
| 赛博朋克、科技、neon | `cyberpunk-neon` | Dark background, neon accents, futuristic | Tech, AI, data |
| 杂志、editorial、magazine | `editorial` | Clean serif typography, magazine layout | Reports, articles |
| 简约、clean、商务 | `clean-light` | White background, minimal, data-forward | Business, consulting |
| 多版块、对比、multi | `multi-panel` | Bold color blocks, section dividers | Comparisons, summaries |
| "有哪些风格？" / "list styles" | Show all above | | |
| *(not specified)* | `cyberpunk-neon` | | |

### Section Variants

All styles support 3 variants:

| Variant | Auto-assigned to | Description |
|---------|------------------|-------------|
| `header` | First section | Title banner, hero visual |
| `normal` | Middle sections | Content, data, charts |
| `footer` | Last section | Sources, credits, call-to-action |

Variant is auto-detected by position. Override with the `variant` field in JSON.

## API Modes

| `--api` | Speed | Cost | How it works |
|---------|-------|------|--------------|
| `rt` (default) | Fast (~2-3 min) | Standard pricing | Parallel sync calls via rayon thread pool |
| `batch` | Slow (5-30 min) | **50% cheaper** | Gemini Batch API, async processing. Falls back to `rt` on timeout. |

Use `--api batch` for large infographics (8+ sections) where cost matters more than speed.

## How It Works

1. **Generate sections** — Each section is generated as a separate 16:9 image
2. **Optional refinement** — Qwen-Edit can refine sections (text correction, cleanup)
3. **Vertical stitch** — All sections stitched top-to-bottom into one tall image

The final output is a single tall PNG — ideal for social media, web pages, or printing as a poster.

## Input JSON Schema

Top-level: array of section objects.

### Section Object

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `prompt` | string | **yes** | Section content description — what to render. Include data, titles, visual elements. |
| `variant` | string | no | Section variant: `"header"`, `"normal"`, `"footer"`. Auto-detected if omitted. |
| `refine_prompt` | string | no | Qwen-Edit instruction for post-generation refinement (requires `--refine` and DASHSCOPE_API_KEY) |

### Prompt Writing Tips

- **Be data-specific**: "3 KPI cards: Revenue $247B, Growth 3.2x, Programs 140+" beats "Some statistics"
- **Describe visual layout**: "Timeline with 5 milestone markers", "2x2 grid of feature cards"
- **Include text content**: Write exact numbers, titles, labels you want to appear
- **Set visual tone**: "Dark background with glowing blue accents", "Clean white with thin dividers"
- **Header sections**: Include a bold title and a striking hero visual
- **Footer sections**: Include sources, credits, URLs in small text

## Examples

### Tech infographic (4 sections)

```json
[
  {"prompt": "TITLE: 'AI in 2026' in bold futuristic font. Subtitle: 'The State of Artificial Intelligence'. Circuit patterns and neural network nodes in the background. Glowing blue neon accents."},
  {"prompt": "3 large KPI cards in a row: '$347B market size' with upward arrow, '3.2x YoY growth' with chart icon, '140+ national AI programs' with globe icon. Dark background, neon blue highlights."},
  {"prompt": "Horizontal timeline: 5 milestone markers — 2020 GPT-3, 2022 ChatGPT, 2023 GPT-4, 2024 Gemini, 2026 AGI Race. Each with an icon. Connected by glowing line."},
  {"prompt": "Footer: 'Sources: McKinsey Global Institute, Stanford HAI, OECD AI Policy Observatory' in small white text. Subtle circuit pattern. Copyright 2026."}
]
```

```bash
mofa infographic --style cyberpunk-neon --out /tmp/ai-poster.png -i tech.json
```

### Business report (5 sections)

```json
[
  {"prompt": "Header: 'Q3 2026 Business Review' large centered title. Company logo placeholder. Subtle gradient background.", "variant": "header"},
  {"prompt": "Revenue overview: Large number '$12.4M' with green upward trend line. Comparison bar chart: Q1 $8.2M, Q2 $10.1M, Q3 $12.4M. Clean minimal design."},
  {"prompt": "Customer metrics: 3 cards — 'NPS Score: 72' with gauge, 'Churn: 2.1%' with downward arrow (green), 'New Customers: 1,847' with person icon."},
  {"prompt": "Product roadmap: 4 phases horizontally — Q4 Launch v2.0, Q1 Mobile App, Q2 Enterprise Tier, Q3 International. Each with status badge."},
  {"prompt": "Footer: 'Confidential — Internal Use Only. Prepared by Strategy Team.' Thin horizontal line above.", "variant": "footer"}
]
```

```bash
mofa infographic --style clean-light --out /tmp/review.png --image-size 2K -i report.json
```

### Magazine editorial (3 sections with refinement)

```json
[
  {"prompt": "Editorial header: Large serif text 'The Future of Remote Work'. Dramatic photo-style background of a modern home office with city skyline through the window.", "variant": "header"},
  {"prompt": "Two-column layout: Left column has body text discussing hybrid work trends. Right column has a vertical bar chart showing remote vs office work percentages by year (2020-2026).", "refine_prompt": "Sharpen the text and make the chart labels more legible"},
  {"prompt": "Quote block: 'The office is no longer a place — it's an experience.' — attributed to a Fortune 500 CEO. Large quotation marks. Subtle texture background.", "variant": "footer"}
]
```

```bash
mofa infographic --style editorial --out /tmp/remote-work.png --refine --image-size 4K -i editorial.json
```

### Batch API for large infographic

```bash
mofa infographic --style multi-panel --api batch --out /tmp/mega-poster.png -i 10-sections.json
```

## CLI Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--style` | `cyberpunk-neon` | Style name (see Styles table) |
| `-o` / `--out` | *required* | Final stitched output image path (PNG) |
| `--work-dir` | parent of --out | Working directory for individual section PNGs |
| `--aspect` | `16:9` | Per-section aspect ratio |
| `--concurrency` | 3 | Parallel generation workers (1-20) |
| `--image-size` | config | `"1K"` / `"2K"` / `"4K"` |
| `--refine` | false | Refine sections with Dashscope Qwen-Edit (needs DASHSCOPE_API_KEY) |
| `--gutter` | 0 | Gap between sections in pixels (0 = seamless) |
| `--api` | `rt` | API mode: `rt` (realtime, fast parallel) or `batch` (50% cheaper, async 5-30 min) |
| `-i` / `--input` | stdin | Input JSON file path |
| `--root` | auto-detected | Path to mofa root directory |

## Resolution & Quality

| Flag | Values | Description |
|------|--------|-------------|
| `--image-size` | `1K`, `2K`, `4K` | Per-section resolution. Higher = sharper but slower and costlier. |
| `--aspect` | ratio string | Per-section aspect ratio. `16:9` (default, landscape), `4:3`, `1:1`. |
| `--gutter` | pixels | Gap between sections. 0 for seamless (default), 10-20 for visible dividers. |
| `--concurrency` | 1-20 | More workers = faster but higher API rate limit risk. Default 3 is safe. |

## Config

`mofa/config.json`:

```json
{
  "api_keys": {
    "gemini": "env:GEMINI_API_KEY",
    "dashscope": "env:DASHSCOPE_API_KEY"
  },
  "gen_model": "gemini-3.1-flash-image-preview",
  "defaults": {
    "infographic": { "style": "cyberpunk-neon", "panels": 3, "refine_with_qwen": true }
  }
}
```

**API keys**: `GEMINI_API_KEY` required for all generation. `DASHSCOPE_API_KEY` only needed for `--refine`.
**Models**: `gen_model` controls image generation model (default: `gemini-3.1-flash-image-preview`).

## Output

- Individual sections saved in `--work-dir` as `section-01.png`, `section-02.png`, ...
- Final stitched image at `--out` path (tall vertical PNG)
- Sections are cached: if `section-XX.png` exists and is >10KB, it's reused (delete to regenerate)
- Final image width matches the widest section; narrower sections are centered
