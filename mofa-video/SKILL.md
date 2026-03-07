---
name: mofa-video
description: "AI-generated animated video cards with BGM. Triggers: video card, animated card, 动态贺卡, mofa video, animate card, video greeting. Pipeline: Gemini image gen → Veo animation → ffmpeg compositing → MP4 with background music."
requires_bins: mofa, ffmpeg
requires_env: GEMINI_API_KEY
---

# mofa-video

CLI: `mofa video`
Styles: `mofa-video/styles/video-card.toml` (animation prompts)
Card styles: `mofa-cards/styles/*.toml` (image generation)
Config: `mofa/config.json`
BGM: `mofa/bgm-cny.mp3`, `mofa/bgm-chinese.mp3`

## Interaction Guide

Before generating, gather preferences interactively. On Telegram, use inline keyboard buttons:

1. **Scene/content** — What should the video card depict?
2. **Card style** — Uses mofa-cards styles for image generation (see mofa-cards for options)
3. **Animation style** — Recommend based on mood:
   - Meditative / peaceful → `shuimo`
   - Festive / celebration → `festive`
   - Romantic / dreamy → `gentle`
   - Energetic / lively → `dynamic`
4. **Background music** — Ask if they want BGM (available: bgm-cny.mp3, bgm-chinese.mp3)
5. **Number of scenes** — Typically 1-3 for a video card
6. **API key** — Check if GEMINI_API_KEY is configured. If not, ask the user to provide it. Also requires `ffmpeg` installed.

**Telegram inline keyboard example:**
```json
message(content="Choose animation style:", metadata={"inline_keyboard": [
  [{"text": "水墨 shuimo", "callback_data": "anim:shuimo"}, {"text": "喜庆 festive", "callback_data": "anim:festive"}],
  [{"text": "温柔 gentle", "callback_data": "anim:gentle"}, {"text": "动感 dynamic", "callback_data": "anim:dynamic"}]
]})
```
User's button press arrives as `[callback] anim:shuimo`.

## Quick Start

```bash
echo '[
  {"name": "scene1", "style": "front", "prompt": "A figure under a flowering tree...",
   "anim_style": "shuimo", "anim_desc": "Petals drifting, leaves swaying gently..."}
]' | mofa video --style video-card --anim-style shuimo --card-dir video-output --bgm mofa/bgm-cny.mp3
```

## Animation Styles

| Tag | Style | Motion |
|-----|-------|--------|
| `shuimo` | 水墨 meditative | Leaves swaying, steam rising, clouds drifting |
| `festive` | 喜庆 lively | Lanterns swaying, firecrackers, plum blossoms |
| `gentle` | 温柔 dreamy | Hair swaying, petals drifting, soft particles |
| `dynamic` | 动感 energetic | Characters gesturing, water flowing, birds flying |

## Input JSON

```json
[
  { "name": "scene1", "prompt": "...", "style": "front",
    "anim_style": "shuimo", "anim_desc": "Scene-specific motion details" }
]
```

## CLI Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--style` | `video-card` | Image style name |
| `--anim-style` | `shuimo` | Animation style name |
| `--card-dir` | required | Output directory for PNGs and MP4s |
| `--bgm` | - | Background music file path |
| `--aspect` | `9:16` | Image aspect ratio |
| `--image-size` | - | `"1K"` / `"2K"` / `"4K"` |
| `--concurrency` | 3 | Parallel limit for image gen |
| `--still-duration` | 2.0 | Seconds to hold still image |
| `--crossfade-dur` | 1.0 | Crossfade duration |
| `--fade-out-dur` | 1.5 | Fade out duration |
| `--music-volume` | 0.3 | BGM volume (0-1) |
| `--music-fade-in` | 2.0 | Music fade in duration |
| `-i` / `--input` | stdin | Input JSON file |

## Config

`mofa/config.json`:

**API keys**: `"env:GEMINI_API_KEY"` — set via `export GEMINI_API_KEY="your-key"`
**Models**: `gen_model` (image gen), `edit_model` (Qwen refinement).
