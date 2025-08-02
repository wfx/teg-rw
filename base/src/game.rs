//! GameDefinition – references all components required to play a game.
//! This includes board, rules, pieces, cards, encounters, and game parameters.

use crate::DataError;
use crate::{verify_file, Validatable};
use serde::Deserialize;
use std::path::Path;

/// Top-level game definition.
#[derive(Debug, Clone, Deserialize)]
pub struct GameDefinition {
    /// Unique game identifier.
    pub id: String,
    /// Human-readable name of the game.
    pub name: String,
    /// Filename of the rule definition (in wad/game folder).
    pub rule: String,
    /// Filename of the board definition.
    pub board: String,
    /// Filename of the pieces definition.
    pub pieces: String,
    /// Optional filename of the cards definition.
    pub cards: Option<String>,
    /// Optional filename of the encounter definition.
    pub encounter: Option<String>,
    /// Game-specific parameters.
    pub parameters: GameParameters,
}

/// Game-specific parameters (players, bonuses, etc.)
#[derive(Debug, Clone, Deserialize)]
pub struct GameParameters {
    /// Minimum number of players.
    pub min_players: u8,
    /// Maximum number of players.
    pub max_players: u8,
    /// Optional field-set bonuses.
    #[serde(default)]
    pub fieldsets: Option<Vec<FieldSetBonus>>,
    /// Placement configuration.
    pub placement: PlacementConfig,
    /// Sequence of bonus figures for trading cards.
    pub card_bonus_sequence: Vec<u32>,
}

/// Defines bonus for controlling a field set (zone)
#[derive(Debug, Clone, Deserialize)]
pub struct FieldSetBonus(pub u8, pub u8); // (FieldSetId, BonusFigures)

/// Defines placement rules for initial game setup
#[derive(Debug, Clone, Deserialize)]
pub struct PlacementConfig {
    pub setup_round_figures: u32,
    pub regular_round_figures: u32,
    pub fieldsets_bonus: bool,
    pub control_bonus: ControlBonusMode,
}

/// When field set bonuses apply
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ControlBonusMode {
    Off,
    Once,
    Turn,
}

impl Validatable for GameDefinition {
    fn validate(&self) -> Result<(), DataError> {
        let base_dir = Path::new("wad/game");

        // 1. id must not be empty
        if self.id.trim().is_empty() {
            return Err(DataError::Validation("id must not be empty".into()));
        }

        // 2. name must not be empty
        if self.name.trim().is_empty() {
            return Err(DataError::Validation("name must not be empty".into()));
        }

        // 3. player count range: min_players > 0 and max_players >= min_players
        let min = self.parameters.min_players;
        let max = self.parameters.max_players;
        if min == 0 || max < min {
            return Err(DataError::Validation("invalid player count range".into()));
        }

        // 4. card_bonus_sequence must contain at least one element
        if self.parameters.card_bonus_sequence.is_empty() {
            return Err(DataError::Validation(
                "card_bonus_sequence must not be empty".into(),
            ));
        }

        // 5. optional fieldsets, if provided, must not be empty
        if let Some(fs) = &self.parameters.fieldsets {
            if fs.is_empty() {
                return Err(DataError::Validation(
                    "fieldsets, if provided, must not be empty".into(),
                ));
            }
        }

        // 6. check existence of required assets (with or without .ron extension)
        verify_file(&self.rule, "Rule", base_dir)?;
        verify_file(&self.board, "Board", base_dir)?;
        verify_file(&self.pieces, "Pieces", base_dir)?;

        // 7. check optional assets: cards and encounter
        if let Some(cards_file) = &self.cards {
            verify_file(cards_file, "Cards", base_dir)?;
        }
        if let Some(enc_file) = &self.encounter {
            verify_file(enc_file, "Encounter", base_dir)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_game {
    use super::*;
    use crate::loader::load_ron;
    use std::fs;
    use std::path::Path;

    #[test]
    fn validate_all_game_files() {
        let dir = "wad/game";
        assert!(
            Path::new(dir).exists(),
            "wad/game directory does not exist: {}",
            dir
        );

        let mut failed = Vec::new();

        for entry in fs::read_dir(dir).expect("Failed to read wad/game directory") {
            let entry = entry.expect("Invalid directory entry");
            let path = entry.path();

            // Filter: Endung .ron und Dateiname endet auf .game.ron
            if path.extension().map_or(false, |ext| ext == "ron")
                && path
                    .file_name()
                    .map_or(false, |name| name.to_string_lossy().ends_with(".game.ron"))
            {
                match load_ron::<GameDefinition, _>(&path) {
                    Ok(_) => { /* alles gut */ }
                    Err(e) => failed.push((path.display().to_string(), e.to_string())),
                }
            }
        }

        if !failed.is_empty() {
            for (file, err) in &failed {
                eprintln!("❌ Validation failed for {}:\n{}", file, err);
            }
            panic!("{} game file(s) failed validation", failed.len());
        }
    }
}
