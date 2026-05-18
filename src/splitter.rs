// SPDX-License-Identifier: MIT OR Apache-2.0

//! Main splitter implementation for German compound words.

use crate::error::{Error, Result};
use crate::ngram::NgramLookup;
use crate::score::ScoreCalculator;
use fst::Map;
use std::fs;
use std::path::Path;

/// Result of splitting a compound word.
#[derive(Debug, Clone, PartialEq)]
pub struct SplitResult {
    /// The probability score for this split (higher is better).
    pub score: f64,
    /// First part of the compound (title-cased).
    pub part1: String,
    /// Second part of the compound (title-cased).
    pub part2: String,
}

/// German compound word splitter using FST data structures.
pub struct Splitter {
    ngram_lookup: NgramLookup,
}

#[cfg(feature = "embed-data")]
const SUFFIX_FST_BYTES: &[u8] = include_bytes!("../data/suffix.fst");
#[cfg(feature = "embed-data")]
const PREFIX_FST_BYTES: &[u8] = include_bytes!("../data/prefix.fst");
#[cfg(feature = "embed-data")]
const INFIX_FST_BYTES: &[u8] = include_bytes!("../data/infix.fst");

impl Splitter {
    /// Create a new Splitter.
    ///
    /// If the `embed-data` feature is enabled, it uses the embedded FST data.
    /// Otherwise, it attempts to load FST files from the "data" directory in the
    /// current working directory.
    ///
    /// # Errors
    ///
    /// Returns an error if FST data cannot be loaded or parsed.
    pub fn new() -> Result<Self> {
        #[cfg(feature = "embed-data")]
        {
            Self::from_raw_data(SUFFIX_FST_BYTES, PREFIX_FST_BYTES, INFIX_FST_BYTES)
        }
        #[cfg(not(feature = "embed-data"))]
        {
            Self::from_data_dir("data")
        }
    }

    /// Create a new Splitter from a custom data directory.
    ///
    /// # Errors
    ///
    /// Returns an error if any FST file cannot be read or parsed.
    pub fn from_data_dir<P: AsRef<Path>>(data_dir: P) -> Result<Self> {
        let data_dir = data_dir.as_ref();

        let suffix_bytes = fs::read(data_dir.join("suffix.fst"))?;
        let prefix_bytes = fs::read(data_dir.join("prefix.fst"))?;
        let infix_bytes = fs::read(data_dir.join("infix.fst"))?;

        let suffix_map = Map::new(suffix_bytes)
            .map_err(|e| Error::Fst(format!("Failed to load suffix map: {}", e)))?;
        let prefix_map = Map::new(prefix_bytes)
            .map_err(|e| Error::Fst(format!("Failed to load prefix map: {}", e)))?;
        let infix_map = Map::new(infix_bytes)
            .map_err(|e| Error::Fst(format!("Failed to load infix map: {}", e)))?;

        let ngram_lookup = NgramLookup::new(suffix_map, prefix_map, infix_map);

        Ok(Self { ngram_lookup })
    }

    /// Create a new Splitter from raw FST data bytes (useful for web/WASM).
    ///
    /// # Errors
    ///
    /// Returns an error if any FST data cannot be parsed.
    pub fn from_raw_data(
        suffix_bytes: &[u8],
        prefix_bytes: &[u8],
        infix_bytes: &[u8],
    ) -> Result<Self> {
        let suffix_map = Map::new(suffix_bytes.to_vec())
            .map_err(|e| Error::Fst(format!("Failed to load suffix map: {}", e)))?;
        let prefix_map = Map::new(prefix_bytes.to_vec())
            .map_err(|e| Error::Fst(format!("Failed to load prefix map: {}", e)))?;
        let infix_map = Map::new(infix_bytes.to_vec())
            .map_err(|e| Error::Fst(format!("Failed to load infix map: {}", e)))?;

        let ngram_lookup = NgramLookup::new(suffix_map, prefix_map, infix_map);

        Ok(Self { ngram_lookup })
    }

    /// Create a new Splitter with custom FST maps (useful for testing).
    #[must_use]
    pub const fn with_ngram_lookup(ngram_lookup: NgramLookup) -> Self {
        Self { ngram_lookup }
    }

