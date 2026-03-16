//! SQLite persistence layer for mofa-memory.
//!
//! Database path resolution (in priority order):
//!   1. `MOFA_MEMORY_DB` environment variable
//!   2. `~/.mofa/memory.db`
//!   3. `/tmp/mofa-memory.db` (fallback when $HOME is unset)

use std::path::PathBuf;

use rusqlite::{Connection, Result as SqlResult, params};
use serde::{Deserialize, Serialize};

// ── MemoryRow ─────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryRow {
    pub id: String,
    pub content: String,
    pub tags: Vec<String>,
    pub source: Option<String>,
    pub embedding: Vec<f32>,
    pub created_at: String,
}

// ── Path resolution ───────────────────────────────────────────────────────────

pub fn db_path() -> PathBuf {
    // 1. Explicit env var override
    if let Ok(path) = std::env::var("MOFA_MEMORY_DB") {
        return PathBuf::from(path);
    }

    // 2. ~/.mofa/memory.db
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home).join(".mofa").join("memory.db");
    }

    // 3. Fallback for environments without $HOME (containers, CI)
    eprintln!("[mofa-memory] WARNING: $HOME not set, using /tmp/mofa-memory.db");
    PathBuf::from("/tmp/mofa-memory.db")
}

// ── Schema ────────────────────────────────────────────────────────────────────

const CREATE_TABLE: &str = "
CREATE TABLE IF NOT EXISTS memories (
    id         TEXT PRIMARY KEY,
    content    TEXT NOT NULL,
    tags       TEXT NOT NULL,
    source     TEXT,
    embedding  TEXT NOT NULL,
    created_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_memories_created ON memories(created_at);
";

// ── open_db ───────────────────────────────────────────────────────────────────

pub fn open_db() -> Result<Connection, String> {
    let path = db_path();

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Cannot create memory directory {}: {e}", parent.display()))?;
    }

    let conn = Connection::open(&path)
        .map_err(|e| format!("Cannot open SQLite at {}: {e}", path.display()))?;

    conn.execute_batch(CREATE_TABLE)
        .map_err(|e| format!("Schema init failed: {e}"))?;

    Ok(conn)
}

// ── insert_memory ─────────────────────────────────────────────────────────────

pub fn insert_memory(conn: &Connection, row: &MemoryRow) -> Result<(), String> {
    let tags_json = serde_json::to_string(&row.tags)
        .map_err(|e| format!("Failed to serialize tags: {e}"))?;
    let embedding_json = serde_json::to_string(&row.embedding)
        .map_err(|e| format!("Failed to serialize embedding: {e}"))?;

    conn.execute(
        "INSERT OR REPLACE INTO memories (id, content, tags, source, embedding, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            row.id,
            row.content,
            tags_json,
            row.source,
            embedding_json,
            row.created_at,
        ],
    )
    .map_err(|e| format!("INSERT failed: {e}"))?;

    Ok(())
}

// ── query_all ─────────────────────────────────────────────────────────────────

/// Load all rows, optionally filtering to those that contain ANY of the given tags.
///
/// Tag matching is done in Rust rather than SQL to avoid relying on SQLite's
/// JSON functions (available only in SQLite >= 3.38, not guaranteed on all platforms).
pub fn query_all(
    conn: &Connection,
    tag_filter: Option<&[String]>,
) -> Result<Vec<MemoryRow>, String> {
    let mut stmt = conn
        .prepare("SELECT id, content, tags, source, embedding, created_at FROM memories ORDER BY created_at DESC")
        .map_err(|e| format!("Prepare failed: {e}"))?;

    let rows: SqlResult<Vec<MemoryRow>> = stmt
        .query_map([], |row| {
            let tags_json: String = row.get(2)?;
            let source: Option<String> = row.get(3)?;
            let embedding_json: String = row.get(4)?;

            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            let embedding: Vec<f32> = serde_json::from_str(&embedding_json).unwrap_or_default();

            Ok(MemoryRow {
                id: row.get(0)?,
                content: row.get(1)?,
                tags,
                source,
                embedding,
                created_at: row.get(5)?,
            })
        })
        .map_err(|e| format!("Query failed: {e}"))?
        .collect();

    let all = rows.map_err(|e| format!("Row read failed: {e}"))?;

    // Apply tag filter in Rust
    if let Some(filter_tags) = tag_filter {
        if filter_tags.is_empty() {
            return Ok(all);
        }
        let filtered = all
            .into_iter()
            .filter(|row| {
                row.tags
                    .iter()
                    .any(|t| filter_tags.iter().any(|f| f.eq_ignore_ascii_case(t)))
            })
            .collect();
        return Ok(filtered);
    }

    Ok(all)
}

