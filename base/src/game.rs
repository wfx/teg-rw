use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GameDefinition {
    /// Unique identifier for this game
    pub id: String,

    /// Display name of the game
    pub name: String,

    /// Author or creator of the game
    pub author: String,

    /// Game version (semantic or otherwise)
    pub version: String,

    /// Short description of the game
    pub description: String,

    // Optional component references.
    // Each of these maps to a RON file with a matching suffix,
    // e.g. "" => loads "<id>.board.ron", "custom" => "custom.board.ron"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub board: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cards: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dices: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pieces: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
}
