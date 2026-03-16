//! mofa-memory: Persistent vector memory across agent runs.
//!
//! Protocol (identical to mofa-fm):
//!   `./mofa-memory <tool_name>` with JSON on stdin, JSON on stdout.
//!
//! Tools:
//!   store_memory    — embed content and persist to SQLite
//!   retrieve_memory — semantic search via cosine similarity
//!   clear_memory    — delete by tag filter or all
//!   list_memories   — list stored entries with metadata

mod db;
mod embed;
mod similarity;

use std::io::Read;

use serde::Deserialize;
use serde_json::json;

// ── Input types ───────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct StoreInput {
    content: String,
    tags: Vec<String>,
    #[serde(default)]
    source: Option<String>,
}

#[derive(Deserialize)]
struct RetrieveInput {
    query: String,
    #[serde(default)]
    top_k: Option<usize>,
    #[serde(default)]
    min_score: Option<f32>,
}

#[derive(Deserialize)]
struct ClearInput {
    #[serde(default)]
    tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct ListInput {
    #[serde(default)]
    tags: Option<Vec<String>>,
}

// ── Protocol helpers (mirrors mofa-fm) ───────────────────────────────────────

fn fail(msg: &str) -> ! {
    let out = json!({"output": msg, "success": false});
    println!("{out}");
    std::process::exit(1);
}

fn succeed(msg: &str) -> ! {
    let out = json!({"output": msg, "success": true});
    println!("{out}");
    std::process::exit(0);
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let end: String = s.chars().take(max).collect();
        format!("{end}...")
    }
}

// ── OpenAI API key resolution ─────────────────────────────────────────────────

fn require_api_key() -> String {
    std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        fail("OPENAI_API_KEY environment variable is not set. \
              mofa-memory uses OpenAI text-embedding-3-small for semantic search.")
    })
}

// ── store_memory ──────────────────────────────────────────────────────────────

fn handle_store(input_json: &str) {
    let input: StoreInput = match serde_json::from_str(input_json) {
        Ok(v) => v,
        Err(e) => fail(&format!("Invalid input for store_memory: {e}")),
    };

    if input.content.trim().is_empty() {
        fail("'content' must not be empty");
    }
    if input.tags.is_empty() {
        fail("'tags' must contain at least one tag");
    }

    let api_key = require_api_key();

    let embedding = embed::embed(&input.content, &api_key).unwrap_or_else(|e| fail(&e));

    let conn = db::open_db().unwrap_or_else(|e| fail(&e));

    let id = uuid::Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now().to_rfc3339();

    let row = db::MemoryRow {
        id: id.clone(),
        content: input.content.clone(),
        tags: input.tags.clone(),
        source: input.source.clone(),
        embedding,
        created_at,
    };

    db::insert_memory(&conn, &row).unwrap_or_else(|e| fail(&e));

    let source_info = input
        .source
        .as_deref()
        .map(|s| format!(", source: {}", truncate(s, 60)))
        .unwrap_or_default();

    succeed(&format!(
        "Stored memory {id} ({} chars, tags: [{}]{source_info})",
        input.content.len(),
        input.tags.join(", "),
    ));
}

// ── retrieve_memory ───────────────────────────────────────────────────────────

