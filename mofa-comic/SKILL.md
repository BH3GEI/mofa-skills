---
name: mofa-comic
description: "AI-generated comic strips and illustrations. Triggers: comic, manga, xkcd, 漫画, comic strip, 四格漫画, panel comic, illustration strip, snoopy. Generates multi-panel comics via Gemini with optional Qwen-Edit refinement, stitched into a single image."
requires_bins: mofa
requires_env: GEMINI_API_KEY
---

# mofa-comic

CLI: `mofa comic` | Styles: `mofa-comic/styles/*.toml` | Config: `mofa/config.json`

## Quick Start

```bash
mofa comic --style xkcd --layout grid --out comic.png --work-dir /tmp/panels -i panels.json
```

## Interaction Guide

Before generating, gather preferences interactively. On Telegram, use inline keyboard buttons:

1. **Story/topic** — What should the comic be about?
2. **Style** — Recommend based on content:
   - Tech humor / explanations → `xkcd`
   - Action / drama / storytelling → `manga`
   - Adventure / editorial → `ligne-claire`
   - Bold / advertising / impactful → `pop-art`
   - Serious / dark narrative → `graphic-novel`
   - Cute / heartwarming / kids → `snoopy`
3. **Number of panels** — Typically 3-4 for a strip, 6-12 for a full story
4. **Layout** — Horizontal strip (default), vertical scroll, or grid
5. **API mode** — `rt` (fast, default) or `batch` (50% cheaper, slower)
6. **API key** — Check if GEMINI_API_KEY is configured. If not, ask the user to provide it.

Present a panel plan (descriptions) for confirmation before generating.

**Telegram inline keyboard example:**
```json
message(content="Choose a comic style:", metadata={"inline_keyboard": [
  [{"text": "xkcd", "callback_data": "style:xkcd"}, {"text": "manga 漫画", "callback_data": "style:manga"}],
  [{"text": "ligne-claire", "callback_data": "style:ligne-claire"}, {"text": "snoopy 史努比", "callback_data": "style:snoopy"}]
]})
```
User's button press arrives as `[callback] style:xkcd`.

## Styles (6)

| User says | `--style` | Theme | Best For |
|-----------|-----------|-------|----------|
| xkcd, stick figure, nerdy | `xkcd` | Stick figures, hand-drawn, minimal | Tech humor, explanations |
| manga, 漫画, anime | `manga` | Japanese manga, screentones, dramatic | Action, storytelling |
| ligne-claire, Tintin, 丁丁 | `ligne-claire` | Clean lines, flat colors, Tintin-style | Adventure, editorial |
| pop-art, Lichtenstein, 波普 | `pop-art` | Bold colors, halftone dots, Lichtenstein | Impactful, advertising |
| graphic-novel, 图像小说, dark | `graphic-novel` | Dark, detailed, atmospheric | Serious narratives |
| snoopy, Peanuts, 史努比 | `snoopy` | Charles Schulz Peanuts style, round heads | Cute, heartwarming, kids |
| "有哪些风格？" / "list styles" | Show all above | | |
| *(not specified)* | `xkcd` | | |

All styles use a single `panel` variant. The style TOML provides a detailed prompt prefix that sets the visual language for every panel.

## Layout Options

| `--layout` | Description | Best For |
|------------|-------------|----------|
| `horizontal` | Panels side-by-side in a row | 3-4 panel strips |
| `vertical` | Panels stacked top-to-bottom | Webtoon/scroll format |
| `grid` | Auto-arranged 2D grid (ceil(sqrt(n)) columns) | 4+ panels, posters |

## API Modes

| `--api` | Speed | Cost | How it works |
|---------|-------|------|--------------|
| `rt` (default) | Fast (~2-3 min) | Standard pricing | Parallel sync calls via rayon thread pool |
| `batch` | Slow (5-30 min) | **50% cheaper** | Gemini Batch API, async processing. Falls back to `rt` on timeout. |

Use `--api batch` for large jobs (10+ panels) where cost matters more than speed.

## Input JSON Schema

Top-level: array of panel objects.

### Panel Object

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `prompt` | string | **yes** | Panel content description — what to draw. Include character actions, expressions, speech bubbles, scene details. |
| `refine_prompt` | string | no | Qwen-Edit instruction for post-generation refinement (requires `--refine` flag and DASHSCOPE_API_KEY) |

### Prompt Writing Tips

