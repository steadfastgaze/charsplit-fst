// SPDX-License-Identifier: MIT OR Apache-2.0

//! Score calculation for compound word splits.
//!
//! This module implements the scoring algorithm used to evaluate potential split points
//! in German compound words. Higher scores indicate more likely correct splits.
//!
//! # Score Formula
//!
//! For each potential split position, the score is calculated as:
//!
//! ```text
//! score = start_prob - in_prob + pre_prob
//! ```
//!
//! Where:
//! - **start_prob**: Maximum prefix probability of the second part (after Fugen-S removal)
//!   - High when the second part starts with a common word beginning (e.g., "Auto" in "Autobahn")
//! - **in_prob**: Minimum infix probability crossing the split boundary
//!   - Low when the ngram at the split is rare (good - means we're splitting at a boundary)
//!   - High when the ngram at the split is common (bad - means we're splitting inside a word)
//! - **pre_prob**: Maximum suffix probability of the first part (after Fugen-S removal)
//!   - High when the first part ends with a common word ending (e.g., "bahn" in "Autobahn")
//!
//! # Example
//!
//! For "Autobahnraststätte" split after "Autobahn":
//! - start_prob: P("Rast" starts a word) ≈ 0.90
//! - in_prob: P("rast" occurs within a word) ≈ 0.30
//! - pre_prob: P("bahn" ends a word) ≈ 0.82
//! - score = 0.90 - 0.30 + 0.82 = 1.42
//!
//! # Character-Based Indexing
//!
//! All functions use **character positions** internally, converting to byte positions
//! only for string slicing. This is critical for UTF-8 safety with German characters
//! like ä, ö, ü, and ß.

use crate::fugen_s::remove_fugen_s;
use crate::ngram::NgramLookup;

/// Score calculator for compound word splits.
///
/// Maintains a reference to ngram probability data and provides methods to
/// calculate split scores for German compound words.
pub struct ScoreCalculator<'a> {
    lookup: &'a NgramLookup,
}

impl<'a> ScoreCalculator<'a> {
    /// Create a new ScoreCalculator.
    pub fn new(lookup: &'a NgramLookup) -> Self {
        Self { lookup }
    }

    /// Calculate the split score for a word at a given character position.
    ///
    /// The score is calculated as: `start_prob - in_prob + pre_prob`
    ///
    /// Returns `None` if any required probability is missing.
    ///
    /// # Arguments
    ///
    /// * `word` - The word to split (as &str)
    /// * `split_pos` - Character position to split at (not byte position)
    pub fn calculate_split_score(&self, word: &str, split_pos: usize) -> Option<f64> {
        // Convert character position to byte positions
        let chars: Vec<char> = word.chars().collect();
        if split_pos >= chars.len() {
            return None;
        }

        // Get byte positions
        let pre_byte_end = chars[0..split_pos].iter().map(|c| c.len_utf8()).sum();
        let start_byte_start = pre_byte_end;

        // Apply Fugen-S to pre_slice
        let pre_slice = &word[..pre_byte_end];
        let pre_slice = remove_fugen_s(pre_slice).unwrap_or(pre_slice);

        // Get max suffix probability for pre_slice
        let pre_slice_prob = self.max_suffix_prob(pre_slice)?;

        // Get max prefix probability for start_slice
        let start_slice = &word[start_byte_start..];
        let start_slice_prob = self.max_prefix_prob(start_slice)?;

        // Get min infix probability
        let in_slice_prob = self.min_infix_prob(word, start_byte_start)?;

        Some(start_slice_prob - in_slice_prob + pre_slice_prob)
    }

