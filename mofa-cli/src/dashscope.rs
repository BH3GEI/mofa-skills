use base64::Engine;
use eyre::Result;
use serde_json::{json, Value};
use std::path::Path;

const DEFAULT_EDIT_MODEL: &str = "qwen-image-edit-max-2026-01-16";

/// A word/line detected by OCR with 4-corner bounding box (pixel coordinates).
/// Corners: top-left (x1,y1), top-right (x2,y2), bottom-right (x3,y3), bottom-left (x4,y4).
#[derive(Debug, Clone)]
pub struct OcrWord {
    pub text: String,
    pub x1: f64, pub y1: f64,
    pub x2: f64, pub y2: f64,
    pub x3: f64, pub y3: f64,
    pub x4: f64, pub y4: f64,
}

impl OcrWord {
    /// Bounding box left edge (min x).
    pub fn left(&self) -> f64 { self.x1.min(self.x4) }
    /// Bounding box top edge (min y).
    pub fn top(&self) -> f64 { self.y1.min(self.y2) }
    /// Bounding box width.
    pub fn width(&self) -> f64 { self.x2.max(self.x3) - self.left() }
    /// Bounding box height.
    pub fn height(&self) -> f64 { self.y3.max(self.y4) - self.top() }
    /// Estimated font size in points (height_px / 1.333).
    pub fn font_size_pt(&self) -> f64 { self.height() / 1.333 }
}

/// Dashscope API client for Qwen image editing.
pub struct DashscopeClient {
    api_key: String,
    http: reqwest::blocking::Client,
}

