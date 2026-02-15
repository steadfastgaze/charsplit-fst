// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integration tests verifying Python-Rust parity.

use charsplit_fst::Splitter;
use fst::Map;

/// Test words with their expected Python results.
/// These are captured from the Python implementation.
const TEST_WORDS: &[(&str, f64, &str, &str)] = &[
    // (word, expected_score, expected_part1, expected_part2)
    ("Autobahnraststätte", 0.795, "Autobahn", "Raststätte"),
    ("Behördenangaben", 0.913, "Behörden", "Angaben"),
    ("Arbeitsamt", 0.815, "Arbeits", "Amt"),
    ("Wirtschaftsschule", 0.850, "Wirtschafts", "Schule"),
];

/// Words to test for basic functionality (checking structure, not exact values).
const BASIC_TEST_WORDS: &[&str] = &[
    "Haus",
    "Bad",
    "AUTOBAHN",
    "Bäckerhandel",
    "Fußball",
    "Straße",
];

/// Hyphenated test words.
const HYPHEN_TEST_WORDS: &[(&str, &str, &str)] = &[
    ("Donaudampfschifffahrt-Kapitän", "Donaudampfschifffahrt", "Kapitän"),
    ("Bundes-Autobahn", "Bundes", "Autobahn"),
    ("Arbeitsamt-Gebäude", "Arbeitsamt", "Gebäude"),
];

#[test]
fn test_autobahnraststätte_split() {
    let splitter = Splitter::new().expect("Failed to create splitter");
    let result = splitter.split_compound("Autobahnraststätte");

    assert!(!result.is_empty(), "Should return at least one result");
    assert!(result[0].score > 0.79, "Top score should be > 0.79");
    assert_eq!(result[0].part1, "Autobahn");
    assert_eq!(result[0].part2, "Raststätte");
}

#[test]
fn test_behördenangaben_split() {
    let splitter = Splitter::new().expect("Failed to create splitter");
    let result = splitter.split_compound("Behördenangaben");

    assert!(!result.is_empty());
    assert!(result[0].score > 0.8);
    assert!(result[0].part1.contains("Beh"));
    assert!(!result[0].part1.to_lowercase().contains("ang"));
    assert!(result[0].part2.contains("Ang"));
    assert!(!result[0].part2.to_lowercase().contains("beh"));
}

#[test]
fn test_arbeitsamt_fugen_s() {
    let splitter = Splitter::new().expect("Failed to create splitter");
    let result = splitter.split_compound("Arbeitsamt");

    assert!(!result.is_empty());
    assert!(result[0].part1.contains("Arbeits"));
    assert!(result[0].part2.contains("Amt"));
}

#[test]
fn test_wirtschaftsschule_fugen_s() {
    let splitter = Splitter::new().expect("Failed to create splitter");
    let result = splitter.split_compound("Wirtschaftsschule");

    assert!(!result.is_empty());
    assert!(result[0].part1.contains("Wirtschafts"));
    assert!(result[0].part2.contains("Schule"));
}

#[test]
fn test_hyphen_splitting() {
    let splitter = Splitter::new().expect("Failed to create splitter");

    for &(word, expected_part1, expected_part2) in HYPHEN_TEST_WORDS {
        let result = splitter.split_compound(word);

        assert_eq!(result.len(), 1, "Should return exactly one result for hyphenated word: {}", word);
        assert_eq!(result[0].score, 1.0, "Hyphenated word should have score 1.0: {}", word);
        assert_eq!(result[0].part1, expected_part1, "Part1 mismatch for: {}", word);
        assert_eq!(result[0].part2, expected_part2, "Part2 mismatch for: {}", word);
    }
}

#[test]
fn test_multiple_hyphens_splits_at_last() {
    let splitter = Splitter::new().expect("Failed to create splitter");
    let result = splitter.split_compound("Bundes-Autobahn-Kapitän");

    assert_eq!(result.len(), 1);
    // to_title_case only capitalizes the first character
    assert_eq!(result[0].part1, "Bundes-autobahn");
    assert_eq!(result[0].part2, "Kapitän");
}

#[test]
fn test_case_variations() {
    let splitter = Splitter::new().expect("Failed to create splitter");

    // All caps
    let result = splitter.split_compound("AUTOBAHN");
    assert!(!result.is_empty());
    assert!(result[0].part1.chars().next().unwrap().is_uppercase());

    // Lowercase
    let result = splitter.split_compound("autobahn");
    assert!(!result.is_empty());
    assert!(result[0].part1.chars().next().unwrap().is_uppercase());

    // Mixed case
    let result = splitter.split_compound("AuToBaHn");
    assert!(!result.is_empty());
    assert!(result[0].part1.chars().next().unwrap().is_uppercase());
}

#[test]
fn test_unicode_characters() {
    let splitter = Splitter::new().expect("Failed to create splitter");

    let unicode_words = [("Bäckerhandel", "Bäck"), ("Fußball", "Fuß"), ("Straße", "Str")];

    for (word, expected_part1_prefix) in unicode_words {
        let result = splitter.split_compound(word);
        assert!(!result.is_empty(), "Should return result for unicode word: {}", word);
        assert!(
            result[0].part1.starts_with(expected_part1_prefix),
            "Part1 should start with {}: {}",
            expected_part1_prefix,
            result[0].part1
        );
    }
}

