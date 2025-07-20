use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, path::Path};
use thiserror::Error;

/// Errors that can occur loading or validating game metadata.
#[derive(Debug, Error)]
pub enum DataError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("RON parse error: {0}")]
    Ron(#[from] ron::Error),
    #[error("Validation error: {0}")]
    Validation(String),
}

/// Defines a single player's configuration (name, hidden goal id, initial set)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerConfig {
    pub id: u32,
    pub name: String,
    pub goal_id: Option<u32>,
    pub starting_set: String,
}

/// Defines a hidden goal or victory condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: u32,
    pub description: String,
}

/// Top-level game metadata structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMeta {
    pub players: Vec<PlayerConfig>,
    pub goals: Vec<Goal>,
    pub max_turns: Option<u32>,
}

impl GameMeta {
    /// Load game metadata from a .ron file, parse and validate.
    pub fn load(path: &Path) -> Result<Self, DataError> {
        let raw = fs::read_to_string(path)?;
        let meta: GameMeta = ron::from_str(&raw)?;
        meta.validate()?;
        Ok(meta)
    }

    /// Ensure unique player IDs and goal references.
    pub fn validate(&self) -> Result<(), DataError> {
        let mut seen_ids = HashSet::new();
        for player in &self.players {
            if !seen_ids.insert(player.id) {
                return Err(DataError::Validation(format!(
                    "Duplicate player id: {}", player.id
                )));
            }
        }
        // Check goal uniqueness
        let mut seen_goals = HashSet::new();
        for goal in &self.goals {
            if !seen_goals.insert(goal.id) {
                return Err(DataError::Validation(format!(
                    "Duplicate goal id: {}", goal.id
                )));
            }
        }
        // Check that any referenced goal_id exists
        for player in &self.players {
            if let Some(gid) = player.goal_id {
                if !seen_goals.contains(&gid) {
                    return Err(DataError::Validation(format!(
                        "Player {} references unknown goal id: {}", player.id, gid
                    )));
                }
            }
        }
        Ok(())
    }
}

