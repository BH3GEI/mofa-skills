---
name: mofa-fm
description: Voice management and TTS with custom voice cloning. Save named voices and reuse them. Triggers: voice clone, save voice, custom voice, my voice, TTS, text to speech, 语音克隆, 自定义声音.
version: 1.0.0
author: hagency
always: false
---

# MoFA FM

Voice management and text-to-speech with custom voice cloning support via OminiX-MLX on Apple Silicon.

## Interaction Guide

Before generating speech, gather preferences interactively. On Telegram, use inline keyboard buttons:

1. **Text** — What should be spoken?
2. **Voice** — List available voices, let user pick:
   - Call `fm_voice_list` first to show preset + custom voices
   - Recommend based on language/gender preference
3. **Language** — chinese, english, japanese, korean
4. **Custom voice** — If user wants their own voice, ask them to send a 3-10s audio clip

**Telegram inline keyboard example:**
```json
message(content="Choose a voice:", metadata={"inline_keyboard": [
  [{"text": "Vivian", "callback_data": "voice:vivian"}, {"text": "Ryan", "callback_data": "voice:ryan"}],
  [{"text": "Serena", "callback_data": "voice:serena"}, {"text": "Aiden", "callback_data": "voice:aiden"}],
  [{"text": "🎤 Use my voice", "callback_data": "voice:custom"}]
]})
```
User's button press arrives as `[callback] voice:vivian`.

## Features

- **Text-to-Speech** with preset or custom voices
- **Voice Cloning**: Upload a 3-10s audio clip, save it as a named voice, reuse it anytime
- **Voice Management**: Save, list, and delete custom voice profiles

## Preset Voices

vivian (default), serena, ryan, aiden, eric, dylan, uncle_fu, ono_anna, sohee

## Custom Voice Workflow

1. User sends a voice clip (3-10 seconds of clear speech)
2. Agent calls `fm_voice_save` with the audio path and a name
3. For TTS, agent calls `fm_tts` with `voice` set to the saved name
4. List all voices with `fm_voice_list`
5. Delete a voice with `fm_voice_delete`

## Configuration

Set `OMINIX_API_URL` to point to the ominix-api server (default: `http://localhost:8080`).
Set `CREW_DATA_DIR` for per-profile voice storage (set automatically by crew gateway).

## Tools

### fm_tts

Synthesize speech from text. Supports preset voices and saved custom voices.

```json
{"text": "Hello world", "voice": "my_voice", "language": "english"}
```

### fm_voice_save

Save an audio file as a named custom voice.

```json
{"name": "my_voice", "audio_path": "/path/to/reference.wav"}
```

### fm_voice_list

List all available voices (preset + custom).

### fm_voice_delete

Delete a saved custom voice.

```json
{"name": "my_voice"}
```

## Languages

chinese, english, japanese, korean
