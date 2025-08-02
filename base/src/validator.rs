//! `Validatable` trait and asset-check helper.
//!
//! This module defines the `Validatable` trait for RON-based data structures
use crate::DataError;
use std::path::Path;

/// Verifies whether `name` or `name.ron` exists under `base`.
pub fn verify_file(name: &str, kind: &str, base: &Path) -> Result<(), DataError> {
    let plain = base.join(name);
    let with_ron = base.join(format!("{}.ron", name));
    if plain.is_file() || with_ron.is_file() {
        Ok(())
    } else {
        Err(DataError::Validation(format!(
            "{} file '{}' not found at either {:?} or {:?}",
            kind, name, plain, with_ron
        )))
    }
}

/// Trait for anything that can validate itself against a base directory.
pub trait Validatable {
    fn validate(&self) -> Result<(), DataError>;
}
