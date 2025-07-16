use crate::field::{FieldId, FieldStructure};
use ron::ser::to_string_pretty;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    Setup,
    Place,
    Interact,
    Move,
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: u32,
    pub name: String,
    pub active: bool,
    pub available_units: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldStatus {
    pub owner_id: Option<u32>,
    pub units: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub participants: Vec<Participant>,
    pub current_participant: usize,
    pub phase: GamePhase,
    pub fields: HashMap<FieldId, FieldStatus>,
}

impl GameState {
    /// Create a new empty game state from a field structure and list of participants.
    pub fn new(field_structure: &FieldStructure, participants: Vec<Participant>) -> Self {
        let fields = field_structure
            .elements
            .keys()
            .map(|&id| {
                (
                    id,
                    FieldStatus {
                        owner_id: None,
                        units: 0,
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        GameState {
            participants,
            current_participant: 0,
            phase: GamePhase::Setup,
            fields,
        }
    }

    /// Save the current game state to a file in RON format.
    pub fn save_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        let ron = to_string_pretty(self, ron::ser::PrettyConfig::default())
            .expect("Failed to serialize game state");
        let mut file = File::create(path)?;
        file.write_all(ron.as_bytes())?;
        Ok(())
    }

    /// Load a game state from a RON file.
    pub fn load_from_file(path: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let game_state: GameState = ron::from_str(&content)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err.to_string()))?;
        Ok(game_state)
    }
}
