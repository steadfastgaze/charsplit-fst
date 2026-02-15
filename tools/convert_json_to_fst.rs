// SPDX-License-Identifier: MIT OR Apache-2.0

//! Convert ngram_probs.json to FST format.
//!
//! This tool reads the JSON ngram probability file and converts it to
//! three separate FST files for efficient loading and querying.
//!
//! # FST Format Benefits
//!
//! - **Compressed**: FSTs use much less space than JSON
//! - **Fast**: O(k) lookup where k is key length (not number of entries)
//! - **Memory-mappable**: Can be loaded directly from disk without full parsing
//!
//! # Critical Requirement
//!
//! FSTs require keys to be inserted in **sorted lexicographic order**.
//! This tool sorts all keys before building, which is essential for correctness.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Converting ngram_probs.json to FST format...");

    // Read the JSON file
    let json_path = "../split_words/ngram_probs.json";
    println!("Reading {}...", json_path);

    let json_data = fs::read_to_string(json_path)?;
    let data: HashMap<String, HashMap<String, f64>> = serde_json::from_str(&json_data)?;

    // Create output directory
    fs::create_dir_all("data")?;

    // Convert each map to FST
    println!("Converting suffix map...");
    build_fst(&data["suffix"], "data/suffix.fst")?;

    println!("Converting prefix map...");
    build_fst(&data["prefix"], "data/prefix.fst")?;

    println!("Converting infix map...");
    build_fst(&data["infix"], "data/infix.fst")?;

    println!("\nConversion complete!");
    println!("Generated files:");
    println!("  - data/suffix.fst");
    println!("  - data/prefix.fst");
    println!("  - data/infix.fst");

    Ok(())
}

/// Build an FST from a HashMap of ngram probabilities.
///
/// # Process
///
/// 1. Convert all f64 probabilities to u64 (lossless bitwise conversion)
/// 2. Sort keys lexicographically (CRITICAL: FSTs require sorted order)
/// 3. Build FST by inserting keys in sorted order
/// 4. Write to disk and report statistics
///
/// # Arguments
///
/// * `data` - HashMap mapping ngram strings to their probabilities
/// * `output` - Path where the FST file will be written
///
/// # Errors
///
/// Returns an error if:
/// - Output file cannot be created
/// - FST building fails (e.g., keys not sorted)
fn build_fst(data: &HashMap<String, f64>, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    // CRITICAL: Sort keys lexicographically (FST requirement)
    // Building an FST with unsorted keys will panic!
    let mut pairs: Vec<(&String, u64)> = data
        .iter()
        .map(|(k, v)| (k, f64_to_u64(*v)))
        .collect();
    pairs.sort_by_key(|(k, _)| *k);

    // Create output file and FST builder
    let file = File::create(output)?;
    let writer = BufWriter::new(file);
    let mut builder = fst::MapBuilder::new(writer)?;

    // Insert all key-value pairs in sorted order
    for (key, value) in pairs {
        builder.insert(key.as_bytes(), value)?;
    }

    // Finalize the FST and write to disk
    builder.finish()?;

    // Report statistics
    let path = Path::new(output);
    let file_size = fs::metadata(path)?.len();
    println!(
        "  {}: {} entries, {} bytes",
        output,
        data.len(),
        file_size
    );

    Ok(())
}

/// Convert f64 to u64 losslessly using bitwise conversion.
///
/// This preserves the exact binary representation of the float,
/// allowing perfect round-trip conversion back to f64.
fn f64_to_u64(v: f64) -> u64 {
    v.to_bits()
}
