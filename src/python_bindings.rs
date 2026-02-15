// SPDX-License-Identifier: MIT OR Apache-2.0

//! Python bindings for German compound word splitter.
//!
//! Provides a PyO3 wrapper around the Rust Splitter implementation,
//! maintaining API compatibility with the original Python version.

use pyo3::prelude::*;
use pyo3::types::PyList;
use crate::splitter::Splitter;

/// German compound word splitter
///
/// Drop-in replacement for the original Python split_words.Splitter class.
///
/// Example:
///     >>> from charsplit_fst import Splitter
///     >>> splitter = Splitter()
///     >>> results = splitter.split_compound("Autobahnraststätte")
///     >>> print(results[0])
///     (0.795, 'Autobahn', 'Raststätte')
#[pyclass(name = "Splitter")]
pub struct PySplitter {
    splitter: Splitter,
}

#[pymethods]
impl PySplitter {
    /// Create a new splitter instance
    ///
    /// Data files are loaded from the default "data" directory.
    ///
    /// Returns:
    ///     A new Splitter instance
    ///
    /// Raises:
    ///     IOError: If FST files cannot be loaded
    #[new]
    #[pyo3(signature = ())]
    pub fn new() -> PyResult<Self> {
        let splitter = Splitter::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        Ok(Self { splitter })
    }

    /// Split a German compound word
    ///
    /// Args:
    ///     word: The compound word to split
    ///
    /// Returns:
    ///     List of (score, part1, part2) tuples sorted by score descending.
    ///     Each tuple contains:
    ///         - score: float probability score (higher is better)
    ///         - part1: str, first part of the compound (title-cased)
    ///         - part2: str, second part of the compound (title-cased)
    ///
    /// Examples:
    ///     >>> splitter = Splitter()
    ///     >>> results = splitter.split_compound("Autobahnraststätte")
    ///     >>> results[0]
    ///     (0.795, 'Autobahn', 'Raststätte')
    #[pyo3(signature = (word,))]
    fn split_compound(&self, word: &str) -> PyResult<Py<PyAny>> {
        let results = self.splitter.split_compound(word);

        // Convert to Python list of tuples
        Python::with_gil(|py| {
            let list = PyList::empty_bound(py);
            for result in results {
                let tuple = (result.score, result.part1.as_str(), result.part2.as_str());
                list.append(tuple)?;
            }
            Ok(list.into_py(py))
        })
    }
}