impl DashscopeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http: reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(300))
                .build()
                .unwrap(),
        }
    }

    /// Refine an image via Qwen-Edit using the multimodal-generation API.
    pub fn refine_image(
        &self,
        image_path: &Path,
        prompt: &str,
        out_file: &Path,
        model: Option<&str>,
    ) -> Result<std::path::PathBuf> {
        let model = model.unwrap_or(DEFAULT_EDIT_MODEL);

        // Resize large images to reduce payload
        let img = image::ImageReader::open(image_path)?
            .with_guessed_format()?
            .decode()?;
        let img = if img.width() > 2048 {
            let scale = 2048.0 / img.width() as f64;
            let new_h = (img.height() as f64 * scale) as u32;
            eprintln!(
                "  Dashscope: resizing {}x{} → 2048x{} for upload",
                img.width(),
                img.height(),
                new_h
            );
            img.resize_exact(2048, new_h, image::imageops::FilterType::Lanczos3)
        } else {
            img
        };

        // Encode as base64 JPEG for inline submission
        let mut jpeg_buf = std::io::Cursor::new(Vec::new());
        img.write_to(&mut jpeg_buf, image::ImageFormat::Jpeg)?;
        let b64 = base64::engine::general_purpose::STANDARD.encode(jpeg_buf.get_ref());
        let data_uri = format!("data:image/jpeg;base64,{b64}");
        eprintln!(
            "  Dashscope: encoded image as base64 ({}KB)",
            b64.len() / 1024
        );

        // Use the multimodal-generation endpoint with messages format
        let body = json!({
            "model": model,
            "input": {
                "messages": [
                    {
                        "role": "user",
                        "content": [
                            { "image": data_uri },
                            { "text": prompt }
                        ]
                    }
                ]
            },
            "parameters": {
                "n": 1,
                "watermark": false
            }
        });

        // Retry with backoff for rate limits
        let max_retries = 3;
        for attempt in 0..=max_retries {
            eprintln!("  Dashscope: submitting edit task (sync)...");

            let resp = self
                .http
                .post("https://dashscope.aliyuncs.com/api/v1/services/aigc/multimodal-generation/generation")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&body)
                .send()?;

            let data: Value = resp.json()?;

            // Check for rate limit
            if let Some(code) = data.get("code").and_then(|c| c.as_str()) {
                if code.contains("RateQuota") || code.contains("Throttling") {
                    if attempt < max_retries {
                        let wait = 10 * (attempt + 1) as u64;
                        eprintln!("  Dashscope: rate limited, retrying in {wait}s...");
                        std::thread::sleep(std::time::Duration::from_secs(wait));
                        continue;
                    }
                }
            }

            // Extract result image URL from response
            let img_url = data
                .pointer("/output/choices/0/message/content/0/image")
                .and_then(|u| u.as_str())
                .ok_or_else(|| eyre::eyre!("Dashscope edit failed: {data}"))?;

            return self.download_result(img_url, out_file);
        }
        unreachable!()
    }

    /// OCR an image using qwen-vl-ocr, returning word-level bounding boxes.
    /// Each result has `text` and `location` [x1,y1, x2,y2, x3,y3, x4,y4] in pixels.
    pub fn ocr_image(&self, image_path: &Path) -> Result<Vec<OcrWord>> {
        let img_data = std::fs::read(image_path)?;
        let b64 = base64::engine::general_purpose::STANDARD.encode(&img_data);
        let ext = image_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("png");
        let mime = if ext == "jpg" || ext == "jpeg" {
            "image/jpeg"
        } else {
            "image/png"
        };
        let data_uri = format!("data:{mime};base64,{b64}");

        let body = json!({
            "model": "qwen-vl-ocr",
            "input": {
                "messages": [
                    {
                        "role": "user",
                        "content": [
                            {
                                "image": data_uri,
                                "min_pixels": 3072,
                                "max_pixels": 8388608
                            },
                            {
                                "type": "text",
                                "text": "Read all text in this image."
                            }
                        ]
                    }
                ]
            },
            "parameters": {
                "ocr_options": {
                    "task": "advanced_recognition"
                }
            }
        });

        let max_retries = 3;
        for attempt in 0..=max_retries {
            let resp = self
                .http
                .post("https://dashscope.aliyuncs.com/api/v1/services/aigc/multimodal-generation/generation")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&body)
                .send()?;

            let data: Value = resp.json()?;

            if let Some(code) = data.get("code").and_then(|c| c.as_str()) {
                if (code.contains("RateQuota") || code.contains("Throttling")) && attempt < max_retries {
                    let wait = 10 * (attempt + 1) as u64;
                    eprintln!("  OCR: rate limited, retrying in {wait}s...");
                    std::thread::sleep(std::time::Duration::from_secs(wait));
                    continue;
                }
            }

            // Parse words_info from response
            let mut words = Vec::new();
            if let Some(choices) = data.pointer("/output/choices").and_then(|c| c.as_array()) {
                for choice in choices {
                    if let Some(content) = choice.pointer("/message/content").and_then(|c| c.as_array()) {
                        for item in content {
                            if let Some(ocr) = item.get("ocr_result") {
                                if let Some(infos) = ocr.get("words_info").and_then(|w| w.as_array()) {
                                    for info in infos {
                                        let text = info.get("text").and_then(|t| t.as_str()).unwrap_or("").to_string();
                                        let loc: Vec<f64> = info
                                            .get("location")
                                            .and_then(|l| l.as_array())
                                            .map(|arr| arr.iter().filter_map(|v| v.as_f64()).collect())
                                            .unwrap_or_default();
                                        if loc.len() == 8 && !text.is_empty() {
                                            words.push(OcrWord {
                                                text,
                                                x1: loc[0], y1: loc[1],
                                                x2: loc[2], y2: loc[3],
                                                x3: loc[4], y3: loc[5],
                                                x4: loc[6], y4: loc[7],
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            eprintln!("  OCR: detected {} text regions", words.len());
            return Ok(words);
        }
        unreachable!()
    }

    fn download_result(&self, img_url: &str, out_file: &Path) -> Result<std::path::PathBuf> {
        let img_resp = self.http.get(img_url).send()?;
        let bytes = img_resp.bytes()?;
        if let Some(parent) = out_file.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        std::fs::write(out_file, &bytes)?;
        eprintln!(
            "  Dashscope: done — {} ({}KB)",
            out_file.file_name().unwrap().to_string_lossy(),
            bytes.len() / 1024
        );
        Ok(out_file.to_path_buf())
    }
}
