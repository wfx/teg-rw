use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, path::Path};
use thiserror::Error;

/// Errors that can occur loading or validating piece data.
#[derive(Debug, Error)]
pub enum DataError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("RON parse error: {0}")]
    Ron(#[from] ron::Error),
    #[error("Validation error: {0}")]
    Validation(String),
}

/// One game piece with a value and optional image path.
/// The name of the piece is derived from the containing PieceSet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Piece {
    pub value: u32,
    pub image: Option<String>,
}

/// A named collection of pieces.
/// In RON, entries are named `set`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PieceSet {
    pub name: String,
    #[serde(rename = "piece")]
    pub pieces: Vec<Piece>,
}

/// Top-level structure for all piece sets.
/// In RON, use `sets` key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PieceStructure {
    #[serde(rename = "sets")]
    pub sets: Vec<PieceSet>,
}

impl PieceStructure {
    /// Load piece data from a .ron file, parse and validate.
    pub fn load(path: &Path) -> Result<Self, DataError> {
        let raw = fs::read_to_string(path)?;
        let structure: PieceStructure = ron::from_str(&raw)?;
        structure.validate()?;
        Ok(structure)
    }

    /// Ensure set names are unique and each set has at least one piece.
    pub fn validate(&self) -> Result<(), DataError> {
        let mut seen_sets = HashSet::new();
        for set in &self.sets {
            if !seen_sets.insert(&set.name) {
                return Err(DataError::Validation(format!(
                    "Duplicate piece set name: {}", set.name
                )));
            }
            if set.pieces.is_empty() {
                return Err(DataError::Validation(format!(
                    "Piece set '{}' must contain at least one piece", set.name
                )));
            }
        }
        Ok(())
    }
}
