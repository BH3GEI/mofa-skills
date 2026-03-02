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
            Self::load(&path).unwrap_or_default()
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
        self.gen_model.as_deref().unwrap_or("gemini-3-pro-image-preview")
    }

    pub fn vision_model(&self) -> &str {
        self.vision_model.as_deref().unwrap_or("gemini-2.5-flash")
    }

    pub fn edit_model(&self) -> &str {
        self.edit_model.as_deref().unwrap_or("qwen-image-edit-max-2026-01-16")
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