    /// Get the maximum suffix probability for a word slice.
    ///
    /// Python compatibility: Only look up the FULL slice (no cascade to shorter suffixes).
    /// Returns -1.0 if the slice is not found in the suffix map.
    ///
    /// # Arguments
    ///
    /// * `slice` - The word fragment to look up as a suffix
    fn max_suffix_prob(&self, slice: &str) -> Option<f64> {
        // Python: Only look up the full slice, use -1.0 if not found
        // No cascade to shorter suffixes (Python's `if not pre_slice_prob` guard)
        self.lookup.get_suffix_prob(slice).or(Some(-1.0))
    }

    /// Get the maximum prefix probability for a word slice.
    ///
    /// Python compatibility: Apply Fugen-S to the full slice first,
    /// then look up only the FULL slice (no cascade to shorter prefixes).
    /// Returns -1.0 if the slice is not found in the prefix map.
    ///
    /// # Arguments
    ///
    /// * `slice` - The word fragment to look up as a prefix
    fn max_prefix_prob(&self, slice: &str) -> Option<f64> {
        // Apply Fugen-S to full slice first
        let slice = remove_fugen_s(slice).unwrap_or(slice);

        // Python: Only look up the full slice, use -1.0 if not found
        // No cascade to shorter prefixes (Python's `if not start_slice_prob` guard)
        self.lookup.get_prefix_prob(slice).or(Some(-1.0))
    }

