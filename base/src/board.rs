use serde::{Deserialize, Serialize};

/// Top-level structure for board definitions.
/// Each board contains sets (continents), fields (countries), and relations (borders).
#[derive(Debug, Serialize, Deserialize)]
pub struct FieldStructure {
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
