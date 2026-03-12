# mofa-skills

AI-powered content generation platform that turns text into visual media — presentations, greeting cards, comics, infographics, and animated videos — using Google Gemini and Alibaba Dashscope APIs.

## Skills

| Skill | Output | Styles | Description |
|-------|--------|--------|-------------|
| **mofa-slides** | `.pptx` | 14 | Presentation decks with full-bleed AI images and editable text overlays |
| **mofa-cards** | `.png` | 7 | Greeting cards in Chinese art styles (ink-wash, guochao, etc.) |
| **mofa-comic** | `.png` | 5 | Multi-panel comic strips (xkcd, manga, ligne-claire, etc.) |
| **mofa-infographic** | `.png` | 4 | Multi-section infographics stitched vertically |
| **mofa-video** | `.mp4` | 4 | Animated video cards with background music via Gemini Veo |
| **mofa-research** | text | — | 3-agent deep research pipeline (search → analyze → synthesize) |
| **mofa-videolizer** | `.srt` | — | Subtitle generation from text + audio (Whisper / basic fallback) |
| **mofa-workflow** | artifacts | — | Multi-agent team pipeline (architect → developer → reviewer → tester) |
| **mofa-news** | `.md` | 8 categories | News digest from Google News, Hacker News, Yahoo, Substack, Medium |
| **mofa-github** | text | — | GitHub integration via `gh` CLI (issues, PRs, CI, releases, API) |
| **mofa-public-apis** | text | 40+ categories | Search free public APIs — browse by category, auth, HTTPS, CORS |
| **mofa-mcdonalds** | text | — | McDonald's China ordering via MCP — 点餐、领券、积分兑换 |
| **mofa-xhs** | text | — | Xiaohongshu (小红书) integration — search, read, like, comment, publish |
| **mofa-crawler** | text | — | Web crawling via Cloudflare Browser Rendering API — full-site extraction with JS rendering |
| **mofa-logo** | SVG | 8 styles | AI logo generation with Claude Opus 4.6 — minimalist, mascot, emblem, wordmark |
| **mofa-fm** | `.wav` | — | Voice TTS with custom voice cloning via Qwen3-TTS on Apple Silicon |

## Architecture

```
mofa-skills/
├── mofa/                 # Shared engine (Node.js) + config
│   ├── lib/
│   │   ├── engine.js           # Image generation orchestration
│   │   ├── toml-style.js       # Style file loader
│   │   └── image-providers.js  # Gemini & Dashscope API clients
│   └── config.json             # API keys + defaults
│
├── mofa-slides/          # 14 presentation styles
├── mofa-cards/           # 7 greeting card styles
├── mofa-comic/           # 5 comic strip styles
├── mofa-infographic/     # 4 infographic styles
├── mofa-video/           # Video animation styles
├── mofa-research/        # DOT-based research pipeline
├── mofa-research-2.0/    # DeerFlow + mofa-research hybrid
├── mofa-videolizer/      # Subtitle generation (Whisper / basic)
├── mofa-workflow/        # DOT-based multi-agent team pipeline
├── mofa-news/            # News digest aggregator
├── mofa-github/          # GitHub integration via gh CLI
├── mofa-public-apis/     # Public API discovery (local cache)
├── mofa-mcdonalds/       # McDonald's China ordering via MCP Server
├── mofa-xhs/             # Xiaohongshu (小红书) integration via xhs-cli
├── mofa-crawler/         # Web crawling via Cloudflare Browser Rendering API
├── mofa-logo/            # AI logo generation with Claude Opus 4.6
├── mofa-fm/              # Voice TTS + cloning (Pure Rust, via ominix-api)
│
└── mofa-cli/             # Pure Rust CLI (single binary, no Node.js)
    └── src/
        ├── main.rs             # CLI entry (slides|cards|comic|infographic|video)
        ├── gemini.rs           # Gemini API client
        ├── dashscope.rs        # Qwen-Edit client
        ├── layout.rs           # VQA text extraction + font calibration
        ├── pptx.rs             # PPTX builder (DrawingML XML)
        ├── image_util.rs       # Image stitching
        ├── veo.rs              # Veo video generation
        └── pipeline/           # Per-skill generation pipelines
```

Two implementation stacks share the same config format, style system, and TOML templates:

- **JavaScript engine** (`mofa/`) — Node.js, requires `@google/genai`, `pptxgenjs`, `sharp`
- **Rust CLI** (`mofa-cli/`) — single binary, zero Node.js dependency

## Setup

### Prerequisites

- **GEMINI_API_KEY** — required for all skills
- **DASHSCOPE_API_KEY** — optional, for Qwen-Edit image refinement
- **Node.js** — for the JS engine
- **ffmpeg** — for video compositing (mofa-video)
- **ImageMagick** (`magick`) — for comic/infographic stitching (JS engine only)

### Configuration

Copy the example config and set your API keys:

```bash
cp mofa/config.example.json mofa/config.json
```

Edit `mofa/config.json` — API keys use `env:VAR_NAME` to read from environment variables:

```json
{
  "api_keys": {
    "gemini": "env:GEMINI_API_KEY",
    "dashscope": "env:DASHSCOPE_API_KEY"
  }
}
```

Or export them directly:

```bash
export GEMINI_API_KEY="your-key-here"
export DASHSCOPE_API_KEY="your-key-here"
```

### JavaScript engine

```bash
cd mofa && npm install
```

### Rust CLI

```bash
cd mofa-cli && cargo build --release
# Binary at mofa-cli/target/release/mofa-cli
```

## Usage (Rust CLI)

```bash
# Presentation deck
mofa slides --style nb-pro --out deck.pptx --slide-dir /tmp/slides --input slides.json

# Greeting cards
mofa cards --style cny-guochao --card-dir /tmp/cards --input cards.json

# Comic strip
mofa comic --style xkcd --out comic.png --input panels.json --layout horizontal

# Infographic
mofa infographic --style cyberpunk-neon --out poster.png --input sections.json

# Animated video card
mofa video --style video-card --anim-style shuimo --card-dir /tmp/video --input cards.json
```

See each skill's `SKILL.md` and `mofa-cli/README.md` for full API documentation.

## Style System

Styles are TOML files with prompt variants. Each skill directory has a `styles/` folder:

```toml
[meta]
name = "nb-pro"
display_name = "NB Pro"

[variants]
default = "normal"

[variants.normal]
prompt = """
Create a presentation slide image...
"""

[variants.cover]
prompt = """
Dark background, centered title...
"""
```

Adding a new style is as simple as dropping a `.toml` file into the appropriate `styles/` directory.

## Models

| Role | Default | Used By |
|------|---------|---------|
| Image generation | `gemini-3.1-flash-image-preview` | All visual skills |
| Vision QA | `gemini-2.5-flash` | autoLayout text extraction |
| Image editing | `qwen-image-edit-max-2026-01-16` | Text removal refinement |

All model names are configurable in `mofa/config.json`.

## License

[Apache License 2.0](LICENSE)