fn handle_retrieve(input_json: &str) {
    let input: RetrieveInput = match serde_json::from_str(input_json) {
        Ok(v) => v,
        Err(e) => fail(&format!("Invalid input for retrieve_memory: {e}")),
    };

    if input.query.trim().is_empty() {
        fail("'query' must not be empty");
    }

    let top_k = input.top_k.unwrap_or(5).max(1);
    let min_score = input.min_score.unwrap_or(0.7).clamp(0.0, 1.0);

    let api_key = require_api_key();
    let query_embedding = embed::embed(&input.query, &api_key).unwrap_or_else(|e| fail(&e));

    let conn = db::open_db().unwrap_or_else(|e| fail(&e));
    let all_rows = db::query_all(&conn, None).unwrap_or_else(|e| fail(&e));

    if all_rows.is_empty() {
        succeed("No memories stored yet. Use store_memory to add some.");
    }

    // Score every row
    let mut scored: Vec<(f32, &db::MemoryRow)> = all_rows
        .iter()
        .map(|row| {
            let score = similarity::cosine(&query_embedding, &row.embedding);
            (score, row)
        })
        .filter(|(score, _)| *score >= min_score)
        .collect();

    // Sort by descending score
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(top_k);

    if scored.is_empty() {
        succeed(&format!(
            "No memories found with similarity >= {min_score:.2}. \
             Try lowering min_score or storing more related content."
        ));
    }

    // Build JSON output
    let results: Vec<serde_json::Value> = scored
        .iter()
        .map(|(score, row)| {
            json!({
                "id": row.id,
                "score": (score * 1000.0).round() / 1000.0,
                "content": row.content,
                "tags": row.tags,
                "source": row.source,
                "created_at": row.created_at,
            })
        })
        .collect();

    let output = serde_json::to_string_pretty(&results)
        .unwrap_or_else(|e| fail(&format!("Failed to serialize results: {e}")));

    succeed(&output);
}

// ── clear_memory ──────────────────────────────────────────────────────────────

fn handle_clear(input_json: &str) {
    let input: ClearInput = match serde_json::from_str(input_json) {
        Ok(v) => v,
        Err(e) => fail(&format!("Invalid input for clear_memory: {e}")),
    };

    let conn = db::open_db().unwrap_or_else(|e| fail(&e));
    let tag_filter = input.tags.as_deref();
    let deleted = db::delete_memories(&conn, tag_filter).unwrap_or_else(|e| fail(&e));

    if let Some(tags) = &input.tags {
        succeed(&format!(
            "Deleted {deleted} memories with tags: [{}]",
            tags.join(", ")
        ));
    } else {
        succeed(&format!("Deleted all {deleted} memories."));
    }
}

// ── list_memories ─────────────────────────────────────────────────────────────

fn handle_list(input_json: &str) {
    let input: ListInput = match serde_json::from_str(input_json) {
        Ok(v) => v,
        Err(e) => fail(&format!("Invalid input for list_memories: {e}")),
    };

    let conn = db::open_db().unwrap_or_else(|e| fail(&e));
    let tag_filter = input.tags.as_deref();
    let rows = db::query_all(&conn, tag_filter).unwrap_or_else(|e| fail(&e));

    if rows.is_empty() {
        let filter_note = input
            .tags
            .as_ref()
            .map(|t| format!(" with tags [{}]", t.join(", ")))
            .unwrap_or_default();
        succeed(&format!("No memories found{filter_note}."));
    }

    let mut output = format!("Stored memories ({}):\n\n", rows.len());
    for row in &rows {
        let preview = truncate(&row.content, 80);
        let source = row
            .source
            .as_deref()
            .map(|s| format!("  source:  {}\n", truncate(s, 60)))
            .unwrap_or_default();
        output.push_str(&format!(
            "id:       {}\ncreated:  {}\ntags:     [{}]\n{source}preview:  {preview}\n\n",
            row.id,
            row.created_at,
            row.tags.join(", "),
        ));
    }

    succeed(output.trim_end());
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let tool_name = args.get(1).map(|s| s.as_str()).unwrap_or("unknown");

    let mut buf = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut buf) {
        fail(&format!("Failed to read stdin: {e}"));
    }

    match tool_name {
        "store_memory"    => handle_store(&buf),
        "retrieve_memory" => handle_retrieve(&buf),
        "clear_memory"    => handle_clear(&buf),
        "list_memories"   => handle_list(&buf),
        _ => fail(&format!(
            "Unknown tool '{tool_name}'. Expected: store_memory, retrieve_memory, clear_memory, list_memories"
        )),
    }
}
