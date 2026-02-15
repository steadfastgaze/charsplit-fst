// SPDX-License-Identifier: MIT OR Apache-2.0

//! Command-line interface for German compound word splitter
//!
//! Supports two modes:
//! 1. Single word argument: charsplit-fst Autobahnraststätte
//! 2. Stdin mode: echo "word" | charsplit-fst (one word per line)

use charsplit_fst::Splitter;
use std::env;
use std::io::{self, BufRead};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    // Single word argument mode
    if args.len() == 2 {
        let word = &args[1];

        match run_single_word(word) {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("Error: {}", e);
                ExitCode::FAILURE
            }
        }
    } else {
        // Stdin mode (one word per line)
        match run_stdin() {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("Error: {}", e);
                ExitCode::FAILURE
            }
        }
    }
}

/// Run in single word mode
fn run_single_word(word: &str) -> Result<(), Box<dyn std::error::Error>> {
    let splitter = Splitter::new()?;
    let results = splitter.split_compound(word);

    for result in results.iter().take(10) {
        println!("{:.3}\t{}\t{}", result.score, result.part1, result.part2);
    }

    Ok(())
}

/// Run in stdin mode
fn run_stdin() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let splitter = Splitter::new()?;

    for line in stdin.lock().lines() {
        let word = line?;
        let word = word.trim();

        if word.is_empty() {
            continue;
        }

        let results = splitter.split_compound(word);
        let best = &results[0];

        println!("{}\t{}\t{}", best.part1, best.part2, word);
    }

    Ok(())
}