// ── delete_memories ───────────────────────────────────────────────────────────

/// Delete memories. If `tag_filter` is None, deletes ALL memories.
/// If `tag_filter` is Some, deletes only memories with at least one matching tag.
pub fn delete_memories(
    conn: &Connection,
    tag_filter: Option<&[String]>,
) -> Result<usize, String> {
    match tag_filter {
        None => {
            // Delete everything
            let count = conn
                .execute("DELETE FROM memories", [])
                .map_err(|e| format!("DELETE failed: {e}"))?;
            Ok(count)
        }
        Some(tags) => {
            if tags.is_empty() {
                return Ok(0);
            }
            // Load matching IDs first, then delete by ID
            let matching = query_all(conn, Some(tags))?;
            let mut deleted = 0;
            for row in &matching {
                deleted += conn
                    .execute("DELETE FROM memories WHERE id = ?1", params![row.id])
                    .map_err(|e| format!("DELETE row {} failed: {e}", row.id))?;
            }
            Ok(deleted)
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory DB");
        conn.execute_batch(CREATE_TABLE).expect("schema");
        conn
    }

    fn make_row(id: &str, content: &str, tags: &[&str]) -> MemoryRow {
        MemoryRow {
            id: id.to_string(),
            content: content.to_string(),
            tags: tags.iter().map(|s| s.to_string()).collect(),
            source: None,
            embedding: vec![1.0, 0.0, 0.0],
            created_at: "2026-03-15T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn insert_and_query_round_trip() {
        let conn = test_conn();
        let row = make_row("id1", "hello world", &["test", "rust"]);
        insert_memory(&conn, &row).unwrap();

        let results = query_all(&conn, None).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content, "hello world");
        assert_eq!(results[0].tags, vec!["test", "rust"]);
    }

    #[test]
    fn tag_filter_returns_only_matching() {
        let conn = test_conn();
        insert_memory(&conn, &make_row("a", "about rust", &["rust", "systems"])).unwrap();
        insert_memory(&conn, &make_row("b", "about python", &["python"])).unwrap();
        insert_memory(&conn, &make_row("c", "about web", &["js", "web"])).unwrap();

        let filter = vec!["rust".to_string()];
        let results = query_all(&conn, Some(&filter)).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "a");
    }

    #[test]
    fn delete_by_tag_leaves_others() {
        let conn = test_conn();
        insert_memory(&conn, &make_row("a", "keep this", &["keep"])).unwrap();
        insert_memory(&conn, &make_row("b", "delete this", &["trash"])).unwrap();

        let deleted = delete_memories(&conn, Some(&["trash".to_string()])).unwrap();
        assert_eq!(deleted, 1);

        let remaining = query_all(&conn, None).unwrap();
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].id, "a");
    }

    #[test]
    fn delete_all_when_no_filter() {
        let conn = test_conn();
        insert_memory(&conn, &make_row("a", "one", &["x"])).unwrap();
        insert_memory(&conn, &make_row("b", "two", &["y"])).unwrap();

        let deleted = delete_memories(&conn, None).unwrap();
        assert_eq!(deleted, 2);

        let remaining = query_all(&conn, None).unwrap();
        assert!(remaining.is_empty());
    }

    #[test]
    fn empty_tag_filter_returns_all() {
        let conn = test_conn();
        insert_memory(&conn, &make_row("a", "one", &["x"])).unwrap();
        insert_memory(&conn, &make_row("b", "two", &["y"])).unwrap();

        let results = query_all(&conn, Some(&[])).unwrap();
        assert_eq!(results.len(), 2);
    }
}
