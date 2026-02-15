// SPDX-License-Identifier: MIT OR Apache-2.0

//! Error types for the German splitter.

use std::io;
use thiserror::Error;

/// Errors that can occur during splitting or FST loading.
#[derive(Error, Debug)]
pub enum Error {
    /// IO error when reading FST files.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// FST error when loading or querying maps.
    #[error("FST error: {0}")]
    Fst(String),

    /// Invalid data format.
    #[error("Invalid data: {0}")]
    InvalidData(String),

    /// Word is too short to split.
    #[error("Word too short: {0}")]
    WordTooShort(String),
}

/// Result type alias for the German splitter.
pub type Result<T> = std::result::Result<T, Error>;
