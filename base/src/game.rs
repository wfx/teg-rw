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
    pub board: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cards: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dices: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pieces: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: String,
}

impl crate::validator::Validatable for GameDefinition {
    fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("GameDefinition: 'id' must not be empty.".to_string());
        }
        if self.name.trim().is_empty() {
            return Err("GameDefinition: 'name' must not be empty.".to_string());
        }
        // You can add further checks here as needed, e.g. for allowed field values
        Ok(())
    }
}
