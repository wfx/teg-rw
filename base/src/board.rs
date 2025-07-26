use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, path::Path};
use thiserror::Error;

/// Unique identifier for a field (tile) on the board.
pub type FieldId = u8;

/// Unique identifier for a group of fields (e.g., a continent).
pub type FieldSetId = u8;

/// Represents a single field (tile) on the game board.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FieldElement {
    pub id: FieldId,
    pub name: String,
    pub set_id: FieldSetId,
    pub position: (u16, u16),
    pub piece_pos: (u16, u16),
    pub filename: String,
}

/// Represents a logical grouping of fields (e.g., continent).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FieldSet {
    pub id: FieldSetId,
    pub name: String,
    pub bonus: u8,
    pub color: (u8, u8, u8),
}

/// Represents an adjacency between two fields.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FieldRelation(pub FieldId, pub FieldId);

/// Top-level board structure with elements, sets, and relations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FieldStructure {
    pub id: String,
    pub name: String,
    pub folder: String,
    pub fields: HashSet<FieldElement>,
    pub sets: HashSet<FieldSet>,
    pub relations: Vec<FieldRelation>,
}

/// Errors that can occur loading or validating board data.
#[derive(Debug, Error)]
pub enum DataError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("RON parse error: {0}")]
    Ron(#[from] ron::error::SpannedError),
    #[error("Validation error: {0}")]
    Validation(String),
}

impl FieldStructure {
    /// Load board data from a .ron file, parse and validate.
    pub fn load(path: &Path) -> Result<Self, DataError> {
        let raw = fs::read_to_string(path)?;
        let structure: FieldStructure = ron::from_str(&raw)?;
        structure.validate()?;
        Ok(structure)
    }

    /// Ensure IDs are unique and references are consistent.
    pub fn validate(&self) -> Result<(), DataError> {
        // Check unique field IDs and that each field references a valid set
        let mut seen_fields = HashSet::new();
        for field in &self.fields {
            if !seen_fields.insert(field.id) {
                return Err(DataError::Validation(format!(
                    "Duplicate field id: {}",
                    field.id
                )));
            }
            if !self.sets.iter().any(|s| s.id == field.set_id) {
                return Err(DataError::Validation(format!(
                    "Field {} references unknown set {}",
                    field.id, field.set_id
                )));
            }
        }

        // Check unique set IDs
        let mut seen_sets = HashSet::new();
        for set in &self.sets {
            if !seen_sets.insert(set.id) {
                return Err(DataError::Validation(format!(
                    "Duplicate set id: {}",
                    set.id
                )));
            }
        }

        // Check relations refer to existing fields
        for &FieldRelation(from, to) in &self.relations {
            if !seen_fields.contains(&from) {
                return Err(DataError::Validation(format!(
                    "Relation from unknown field id: {}",
                    from
                )));
            }
            if !seen_fields.contains(&to) {
                return Err(DataError::Validation(format!(
                    "Relation to unknown field id: {}",
                    to
                )));
            }
        }

        Ok(())
    }
}
