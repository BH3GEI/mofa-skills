---
name: videolizer
description: Generate SRT subtitles from text and audio using Whisper or basic timing.
requires_bins: python3,ffmpeg
requires_env: GEMINI_API_KEY
---

# Subtitle Generator

Generate `.srt` subtitle files from script text + audio.

## Trigger Phrases

- "生成字幕"
- "subtitle generation"
- "SRT file"
- "add subtitles"
- "字幕文件"
- "转字幕"

## Requirements

```bash
# Required
pip install moviepy

# Optional (better quality)
pip install openai-whisper
```

## Modes

| Mode | Tool | Quality |
|------|------|---------|
| whisper | `openai-whisper` | Word-level timestamps, high accuracy |
| basic | `moviepy` | Even distribution, fallback |

Auto-detection: uses whisper if installed, falls back to basic mode.

## Usage

```bash
# Generate subtitles
python3 -m videolizer subtitles --text script.txt --audio voice.wav --out subs.srt

# Or manually with whisper
whisper voice.wav --output_format srt --output_dir ./

# Or basic mode (estimate timing from audio duration)
python3 -c "
from moviepy.editor import AudioFileClip
audio = AudioFileClip('voice.wav')
duration = audio.duration
print(f'Audio duration: {duration}s')
"
```

## Workflow

1. **Write script** text file
2. **Generate audio** via TTS or recording
3. **Generate subtitles** with this skill
4. **Burn subtitles** into video with `ffmpeg`:

```bash
ffmpeg -i video.mp4 -vf "subtitles=subs.srt" -c:a copy output.mp4
```

## Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `VIDEOLIZER_MODE` | `auto` | `auto` / `whisper` / `basic` |
| `WHISPER_MODEL` | `base` | Whisper model size |
| `WORDS_PER_GROUP` | `3` | Words per subtitle entry |

## Fallback Behavior

If whisper fails:
1. Log warning
2. Switch to basic mode
3. Spread words evenly across audio duration
4. Never crash — always produce a valid SRT