    /// Split a German compound word into its component parts.
    ///
    /// Returns a list of possible splits, sorted by score (highest first).
    ///
    /// # Arguments
    ///
    /// * `word` - The compound word to split
    ///
    /// # Returns
    ///
    /// A vector of `SplitResult` containing all possible splits, sorted by score.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use charsplit_fst::Splitter;
    /// let splitter = Splitter::new().unwrap();
    /// let results = splitter.split_compound("Autobahnraststätte");
    ///
    /// assert!(!results.is_empty());
    /// println!("Best split: {} | {}", results[0].part1, results[0].part2);
    /// ```
    pub fn split_compound(&self, word: &str) -> Vec<SplitResult> {
        let word_lower = word.to_lowercase();

        // Hyphen handling (early exit) - rfind returns byte position
        if let Some(byte_pos) = word_lower.rfind('-') {
            // Hyphen is a single byte (ASCII), so byte_pos + 1 is safe
            return vec![SplitResult {
                score: 1.0,
                part1: to_title_case(&word_lower[..byte_pos]),
                part2: to_title_case(&word_lower[byte_pos + 1..]),
            }];
        }

        // Main algorithm
        let mut scores = Vec::new();
        let calculator = ScoreCalculator::new(&self.ngram_lookup);

        // Get character count (not byte count)
        let char_count = word_lower.chars().count();

        // Iterate through characters, start at position 3, go to 3rd last
        for split_pos in 3..char_count.saturating_sub(2) {
            if let Some(score) = calculator.calculate_split_score(&word_lower, split_pos) {
                // Convert character position to byte positions for slicing
                let chars: Vec<char> = word_lower.chars().collect();
                let byte_end: usize = chars[0..split_pos].iter().map(|c| c.len_utf8()).sum();
                let byte_start: usize = byte_end;

                scores.push(SplitResult {
                    score,
                    part1: to_title_case(&word_lower[..byte_end]),
                    part2: to_title_case(&word_lower[byte_start..]),
                });
            }
        }

        // Sort by score descending
        scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        if scores.is_empty() {
            vec![SplitResult {
                score: 0.0,
                part1: to_title_case(&word_lower),
                part2: to_title_case(&word_lower),
            }]
        } else {
            scores
        }
    }
}

impl Default for Splitter {
    fn default() -> Self {
        Self::new().expect("Failed to create default Splitter")
    }
}

/// Convert a string to title case (first character uppercase, rest lowercase).
fn to_title_case(s: &str) -> String {
    s.split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;
    use fst::Map;
    use fst::MapBuilder;

    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello"), "Hello");
        assert_eq!(to_title_case("HELLO"), "HELLO"); // Only first is uppercased
        assert_eq!(to_title_case("hELLO"), "HELLO");
        assert_eq!(to_title_case(""), "");
        assert_eq!(to_title_case("ä"), "Ä");
        assert_eq!(to_title_case("ß"), "SS"); // ß uppercases to SS
    }

    #[test]
    fn test_to_title_case_with_hyphens() {
        assert_eq!(to_title_case("bundes-autobahn"), "Bundes-Autobahn");
        assert_eq!(to_title_case("a-b-c"), "A-B-C");
        assert_eq!(to_title_case("a-"), "A-");
        assert_eq!(to_title_case("-b"), "-B");
    }

    #[test]
    fn test_hyphen_splitting() {
        let splitter = create_test_splitter();
        let result = splitter.split_compound("Bundes-Autobahn");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].score, 1.0);
        assert_eq!(result[0].part1, "Bundes");
        assert_eq!(result[0].part2, "Autobahn");
    }

    #[test]
    fn test_multiple_hyphens() {
        let splitter = create_test_splitter();
        let result = splitter.split_compound("Bundes-Autobahn-Kapitän");

        // Should split at the last hyphen
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].part1, "Bundes-Autobahn");
        assert_eq!(result[0].part2, "Kapitän");
    }

    #[test]
    fn test_case_preservation() {
        let splitter = create_test_splitter();
        let result = splitter.split_compound("bundes-autobahn");

        // Output should be title-cased regardless of input
        assert_eq!(result[0].part1, "Bundes");
        assert_eq!(result[0].part2, "Autobahn");
    }

    fn create_test_splitter() -> Splitter {
        // Create minimal test maps with dummy data
        let mut builder = MapBuilder::new(Vec::new()).unwrap();
        builder.insert(b"\x00", 0).unwrap();
        let dummy_map = Map::new(builder.into_inner().unwrap()).unwrap();

        Splitter::with_ngram_lookup(NgramLookup::new(
            dummy_map.clone(),
            dummy_map.clone(),
            dummy_map,
        ))
    }
}
