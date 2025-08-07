use serde::{Deserialize, Serialize};

/// Top-level structure for board definitions.
/// Each board contains sets (continents), fields (countries), and relations (borders).
#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    /// Unique identifier for this board definition.
    pub id: String,

    /// Human-readable name of the board.
    pub name: String,

    /// Author of the board.
    pub author: String,

    /// Version string (semantic or otherwise).
    pub version: String,

    /// Short description of the board's theme or origin.
    pub description: String,

    /// List of territory sets (e.g. continents).
    pub sets: Vec<FieldSet>,

    /// List of individual fields (e.g. countries).
    pub fields: Vec<FieldElement>,

    /// Connections between fields (bidirectional borders).
    pub relations: Vec<(u8, u8)>,
}

/// Represents a group of fields that share a common theme or bonus (e.g. a continent).
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct FieldSet {
    /// Numeric ID (must be unique).
    pub id: u8,

    /// Name of the set (e.g. "Asia", "Europe").
    pub name: String,
}

/// Represents a single field (territory or country) on the board.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct FieldElement {
    /// Unique numeric ID.
    pub id: u8,

    /// Name of the field (e.g. "Argentina").
    pub name: String,

    /// ID of the set (continent) this field belongs to.
    pub set_id: u8,

    /// Position of the field on the board image (x, y).
    pub position: (i16, i16),

    /// Position where the piece should be rendered (relative or absolute).
    pub piece_pos: (i16, i16),

    /// Optional filename for field artwork (can be empty).
    pub filename: String,
}

impl crate::validator::Validatable for Board {
    fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("Board: 'id' must not be empty.".into());
        }
        if self.fields.is_empty() {
            return Err("Board: 'fields' must not be empty.".into());
        }
        // Check for duplicate IDs
        let mut ids = std::collections::HashSet::new();
        for field in &self.fields {
            if !ids.insert(field.id) {
                return Err(format!("Board: duplicate field id {}", field.id));
            }
        }
        // Check that relations reference valid field ids
        for &(a, b) in &self.relations {
            if !ids.contains(&a) || !ids.contains(&b) {
                return Err(format!(
                    "Board: relation ({},{}) refers to unknown field id",
                    a, b
                ));
            }
        }
        Ok(())
    }
}
