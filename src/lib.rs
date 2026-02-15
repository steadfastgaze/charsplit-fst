// SPDX-License-Identifier: MIT OR Apache-2.0

//! German compound word splitter using FST (Finite State Transducer) data structures.
//!
//! This library provides efficient splitting of German compound words into their
//! component parts, using pre-computed ngram probabilities stored in FST format.
//!
//! # Example
//!
//! ```no_run
//! use charsplit_fst::Splitter;
//!
//! let splitter = Splitter::new().unwrap();
//! let results = splitter.split_compound("Autobahnraststätte");
//!
//! for result in results.iter().take(3) {
//!     println!("{:.3} | {} | {}", result.score, result.part1, result.part2);
//! }
//! ```
//!
//! # Output Format
//!
//! Each split result contains:
//! - `score`: Probability score (higher is better)
//! - `part1`: First part of the compound (title-cased)
//! - `part2`: Second part of the compound (title-cased)

pub mod error;
pub mod fugen_s;
pub mod ngram;
pub mod score;
pub mod splitter;

// Only include Python bindings when building as a Python extension
#[cfg(feature = "python")]
pub mod python_bindings;

// Only include web bindings when building for web/WASM
#[cfg(feature = "web")]
pub mod web;

pub use error::{Error, Result};
pub use splitter::{SplitResult, Splitter};

// Python module definition (only when building as extension)
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn charsplit_fst(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<python_bindings::PySplitter>()?;
    Ok(())
}
