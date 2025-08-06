//! Generic loader for RON-based data structures and asset-check helper.
//!
//! This module provides a helper to ensure external asset files exist
//! and a generic `load_ron` function that reads, parses, and validates
//! RON data assets.

use crate::validator::{verify_file, Validatable};
use crate::DataError;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::path::{Path, PathBuf};

/// Loads a RON file from the given path, deserializes it into `T`, and validates it.
///
/// # Type Parameters
///
/// * `T` - The target type implementing `DeserializeOwned` and `Validatable`.
/// * `P` - A path-like type to the RON file.
///
/// # Errors
///
/// - `DataError::AssetNotFound` if the file is missing.
/// - `DataError::Io` if opening or reading the file fails.
/// - `DataError::ParseRon` if RON deserialization fails.
/// - Any `DataError::Validation` from `T::validate()` if the data is invalid.
use std::ffi::OsStr;

pub fn load_ron<T, P>(path: P) -> Result<T, DataError>
where
    T: DeserializeOwned + Validatable,
    P: AsRef<Path>,
{
    let path_buf: PathBuf = path.as_ref().to_path_buf();

    let base = path_buf.parent().ok_or_else(|| {
        DataError::Validation(format!(
            "Invalid path (no parent dir): {}",
            path_buf.display()
        ))
    })?;

    let name = path_buf
        .file_stem()
        .and_then(OsStr::to_str)
        .ok_or_else(|| {
            DataError::Validation(format!("Invalid file name: {}", path_buf.display()))
        })?;

    verify_file(name, "RON data file", base)?;

    let file = File::open(&path_buf).map_err(|source| DataError::Io {
        path: path_buf.clone(),
        source,
    })?;

    let data: T = ron::de::from_reader(file).map_err(|source| DataError::ParseRon {
        path: path_buf.clone(),
        source: source.into(),
    })?;

    data.validate()?;

    Ok(data)
}
