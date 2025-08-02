//! Central error types for data loading and validation.
//!
//! This module defines `DataError`, the primary error type used throughout
//! the loader and validator modules to represent I/O, parsing, validation,
//! and asset-related errors.

use std::io;
use std::path::PathBuf;

use ron::de::Error as RonError;
use thiserror::Error;

/// Represents all errors that can occur while loading, parsing, and validating
/// RON-based data assets or checking external assets like images.

pub type Result<T> = std::result::Result<T, DataError>;

#[derive(Debug, Error)]
pub enum DataError {
    /// An I/O error occurred when trying to read a file.
    #[error("I/O error while reading {path:?}: {source}")]
    Io {
        /// Path of the file that failed to be read.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: io::Error,
    },

    /// Failed to parse RON data from a file.
    #[error("RON parse error in {path:?}: {source}")]
    ParseRon {
        /// Path of the file that contained invalid RON.
        path: PathBuf,
        /// Underlying RON parsing error.
        #[source]
        source: RonError,
    },

    /// A required external asset (e.g., image or other file) was not found.
    #[error("Asset not found: {0}")]
    AssetNotFound(PathBuf),

    /// A validation rule failed with the provided message.
    #[error("Validation error: {0}")]
    Validation(String),
}