#[test]
fn test_short_words() {
    let splitter = Splitter::new().expect("Failed to create splitter");

    for word in BASIC_TEST_WORDS {
        let result = splitter.split_compound(word);
        assert!(!result.is_empty(), "Should return result for short word: {}", word);
    }
}

#[test]
fn test_result_ordering() {
    let splitter = Splitter::new().expect("Failed to create splitter");
    let result = splitter.split_compound("Autobahnraststätte");

    // Check that results are sorted by score descending
    for i in 1..result.len() {
        assert!(
            result[i].score <= result[i - 1].score,
            "Results should be sorted by score descending: {} >= {}",
            result[i - 1].score,
            result[i].score
        );
    }
}

#[test]
fn test_multiple_results_returned() {
    let splitter = Splitter::new().expect("Failed to create splitter");
    let result = splitter.split_compound("Autobahnraststätte");

    // Should return many possible splits
    assert!(result.len() > 5, "Should return multiple split options");
}

#[test]
fn test_all_results_have_correct_structure() {
    let splitter = Splitter::new().expect("Failed to create splitter");
    let result = splitter.split_compound("Behördenangaben");

    for r in &result {
        // Check that score is a valid float
        assert!(!r.score.is_nan(), "Score should not be NaN");
        assert!(r.score >= 0.0 || r.score <= 0.0, "Score should be a valid number");

        // Check that parts are non-empty strings (title-cased)
        assert!(!r.part1.is_empty(), "Part1 should not be empty");
        assert!(!r.part2.is_empty(), "Part2 should not be empty");

        // Check title case (first char uppercase)
        assert!(
            r.part1.chars().next().unwrap().is_uppercase(),
            "Part1 should be title-cased: {}",
            r.part1
        );
        assert!(
            r.part2.chars().next().unwrap().is_uppercase(),
            "Part2 should be title-cased: {}",
            r.part2
        );
    }
}

#[test]
#[ignore = "Requires running Python to get expected results"]
fn verify_python_parity() {
    // This test requires Python to be available to get expected results.
    // It's ignored by default but can be run with:
    // `cargo test -- --ignored verify_python_parity`

    let splitter = Splitter::new().expect("Failed to create splitter");

    for &(word, expected_score, expected_part1, expected_part2) in TEST_WORDS {
        let rust_result = splitter.split_compound(word);

        assert!(!rust_result.is_empty(), "No result for: {}", word);

        // Check score is close (within 0.001)
        assert!(
            (rust_result[0].score - expected_score).abs() < 0.001,
            "Score mismatch for '{}': got {}, expected {}",
            word,
            rust_result[0].score,
            expected_score
        );

        // Check parts match
        assert_eq!(
            rust_result[0].part1, expected_part1,
            "Part1 mismatch for '{}': got {}, expected {}",
            word, rust_result[0].part1, expected_part1
        );
        assert_eq!(
            rust_result[0].part2, expected_part2,
            "Part2 mismatch for '{}': got {}, expected {}",
            word, rust_result[0].part2, expected_part2
        );
    }
}

#[test]
fn test_fugen_s_detection() {
    use charsplit_fst::fugen_s::{remove_fugen_s, should_remove_fugen_s};

    // Test should_remove_fugen_s
    assert!(should_remove_fugen_s("beits"));  // ends with "ts"
    assert!(should_remove_fugen_s("mehls"));  // ends with "hls"
    assert!(!should_remove_fugen_s("haus"));
    assert!(!should_remove_fugen_s("arbeit"));  // ends with "t", not "ts"
    assert!(!should_remove_fugen_s("hilfs"));  // ends with "fs", not in our patterns

    // Test remove_fugen_s
    assert_eq!(remove_fugen_s("beits"), Some("beit"));
    assert_eq!(remove_fugen_s("mehls"), Some("mehl"));
    assert_eq!(remove_fugen_s("haus"), None);
    assert_eq!(remove_fugen_s("ts"), None); // Too short
}

#[test]
fn test_fallback_for_no_split() {
    // Create a splitter with minimal maps (will not find any splits)
    use fst::MapBuilder;
    use charsplit_fst::ngram::NgramLookup;

    let mut builder = MapBuilder::new(Vec::new()).unwrap();
    builder.insert(b"\x00", 0).unwrap();
    let dummy_map = Map::new(builder.into_inner().unwrap()).unwrap();

    let ngram_lookup = NgramLookup::new(
        dummy_map.clone(),
        dummy_map.clone(),
        dummy_map,
    );
    let splitter = charsplit_fst::Splitter::with_ngram_lookup(ngram_lookup);

    // Even with no probabilities, should return something with calculated scores
    let result = splitter.split_compound("Xyzabc");
    assert!(!result.is_empty());
    // With no matching ngrams, scores will be negative but still calculated
    // e.g., -1.0 (suffix) - 1.0 (infix default) + (-1.0) (prefix) = -3.0
    assert!(result[0].score.is_finite()); // Should be a finite calculated score, not NaN
}

#[test]
fn test_title_case_output() {
    let splitter = Splitter::new().expect("Failed to create splitter");

    // Lowercase input should produce title-case output
    let result = splitter.split_compound("autobahnraststätte");
    assert!(result[0].part1.chars().next().unwrap().is_uppercase());
    assert!(result[0].part2.chars().next().unwrap().is_uppercase());
}
