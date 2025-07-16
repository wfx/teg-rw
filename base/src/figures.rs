//! Defines the structure and loading logic for figure sets (piece sets)
//! Each figure set consists of named boxes (typically per player),
//! and each box defines several figures of differing value.

use ron::de::SpannedError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use thiserror::Error;

/// One figure (piece) with a value and an image path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Figure {
    /// Logical unit value of this figure (e.g. 1, 5, 10).
    pub value: u32,
    /// Path to the image representing this figure.
    pub image: String,
}

/// Public representation of a figure set: a map of box names to figures.
#[derive(Debug, Clone)]
pub struct FigureSet {
    /// Name of this figure set (e.g. "Classic", "Modern").
    pub name: String,
    /// Mapping from box names to sequences of figures.
    pub boxes: HashMap<String, Vec<Figure>>,
}

/// Intermediate struct for deserializing the RON format.
#[derive(Debug, Deserialize)]
struct FigureBoxDef {
    /// Name of this box (e.g. player color).
    name: String,
    /// Figures in this box.
    pieces: Vec<Figure>,
}

/// Intermediate struct matching the RON file's top-level structure.
#[derive(Debug, Deserialize)]
struct FigureSetDef {
    /// Name of the figure set.
    name: String,
    /// A list of boxes with names and their pieces.
    boxes: Vec<FigureBoxDef>,
}

/// Errors that can occur while loading a figure set.
#[derive(Debug, Error)]
pub enum FigureLoadError {
    /// I/O error when reading the RON file.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// Error parsing the RON content.
    #[error("Parse error: {0}")]
    Parse(#[from] SpannedError),
}

impl FigureSet {
    /// Load a FigureSet from a RON file at the given path.
    /// Supports the RON format where `boxes` is a list of `(name, pieces)` tuples.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, FigureLoadError> {
        let content = fs::read_to_string(path.as_ref())?;
        let def: FigureSetDef = ron::from_str(&content)?;
        // Transform into a map of box name -> figures
        let boxes_map = def
            .boxes
            .into_iter()
            .map(|b| (b.name, b.pieces))
            .collect::<HashMap<_, _>>();
        Ok(FigureSet {
            name: def.name,
            boxes: boxes_map,
        })
    }

    /// Validate the integrity of the figure set.
    /// Ensures each box is non-empty and has unique values within itself.
    pub fn validate(&self) -> Result<(), FigureValidationError> {
        if self.boxes.is_empty() {
            return Err(FigureValidationError::EmptySet(self.name.clone()));
        }
        for (box_name, figs) in &self.boxes {
            if figs.is_empty() {
                return Err(FigureValidationError::EmptyBox(box_name.clone()));
            }
            let mut seen = HashSet::new();
            for fig in figs {
                if !seen.insert(fig.value) {
                    return Err(FigureValidationError::DuplicateValue(fig.value));
                }
            }
        }
        Ok(())
    }
}

/// Domain-specific validation errors for figure sets.
#[derive(Debug, Error)]
pub enum FigureValidationError {
    /// The entire set has no boxes defined.
    #[error("Figure set '{0}' contains no boxes.")]
    EmptySet(String),
    /// A specific box is empty.
    #[error("Box '{0}' contains no figures.")]
    EmptyBox(String),
    /// A duplicate figure value was found within a box.
    #[error("Duplicate figure value found: {0}.")]
    DuplicateValue(u32),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// Helper to locate the `data/figures` directory from the crate root.
    fn get_figures_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../data/figures")
    }

    #[test]
    fn test_all_figures_in_data_directory() {
        let fig_dir = get_figures_path();
        assert!(
            fig_dir.exists(),
            "Figures directory does not exist: {:?}",
            fig_dir
        );
        let mut count = 0;
        for entry in fs::read_dir(&fig_dir).expect("Failed to read figures directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("ron") {
                count += 1;
                println!("Testing figure file: {}", path.display());

                // load via the public API
                let figure_set = FigureSet::from_file(&path).expect("Failed to load RON file");

                // ensure the set passes validation
                figure_set
                    .validate()
                    .expect(&format!("Validation failed for file: {}", path.display()));
            }
        }
        assert!(count > 0, "No .ron figure files found in {:?}", fig_dir);
    }
}