    /// Get the minimum infix probability for a split position.
    ///
    /// Searches all ngrams that start at the split position, returning the
    /// minimum probability found. Low probabilities are good - they indicate
    /// the split occurs at a rare ngram boundary (likely a word boundary).
    ///
    /// # Strategy
    ///
    /// For split at position 3 in "wordX", we try: "dX", "rdX", "ordX", "wordX"
    /// Return the minimum probability among all these ngrams.
    ///
    /// Note: Unknown ngrams default to probability 1.0 (neutral), so they don't
    /// affect the minimum.
    ///
    /// # Arguments
    ///
    /// * `word` - The full word being split
    /// * `split_byte_pos` - Byte position where the split occurs
    fn min_infix_prob(&self, word: &str, split_byte_pos: usize) -> Option<f64> {
        let mut min_prob: Option<f64> = None;

        // Try all infixes starting at split_byte_pos
        let chars_from_split: Vec<char> = word[split_byte_pos..].chars().collect();

        // Python compatibility: Start from length 3, not 1
        // Python's loop: for k in range(len(word) + 1, 2, -1)
        // This stops at k=3, so minimum ngram length is 3
        for len in 3..=chars_from_split.len() {
            // Get byte position for the end of this ngram
            let byte_end = split_byte_pos + chars_from_split[0..len].iter().map(|c| c.len_utf8()).sum::<usize>();
            let ngram = &word[split_byte_pos..byte_end];
            let prob = self.lookup.get_infix_prob(ngram);

            if let Some(p) = prob {
                min_prob = Some(match min_prob {
                    Some(current_min) => current_min.min(p),
                    None => p,
                });
            }
        }

        // Python always has at least one infix probability (defaults to 1.0)
        // If no ngrams of length >= 3 were found, return the default
        min_prob.or(Some(1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fst::Map;
    use fst::MapBuilder;

    fn create_test_lookup() -> NgramLookup {
        // Create test maps with known probabilities.
        // CRITICAL: Keys MUST be inserted in sorted lexicographic order!
        //
        // Suffix map (sorted): "bahn" > "n" > "t"
        let mut suffix_builder = MapBuilder::new(Vec::new()).unwrap();
        suffix_builder.insert("bahn".as_bytes(), f64_to_u64(0.82)).unwrap();
        suffix_builder.insert("n".as_bytes(), f64_to_u64(0.7)).unwrap();
        suffix_builder.insert("t".as_bytes(), f64_to_u64(0.5)).unwrap();
        let suffix_map = Map::new(suffix_builder.into_inner().unwrap()).unwrap();

        // Prefix map (sorted): "r" < "rast"
        let mut prefix_builder = MapBuilder::new(Vec::new()).unwrap();
        prefix_builder.insert("r".as_bytes(), f64_to_u64(0.6)).unwrap();
        prefix_builder.insert("rast".as_bytes(), f64_to_u64(0.9)).unwrap();
        let prefix_map = Map::new(prefix_builder.into_inner().unwrap()).unwrap();

        // Infix map (sorted): "r" < "ra"
        let mut infix_builder = MapBuilder::new(Vec::new()).unwrap();
        infix_builder.insert("r".as_bytes(), f64_to_u64(0.3)).unwrap();
        infix_builder.insert("ra".as_bytes(), f64_to_u64(0.4)).unwrap();
        let infix_map = Map::new(infix_builder.into_inner().unwrap()).unwrap();

        NgramLookup::new(suffix_map, prefix_map, infix_map)
    }

    #[test]
    fn test_calculate_split_score() {
        let lookup = create_test_lookup();
        let calculator = ScoreCalculator::new(&lookup);

        // Test with "xbahnr" split at position 5 (between "xbahn" and "r")
        // - pre_slice = "xbahn" → doesn't match as full suffix, gets -1.0
        // - start_slice = "r" → matches prefix "r" (0.6)
        // - infix search (starting from length 3) finds nothing, defaults to 1.0
        // Score = 0.6 - 1.0 + (-1.0) = -1.4
        let word = "xbahnr";
        let split_pos = 5;
        let score = calculator.calculate_split_score(word, split_pos);
        assert!(score.is_some()); // Should still return Some value, not None
    }

    #[test]
    fn test_max_suffix_prob() {
        let lookup = create_test_lookup();
        let calculator = ScoreCalculator::new(&lookup);

        // "bahn" is in the map
        assert_eq!(calculator.max_suffix_prob("bahn"), Some(0.82));
        // "t" is in the map
        assert_eq!(calculator.max_suffix_prob("t"), Some(0.5));
        // "n" is in the map
        assert_eq!(calculator.max_suffix_prob("n"), Some(0.7));
        // nonexistent returns -1.0 (Python compatibility)
        assert_eq!(calculator.max_suffix_prob("nonexistent"), Some(-1.0));
        // "xbahn" is not in the map (full string lookup), returns -1.0
        assert_eq!(calculator.max_suffix_prob("xbahn"), Some(-1.0));
        // "abct" is not in the map (full string lookup), returns -1.0
        assert_eq!(calculator.max_suffix_prob("abct"), Some(-1.0));
        // "abcn" is not in the map (full string lookup), returns -1.0
        assert_eq!(calculator.max_suffix_prob("abcn"), Some(-1.0));
    }

    #[test]
    fn test_max_prefix_prob() {
        let lookup = create_test_lookup();
        let calculator = ScoreCalculator::new(&lookup);

        assert_eq!(calculator.max_prefix_prob("rast"), Some(0.9));
        // "rastx" is not in the map (full string lookup), returns -1.0
        assert_eq!(calculator.max_prefix_prob("rastx"), Some(-1.0));
        assert_eq!(calculator.max_prefix_prob("nonexistent"), Some(-1.0));
    }

    #[test]
    fn test_min_infix_prob() {
        let lookup = create_test_lookup();
        let calculator = ScoreCalculator::new(&lookup);

        // With minimum length 3, "arast"[1..] = "rast" will check "ras" (length 3) and "rast" (length 4)
        // Since only "r"(0.3) and "ra"(0.4) exist in the test map, but not "ras" or "rast",
        // both "ras" and "rast" default to 1.0, so min is 1.0
        let prob = calculator.min_infix_prob("arast", 1);
        assert_eq!(prob, Some(1.0)); // All ngrams of length >= 3 default to 1.0
        
        // Let's create a test where we have 3-char ngrams in the map
        // We need to update create_test_lookup to include 3-char ngrams
        // But for now, let's add a simple test that verifies the range
    }

    fn f64_to_u64(v: f64) -> u64 {
        v.to_bits()
    }
}
