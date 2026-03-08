// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]

use eyre::{Result, WrapErr};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Default)]
pub struct MofaConfig {
    #[serde(default)]
    pub api_keys: HashMap<String, String>,
    #[serde(default)]
    pub defaults: Defaults,
    pub gen_model: Option<String>,
    pub vision_model: Option<String>,
    pub edit_model: Option<String>,
    /// OCR endpoint URL for grounded text extraction (e.g. "http://localhost:8080/v1/ocr").
    /// When set, auto-layout uses OCR+VQA mode (precise bounding boxes from OCR + font styling from VQA).
    /// When absent, falls back to VQA-only mode.
    pub ocr_url: Option<String>,
    /// Legacy alias for ocr_url.
    pub deepseek_ocr_url: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Defaults {
    pub slides: Option<SlideDefaults>,
    pub cards: Option<CardDefaults>,
    pub video: Option<VideoDefaults>,
    pub comic: Option<ComicDefaults>,
    pub infographic: Option<InfographicDefaults>,
}

#[derive(Deserialize, Debug, Default)]
pub struct SlideDefaults {
    pub style: Option<String>,
    pub image_size: Option<String>,
    pub concurrency: Option<usize>,
    pub auto_layout: Option<bool>,
}

#[derive(Deserialize, Debug, Default)]
pub struct CardDefaults {
    pub style: Option<String>,
    pub aspect_ratio: Option<String>,
    pub image_size: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct VideoDefaults {
    pub anim_style: Option<String>,
    pub bgm: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct ComicDefaults {
    pub style: Option<String>,
    pub panels: Option<usize>,
    pub refine_with_qwen: Option<bool>,
}

#[derive(Deserialize, Debug, Default)]
pub struct InfographicDefaults {
    pub style: Option<String>,
    pub panels: Option<usize>,
    pub refine_with_qwen: Option<bool>,
}

/// Resolve a value that may be `"env:VAR_NAME"` → env var lookup, or literal.
pub fn resolve_key(val: &str) -> Option<String> {
    if let Some(var) = val.strip_prefix("env:") {
        std::env::var(var).ok()
    } else {
        Some(val.to_string())
    }
}

impl MofaConfig {
    /// Load config from a config.json file.
    pub fn load(path: &Path) -> Result<Self> {
        let data = std::fs::read_to_string(path)
            .wrap_err_with(|| format!("reading config: {}", path.display()))?;
        let cfg: Self = serde_json::from_str(&data)?;
        Ok(cfg)
    }

    /// Load from the default location relative to the mofa root dir.
    pub fn load_default(mofa_root: &Path) -> Self {
        let path = mofa_root.join("mofa").join("config.json");
        if path.exists() {
            Self::load(&path).unwrap_or_else(|e| {
                eprintln!("Warning: failed to parse {}: {e}", path.display());
                Self::default()
            })
        } else {
            Self::default()
        }
    }

    /// Resolve the Gemini API key from config or env.
    pub fn gemini_key(&self) -> Option<String> {
        if let Some(val) = self.api_keys.get("gemini") {
            if let Some(k) = resolve_key(val) {
                return Some(k);
            }
        }
        std::env::var("GEMINI_API_KEY").ok()
    }

    /// Resolve the Dashscope API key from config or env.
    pub fn dashscope_key(&self) -> Option<String> {
        if let Some(val) = self.api_keys.get("dashscope") {
            if let Some(k) = resolve_key(val) {
                return Some(k);
            }
        }
        std::env::var("DASHSCOPE_API_KEY").ok()
    }

    pub fn gen_model(&self) -> &str {
        self.gen_model.as_deref().unwrap_or("gemini-3.1-flash-image-preview")
    }

    pub fn vision_model(&self) -> &str {
        self.vision_model.as_deref().unwrap_or("gemini-2.5-flash")
    }

    pub fn edit_model(&self) -> &str {
        self.edit_model.as_deref().unwrap_or("qwen-image-edit-max-2026-01-16")
    }

    /// Resolve the OCR endpoint URL from config or env.
    /// Checks `ocr_url` first, then legacy `deepseek_ocr_url`, then env vars.
    pub fn ocr_url(&self) -> Option<String> {
        if let Some(ref url) = self.ocr_url {
            return Some(resolve_key(url).unwrap_or_else(|| url.clone()));
        }
        if let Some(ref url) = self.deepseek_ocr_url {
            return Some(resolve_key(url).unwrap_or_else(|| url.clone()));
        }
        std::env::var("OCR_URL")
            .or_else(|_| std::env::var("DEEPSEEK_OCR_URL"))
            .ok()
    }
}

/// Find the mofa root directory by walking up from the binary or CWD.
pub fn find_mofa_root() -> PathBuf {
    // Try CWD first
    let cwd = std::env::current_dir().unwrap_or_default();
    if cwd.join("mofa").join("config.json").exists() {
        return cwd;
    }
    // Try relative to binary
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent().and_then(|p| p.parent()) {
            if parent.join("mofa").join("config.json").exists() {
                return parent.to_path_buf();
            }
        }
    }
    // Fallback: ~/.crew/skills/mofa parent
    if let Some(home) = dirs_fallback() {
        let skills = home.join(".crew").join("skills").join("mofa");
        if skills.join("config.json").exists() {
            return skills.parent().unwrap().parent().unwrap().to_path_buf();
        }
    }
    cwd
}

fn dirs_fallback() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}