- **Be specific**: "A programmer with messy hair stares at a monitor showing '99 bugs found'" beats "A programmer looking at bugs"
- **Include speech bubbles**: Write `Speech bubble: "text here"` in the prompt
- **Describe expressions**: "jaw drops", "eyes widen", "smirks"
- **Set the scene**: "dimly lit office", "sunny park bench", "crowded subway"
- **Number panels**: For coherent stories, include "Panel X of Y:" context

## Examples

### Simple 3-panel strip

```json
[
  {"prompt": "A programmer staring at a screen showing '99 bugs found'. Speech bubble: 'Fixed one bug...'"},
  {"prompt": "The screen now shows '117 bugs found'. The programmer's jaw drops in disbelief."},
  {"prompt": "The programmer closes the laptop and walks away into the sunset. Speech bubble: 'I quit.'"}
]
```

```bash
mofa comic --style xkcd --out /tmp/bugs.png --layout horizontal -i panels.json
```

### Multi-panel story with grid layout

```json
[
  {"prompt": "Wide establishing shot: A busy coffee shop. Title text: 'THE MEETING'"},
  {"prompt": "Close-up: Two people sit across a table. Person A holds a thick folder. Speech bubble A: 'I have a proposal.'"},
  {"prompt": "Person B raises an eyebrow skeptically. Speech bubble B: 'Go on...'"},
  {"prompt": "Person A opens the folder revealing a single page that says 'PIZZA FRIDAY'. Speech bubble A: 'It's revolutionary.'"},
  {"prompt": "Person B stares blankly. Long pause."},
  {"prompt": "Both are now eating pizza together, smiling. Text: 'Proposal approved unanimously.'"}
]
```

```bash
mofa comic --style ligne-claire --out /tmp/meeting.png --layout grid --gutter 30 -i story.json
```

### Manga with refinement

```json
[
  {"prompt": "Dramatic close-up of a samurai drawing a katana. Speed lines radiating outward. Text: 第一章", "refine_prompt": "Make the speed lines more dramatic and add motion blur"},
  {"prompt": "Wide shot: The samurai stands alone on a moonlit bridge. Cherry blossoms falling."},
  {"prompt": "Action shot: The samurai slashes through the air. SLASH sound effect in bold Japanese style."}
]
```

```bash
mofa comic --style manga --out /tmp/samurai.png --layout vertical --refine --image-size 2K -i manga.json
```

### Batch API for large comic

```bash
mofa comic --style graphic-novel --api batch --layout grid --gutter 20 --out /tmp/epic.png -i 12-panels.json
```

## CLI Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--style` | `xkcd` | Style name (see Styles table) |
| `-o` / `--out` | *required* | Final stitched output image path (PNG) |
| `--work-dir` | parent of --out | Working directory for individual panel PNGs |
| `--layout` | `horizontal` | `"horizontal"` / `"vertical"` / `"grid"` |
| `--concurrency` | 3 | Parallel generation workers (1-20) |
| `--image-size` | config | `"1K"` / `"2K"` / `"4K"` |
| `--refine` | false | Refine panels with Dashscope Qwen-Edit (needs DASHSCOPE_API_KEY) |
| `--gutter` | 20 | Gap between panels in pixels |
| `--api` | `rt` | API mode: `rt` (realtime, fast parallel) or `batch` (50% cheaper, async 5-30 min) |
| `-i` / `--input` | stdin | Input JSON file path |
| `--root` | auto-detected | Path to mofa root directory |

## Resolution & Quality

| Flag | Values | Description |
|------|--------|-------------|
| `--image-size` | `1K`, `2K`, `4K` | Per-panel resolution. Higher = sharper but slower and costlier. |
| `--gutter` | pixels | Gap between panels. 0 for seamless, 20-30 for classic comic spacing. |
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
    "comic": { "style": "xkcd", "panels": 3, "refine_with_qwen": false }
  }
}
```

**API keys**: `GEMINI_API_KEY` required for all generation. `DASHSCOPE_API_KEY` only needed for `--refine`.
**Models**: `gen_model` controls image generation model (default: `gemini-3.1-flash-image-preview`).

## Output

- Individual panels saved in `--work-dir` as `panel-01.png`, `panel-02.png`, ...
- Final stitched image at `--out` path
- Panels are cached: if `panel-XX.png` exists and is >10KB, it's reused (delete to regenerate)
- Gemini may return JPEG data saved as .png — the stitcher handles both formats automatically
