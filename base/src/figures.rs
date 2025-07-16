//! Defines the structure and loading logic for figure sets (piece sets)
//! Each figure set consists of named boxes (typically per player),
//! and each box defines several figures of differing value.

use ron::de::from_str;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

/// One figure with a value and an image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Figure {
    pub value: u32,
    pub image: String,
}

/// A box of figures, typically assigned to a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigureBox {
    pub name: String,
    pub pieces: Vec<Figure>,
}

/// A full set of boxes used in a game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigureSet {
    pub name: String,
    pub boxes: Vec<FigureBox>,
}

impl FigureSet {
    /// Load a figure set from a RON file.
    pub fn load_from_file(path: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let parsed: FigureSet = from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_load_all_figure_sets() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../data/figures");
        assert!(dir.exists(), "Directory not found");

        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().map(|e| e == "ron").unwrap_or(false) {
                let content = fs::read_to_string(&path).expect("Failed to read file");
                let parsed: FigureSet = ron::from_str(&content).expect("Failed to parse");
                assert!(!parsed.boxes.is_empty());
                assert!(parsed.boxes.iter().all(|b| !b.pieces.is_empty()));
            }
        }
    }
}
