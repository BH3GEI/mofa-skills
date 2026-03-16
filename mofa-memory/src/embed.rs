//! OpenAI embeddings API client for mofa-memory.
//!
//! Uses `text-embedding-3-small` (1536 dimensions) — the cheapest OpenAI embedding model.
//! Respects `OPENAI_BASE_URL` for routing through compatible gateways (e.g. r9s.ai, OpenRouter),
//! following the configurable base URL pattern introduced in mofa-skills PR #1.

use std::time::Duration;

use serde::Deserialize;

// ── API response types ────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct EmbedResponse {
    data: Vec<EmbedData>,
}

#[derive(Deserialize)]
struct EmbedData {
    embedding: Vec<f32>,
}

// ── embed ─────────────────────────────────────────────────────────────────────

/// Generate an embedding vector for the given text using OpenAI's API.
///
/// Environment variables:
///   - `OPENAI_API_KEY`  (required)
///   - `OPENAI_BASE_URL` (optional, defaults to "https://api.openai.com")
pub fn embed(content: &str, api_key: &str) -> Result<Vec<f32>, String> {
    let base_url = std::env::var("OPENAI_BASE_URL")
        .unwrap_or_else(|_| "https://api.openai.com".to_string());
    let base_url = base_url.trim_end_matches('/');
    let url = format!("{base_url}/v1/embeddings");

    let client = reqwest::blocking::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {e}"))?;

    let body = serde_json::json!({
        "model": "text-embedding-3-small",
        "input": content
    });

    let resp = client
        .post(&url)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .map_err(|e| format!("Embedding request failed: {e}"))?;

    let status = resp.status();
    if !status.is_success() {
        let text = resp.text().unwrap_or_default();
        return Err(format!(
            "OpenAI embeddings error (HTTP {status}): {}",
            truncate(&text, 300)
        ));
    }

    let parsed: EmbedResponse = resp
        .json()
        .map_err(|e| format!("Failed to parse embedding response: {e}"))?;

    parsed
        .data
        .into_iter()
        .next()
        .map(|d| d.embedding)
        .ok_or_else(|| "Embedding response contained no data".to_string())
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let end: String = s.chars().take(max).collect();
        format!("{end}...")
    }
}
