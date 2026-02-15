// SPDX-License-Identifier: MIT OR Apache-2.0

//! FST-based ngram probability lookup.
//!
//! This module provides efficient probability lookups using Finite State Transducer (FST)
//! maps. FSTs provide:
//! - **Memory efficiency**: Compressed representation of ngram → probability mappings
//! - **Fast lookups**: O(k) where k is the key length, not the number of entries
//! - **Immutable**: Maps are built once and queried many times (perfect for read-only data)
//!
//! # Data Format
//!
//! Probabilities are stored as `f64` values, converted to/from `u64` using bitwise
//! operations for lossless storage.
//!
//! # Thread Safety
//!
//! FST maps are wrapped in `Arc` for efficient sharing across threads without cloning.

use fst::Map;
use std::sync::Arc;

/// Ngram probability lookup using FST maps.
///
/// This struct wraps three FST maps:
/// - **suffix_map**: Probabilities of ngrams at the end of words (e.g., "bahn" in "Autobahn")
/// - **prefix_map**: Probabilities of ngrams at the start of words (e.g., "auto" in "Auto")
/// - **infix_map**: Probabilities of ngrams within words (used to penalize unlikely splits)
///
/// All maps are reference-counted (`Arc`) for efficient sharing.
pub struct NgramLookup {
    /// FST map for suffix probabilities (word endings)
    pub(crate) suffix_map: Arc<Map<Vec<u8>>>,
    /// FST map for prefix probabilities (word beginnings)
    pub(crate) prefix_map: Arc<Map<Vec<u8>>>,
    /// FST map for infix probabilities (internal ngrams)
    pub(crate) infix_map: Arc<Map<Vec<u8>>>,
}

impl NgramLookup {
    /// Create a new NgramLookup from FST maps.
    pub fn new(
        suffix_map: Map<Vec<u8>>,
        prefix_map: Map<Vec<u8>>,
        infix_map: Map<Vec<u8>>,
    ) -> Self {
        Self {
            suffix_map: Arc::new(suffix_map),
            prefix_map: Arc::new(prefix_map),
            infix_map: Arc::new(infix_map),
        }
    }

    /// Get suffix probability for an ngram.
    ///
    /// Returns `None` if the ngram is not found in the suffix map.
    ///
    /// # Arguments
    ///
    /// * `ngram` - The ngram to look up (typically the ending of a word fragment)
    ///
    /// # Returns
    ///
    /// * `Some(f64)` - The probability if found
    /// * `None` - If the ngram doesn't exist in our training data
    pub fn get_suffix_prob(&self, ngram: &str) -> Option<f64> {
        self.suffix_map
            .get(ngram.as_bytes())
            .map(|v| f64::from_bits(v))  // Lossless u64 → f64 conversion
    }

    /// Get prefix probability for an ngram.
    ///
    /// Returns `None` if the ngram is not found in the prefix map.
    ///
    /// # Arguments
    ///
    /// * `ngram` - The ngram to look up (typically the beginning of a word fragment)
    ///
    /// # Returns
    ///
    /// * `Some(f64)` - The probability if found
    /// * `None` - If the ngram doesn't exist in our training data
    pub fn get_prefix_prob(&self, ngram: &str) -> Option<f64> {
        self.prefix_map
            .get(ngram.as_bytes())
            .map(|v| f64::from_bits(v))  // Lossless u64 → f64 conversion
    }

    /// Get infix probability for an ngram.
    ///
    /// Returns `Some(1.0)` if the ngram is not found (default probability).
    /// This default means "unknown ngrams have no penalty" - they don't affect the score.
    ///
    /// # Arguments
    ///
    /// * `ngram` - The ngram to look up (crosses a potential split boundary)
    ///
    /// # Returns
    ///
    /// * `Some(f64)` - Always returns `Some`, with 1.0 as default for unknown ngrams
    pub fn get_infix_prob(&self, ngram: &str) -> Option<f64> {
        self.infix_map
            .get(ngram.as_bytes())
            .map(|v| f64::from_bits(v))  // Lossless u64 → f64 conversion
            .or(Some(1.0))  // Default: unknown ngrams are neutral
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fst::Map;
    use fst::MapBuilder;

    #[test]
    fn test_get_suffix_prob() {
        // Create test map with sample ngrams
        let mut builder = MapBuilder::new(Vec::new()).unwrap();
        builder.insert("bahn".as_bytes(), f64_to_u64(0.82)).unwrap();
        builder.insert("haus".as_bytes(), f64_to_u64(0.75)).unwrap();
        let suffix_map = Map::new(builder.into_inner().unwrap()).unwrap();

        // Create minimal empty maps for other lookups
        // Note: FSTs can't be truly empty - we insert a dummy null byte
        let mut empty_builder = MapBuilder::new(Vec::new()).unwrap();
        empty_builder.insert(b"\0", 0).unwrap();
        let empty_map = Map::new(empty_builder.into_inner().unwrap()).unwrap();

        let lookup = NgramLookup::new(suffix_map, empty_map.clone(), empty_map);

        assert_eq!(lookup.get_suffix_prob("bahn"), Some(0.82));
        assert_eq!(lookup.get_suffix_prob("haus"), Some(0.75));
        assert_eq!(lookup.get_suffix_prob("nonexistent"), None);
    }

    #[test]
    fn test_get_prefix_prob() {
        let mut builder = MapBuilder::new(Vec::new()).unwrap();
        builder.insert("auto".as_bytes(), f64_to_u64(0.95)).unwrap();
        let prefix_map = Map::new(builder.into_inner().unwrap()).unwrap();

        // Create minimal empty maps (see test_get_suffix_prob for explanation)
        let mut empty_builder = MapBuilder::new(Vec::new()).unwrap();
        empty_builder.insert(b"\0", 0).unwrap();
        let empty_map = Map::new(empty_builder.into_inner().unwrap()).unwrap();

        let lookup = NgramLookup::new(empty_map.clone(), prefix_map, empty_map);

        assert_eq!(lookup.get_prefix_prob("auto"), Some(0.95));
        assert_eq!(lookup.get_prefix_prob("nonexistent"), None);
    }

    #[test]
    fn test_get_infix_prob() {
        let mut builder = MapBuilder::new(Vec::new()).unwrap();
        builder.insert("abc".as_bytes(), f64_to_u64(0.5)).unwrap();
        let infix_map = Map::new(builder.into_inner().unwrap()).unwrap();

        // Create minimal empty maps (see test_get_suffix_prob for explanation)
        let mut empty_builder = MapBuilder::new(Vec::new()).unwrap();
        empty_builder.insert(b"\0", 0).unwrap();
        let empty_map = Map::new(empty_builder.into_inner().unwrap()).unwrap();

        let lookup = NgramLookup::new(empty_map.clone(), empty_map, infix_map);

        assert_eq!(lookup.get_infix_prob("abc"), Some(0.5));
        // Nonexistent infixes return default 1.0 (neutral probability)
        assert_eq!(lookup.get_infix_prob("nonexistent"), Some(1.0));
    }

    fn f64_to_u64(v: f64) -> u64 {
        v.to_bits()
    }
}
