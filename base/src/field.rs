use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
#[allow(unused_imports)]
use std::{convert, fmt, option, write};

/// Unique identifier for a field (single tile on the board).
pub type FieldId = u32;

/// Unique identifier for a field set (group of related fields).
pub type FieldSetId = u32;

/// A single field element on the game board.
/// Each field belongs to exactly one FieldSet and has a visual position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldElement {
    /// Unique identifier of the field.
    pub id: FieldId,

    /// Display name of the field (e.g., "Kamchatka").
    pub name: String,

    /// ID of the FieldSet this field belongs to.
    pub set_id: FieldSetId,

    /// Logical (x, y) position used for display purposes.
    pub position: (f32, f32),
}

/// A logical group of fields (e.g., a continent or region).
/// Used to assign bonus points and visual grouping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSet {
    /// Unique identifier of the set.
    pub id: FieldSetId,

    /// Display name of the set (e.g., "Europe").
    pub name: String,

    /// Color assigned to this set (RGB).
    pub color: (u8, u8, u8),

    /// Bonus points awarded when all fields in the set are controlled.
    pub bonus: u32,
}

/// A relation between two fields indicating adjacency.
/// Used to define borders or valid movement paths.
/// This is a *directed* relation (A → B is not the same as B → A).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FieldRelation(pub FieldId, pub FieldId);

/// The complete definition of a game board, including all fields,
/// their grouping into sets, and relations between them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldStructure {
    /// All defined field elements, indexed by their unique ID.
    pub elements: HashMap<FieldId, FieldElement>,

    /// All logical field sets (e.g., continents or zones).
    pub sets: HashMap<FieldSetId, FieldSet>,

    /// Directed relations between fields (e.g., borders or valid paths).
    pub relations: HashSet<FieldRelation>,
}

/// Error type for loading a field structure.
#[derive(Debug, thiserror::Error)]
pub enum FieldLoadError {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse RON: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

/// Error during board structure validation.
#[derive(Debug, thiserror::Error)]
pub enum FieldValidationError {
    #[error("Missing FieldSet with id: {0}")]
    MissingSet(FieldSetId),

    #[error("Unknown FieldId in relation: ({0}, {1})")]
    InvalidRelation(FieldId, FieldId),
}

impl FieldStructure {
    /// Loads a FieldStructure from a RON file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, FieldLoadError> {
        let content = fs::read_to_string(path)?;
        let structure: FieldStructure = ron::from_str(&content)?;
        Ok(structure)
    }

    /// Checks for structural consistency:
    /// - all set_ids exist
    /// - all relations refer to valid field ids
    pub fn validate(&self) -> Result<(), FieldValidationError> {
        for element in self.elements.values() {
            if !self.sets.contains_key(&element.set_id) {
                return Err(FieldValidationError::MissingSet(element.set_id));
            }
        }

        for FieldRelation(a, b) in &self.relations {
            if !self.elements.contains_key(a) || !self.elements.contains_key(b) {
                return Err(FieldValidationError::InvalidRelation(*a, *b));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};

    fn get_boards_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../data/boards")
    }

    #[test]
    fn test_all_boards_in_data_directory() {
        let board_dir = get_boards_path();
        assert!(board_dir.exists(), "Boards directory does not exist");

        let entries = fs::read_dir(&board_dir).expect("Cannot read boards directory");

        let mut count = 0;

        for entry in entries {
            let path = entry.expect("Invalid entry").path();

            if path.extension().map_or(false, |ext| ext == "ron") {
                count += 1;
                println!("Testing board file: {}", path.display());

                let structure = FieldStructure::from_file(&path).expect("Failed to load RON file");

                structure.validate().expect("Board structure is invalid");
            }
        }

        assert!(count > 0, "No .ron board files found in {:?}", board_dir);
    }
}
