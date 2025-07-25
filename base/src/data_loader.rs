use crate::{board, game, piece, rule};
use std::path::Path;
use thiserror::Error;

/// Errors that can occur loading game data.
#[derive(Debug, Error)]
pub enum DataError {
    #[error("Failed to load board: {0}")]
    Board(#[from] board::DataError),
    #[error("Failed to load pieces: {0}")]
    Pieces(#[from] piece::DataError),
    #[error("Failed to load game metadata: {0}")]
    Game(#[from] game::DataError),
    #[error("Failed to load rules: {0}")]
    Rules(#[from] rule::DataError),
}

/// Gebündelte Spiel-Daten
pub struct GameData {
    pub board: board::FieldStructure,
    pub pieces: piece::PieceStructure,
    pub game: game::GameMeta,
    pub rules: rule::RuleSet,
}

/// Lade eine gesamte Variante (z.B. "classic") aus RON-Dateien.
/// Erwartet Dateien im Format: `<variant>.board.ron`, `<variant>.pieces.ron`, etc.
pub fn load_variant(variant: &str, dir: &Path) -> Result<GameData, DataError> {
    let board = board::load(&dir.join(format!("{}.board.ron", variant)))?;
    let pieces = piece::load(&dir.join(format!("{}.pieces.ron", variant)))?;
    let game = game::load(&dir.join(format!("{}.game.ron", variant)))?;
    let rules = rule::load(&dir.join(format!("{}.rule.ron", variant)))?;
    Ok(GameData {
        board,
        pieces,
        game,
        rules,
    })
}
