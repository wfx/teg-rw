//! Generic loader for RON-based data structures.
//!
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;

/// Loads and deserializes a RON file into the given type.
/// Returns a Result<T, String>.
pub fn load_ron<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    let file = File::open(path).map_err(|e| format!("Could not open file '{}': {}", path, e))?;
    let reader = BufReader::new(file);
    ron::de::from_reader(reader).map_err(|e| format!("Failed to parse '{}': {}", path, e))
}

/// Loads, deserializes and validates a RON file.
/// T must implement Validatable.
pub fn load_and_validate_ron<T>(path: &str) -> Result<T, String>
where
    T: DeserializeOwned + crate::validator::Validatable,
{
    let value = load_ron::<T>(path)?;
    value.validate()?;
    Ok(value)
}
