//! Cosine similarity for comparing embedding vectors.

/// Compute cosine similarity between two float vectors.
///
/// Returns a value in [-1.0, 1.0] where 1.0 means identical direction.
/// Returns 0.0 for zero-length or mismatched-dimension vectors (logged to stderr).
///
/// Note: OpenAI `text-embedding-3-small` vectors are L2-normalized, so
/// cosine similarity equals dot product in practice. The explicit norm
/// computation is kept for correctness with other embedding providers.
pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        eprintln!(
            "[mofa-memory] WARNING: embedding dimension mismatch ({} vs {}), returning 0.0",
            a.len(),
            b.len()
        );
        return 0.0;
    }
    if a.is_empty() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    (dot / (norm_a * norm_b)).clamp(-1.0, 1.0)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 1e-6;

    #[test]
    fn identical_vectors_score_one() {
        let v = vec![1.0f32, 2.0, 3.0];
        assert!((cosine(&v, &v) - 1.0).abs() < EPS);
    }

    #[test]
    fn orthogonal_vectors_score_zero() {
        let a = vec![1.0f32, 0.0, 0.0];
        let b = vec![0.0f32, 1.0, 0.0];
        assert!(cosine(&a, &b).abs() < EPS);
    }

    #[test]
    fn opposite_vectors_score_minus_one() {
        let a = vec![1.0f32, 0.0];
        let b = vec![-1.0f32, 0.0];
        assert!((cosine(&a, &b) + 1.0).abs() < EPS);
    }

    #[test]
    fn mismatched_dimensions_returns_zero() {
        let a = vec![1.0f32, 2.0];
        let b = vec![1.0f32];
        assert_eq!(cosine(&a, &b), 0.0);
    }

    #[test]
    fn empty_vectors_return_zero() {
        assert_eq!(cosine(&[], &[]), 0.0);
    }

    #[test]
    fn unit_vectors_at_45_degrees() {
        let s = std::f32::consts::FRAC_1_SQRT_2;
        let a = vec![s, s];
        let b = vec![1.0f32, 0.0];
        assert!((cosine(&a, &b) - s).abs() < EPS);
    }

    #[test]
    fn result_is_clamped_to_valid_range() {
        // Due to floating point, dot/norm can slightly exceed 1.0 — clamp handles this
        let v = vec![1.0f32];
        let score = cosine(&v, &v);
        assert!(score >= -1.0 && score <= 1.0);
    }
}
